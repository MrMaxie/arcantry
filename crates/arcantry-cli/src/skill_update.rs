use crate::{SkillStatusOptions, SkillUpdateOptions, embedded};
use anyhow::{Context, Result, bail};
use arcantry_core::catalog::{self, SkillPackageManifest};
use directories::ProjectDirs;
use fs4::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

const PLAN_SCHEMA: &str = "skill-update@1";
const GITHUB_REPOSITORY: &str = "MrMaxie/arcantry";
const GITHUB_BRANCH: &str = "master";
const MAX_FILE_BYTES: u64 = 4 * 1024 * 1024;
const MAX_PACKAGE_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct Receipt {
  schema_version: u32,
  name: String,
  source_kind: String,
  source_locator: String,
  version: String,
  revision: String,
  digest: String,
  targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSource {
  kind: String,
  locator: String,
  revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageFile {
  path: String,
  sha256: String,
  content_hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackagePayload {
  version: String,
  role: String,
  digest: String,
  files: Vec<PackageFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePlan {
  schema: String,
  name: String,
  source: UpdateSource,
  package: PackagePayload,
  target_roots: Vec<String>,
  receipt_preimage: Option<Receipt>,
  installed_digest: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct StatusRecord {
  name: String,
  state: String,
  version: Option<String>,
  revision: Option<String>,
  digest: Option<String>,
  source: Option<String>,
  available_version: Option<String>,
  available_revision: Option<String>,
  available_digest: Option<String>,
  update_available: Option<bool>,
}

pub fn status(
  cwd: &Path,
  name: Option<String>,
  options: SkillStatusOptions,
  check: bool,
  json: bool,
) -> Result<i32> {
  validate_status_options(&options)?;
  let target_roots = status_targets(cwd, &options)?;
  let names = if let Some(name) = name {
    vec![name]
  } else {
    embedded::public_catalog()?
      .skills
      .into_iter()
      .map(|entry| entry.name)
      .collect()
  };
  let mut records = Vec::new();
  for name in names {
    let receipt = read_receipt(&receipt_path(&target_roots, &name)?)?;
    let installed = installed_manifest(&target_roots, &name)?;
    let state = match (&receipt, &installed) {
      (Some(receipt), Some(manifest)) if receipt.digest == manifest.digest => "managed",
      (Some(_), Some(_)) => "modified",
      (Some(_), None) => "missing",
      (None, Some(_)) => "unmanaged",
      (None, None) => "not-installed",
    }
    .to_owned();
    let available = if check {
      Some(resolve_package(
        cwd,
        &name,
        options.catalog_root.as_deref(),
      )?)
    } else {
      None
    };
    let update_available = available.as_ref().and_then(|(_, package)| {
      installed
        .as_ref()
        .map(|current| current.digest != package.digest)
    });
    records.push(StatusRecord {
      name,
      state,
      version: installed.as_ref().map(|value| value.version.clone()),
      revision: receipt.as_ref().map(|value| value.revision.clone()),
      digest: installed.as_ref().map(|value| value.digest.clone()),
      source: receipt.as_ref().map(|value| value.source_locator.clone()),
      available_version: available.as_ref().map(|(_, value)| value.version.clone()),
      available_revision: available
        .as_ref()
        .map(|(source, _)| source.revision.clone()),
      available_digest: available.as_ref().map(|(_, value)| value.digest.clone()),
      update_available,
    });
  }
  if json {
    println!("{}", serde_json::to_string_pretty(&records)?);
  } else {
    for record in records {
      println!("{}\t{}", record.name, record.state);
      if let Some(version) = record.version {
        println!(
          "  installed: {version} {}",
          record.digest.unwrap_or_default()
        );
      }
      if let Some(revision) = record.revision {
        println!("  revision: {revision}");
      }
      if let Some(available) = record.available_version {
        println!(
          "  available: {available} {} ({})",
          record.available_digest.unwrap_or_default(),
          record.available_revision.unwrap_or_default()
        );
        println!(
          "  update: {}",
          if record.update_available == Some(true) {
            "available"
          } else {
            "current"
          }
        );
      }
    }
  }
  Ok(0)
}

pub fn preview(
  cwd: &Path,
  name: String,
  options: SkillUpdateOptions,
  output: Option<&Path>,
) -> Result<i32> {
  validate_update_options(&options)?;
  let target_roots = update_targets(cwd, &options)?;
  let receipt_path = receipt_path(&target_roots, &name)?;
  let receipt = read_receipt(&receipt_path)?;
  let installed = installed_manifest(&target_roots, &name)?;
  if installed.is_some() && receipt.is_none() {
    bail!("The installed skill is unmanaged. Link or migrate it explicitly before updating.");
  }
  if let (Some(receipt), Some(installed)) = (&receipt, &installed)
    && receipt.digest != installed.digest
  {
    bail!("The managed skill has local changes; refusing to prepare an overwrite.");
  }
  let (source, package) = resolve_package(cwd, &name, options.catalog_root.as_deref())?;
  let plan = UpdatePlan {
    schema: PLAN_SCHEMA.to_owned(),
    name,
    source,
    package,
    target_roots: target_roots.iter().map(|path| path_string(path)).collect(),
    receipt_preimage: receipt,
    installed_digest: installed.map(|value| value.digest),
  };
  let serialized = format!("{}\n", serde_json::to_string_pretty(&plan)?);
  if let Some(path) = output {
    arcantry_core::project_plan::write_new(path, &serialized)?;
    eprintln!("Saved skill update plan.");
  } else {
    println!("{serialized}");
  }
  Ok(0)
}

pub fn apply(cwd: &Path, plan_path: &str) -> Result<i32> {
  let source = if plan_path == "-" {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;
    source
  } else {
    let path = PathBuf::from(plan_path);
    fs::read_to_string(if path.is_absolute() {
      path
    } else {
      cwd.join(path)
    })?
  };
  let plan: UpdatePlan = serde_json::from_str(&source)?;
  if plan.schema != PLAN_SCHEMA {
    bail!("Unsupported skill update plan schema.");
  }
  validate_name_and_paths(&plan)?;
  let target_roots = plan
    .target_roots
    .iter()
    .map(PathBuf::from)
    .collect::<Vec<_>>();
  let data_root = data_root()?;
  fs::create_dir_all(data_root.join("locks"))?;
  let lock_path = data_root.join("locks").join(format!("{}.lock", plan.name));
  let lock = OpenOptions::new()
    .create(true)
    .truncate(false)
    .read(true)
    .write(true)
    .open(lock_path)?;
  FileExt::lock(&lock)?;
  let result = apply_locked(&data_root, &target_roots, &plan);
  let _ = FileExt::unlock(&lock);
  result?;
  println!(
    "Updated {} to {} at {} ({}).",
    plan.name, plan.package.version, plan.source.revision, plan.package.digest
  );
  Ok(0)
}

fn apply_locked(data_root: &Path, target_roots: &[PathBuf], plan: &UpdatePlan) -> Result<()> {
  apply_locked_with_receipt_writer(data_root, target_roots, plan, write_receipt)
}

fn apply_locked_with_receipt_writer(
  data_root: &Path,
  target_roots: &[PathBuf],
  plan: &UpdatePlan,
  receipt_writer: impl FnOnce(&Path, &Receipt) -> Result<()>,
) -> Result<()> {
  let receipt_path = receipt_path_in(data_root, target_roots, &plan.name);
  let current_receipt = read_receipt(&receipt_path)?;
  if current_receipt != plan.receipt_preimage {
    bail!("The installation receipt changed after the plan was created.");
  }
  let current = installed_manifest(target_roots, &plan.name)?;
  if current.as_ref().map(|value| value.digest.clone()) != plan.installed_digest {
    bail!("The installed skill changed after the plan was created.");
  }
  verify_payload(&plan.name, &plan.package)?;
  let snapshot = materialize_snapshot(data_root, &plan.name, &plan.package)?;
  let results = catalog::link(&snapshot, &plan.name, target_roots, true)?;
  let receipt = Receipt {
    schema_version: 1,
    name: plan.name.clone(),
    source_kind: plan.source.kind.clone(),
    source_locator: plan.source.locator.clone(),
    version: plan.package.version.clone(),
    revision: plan.source.revision.clone(),
    digest: plan.package.digest.clone(),
    targets: plan.target_roots.clone(),
  };
  if let Err(error) = receipt_writer(&receipt_path, &receipt) {
    catalog::rollback_links(&results)
      .context("Receipt write failed and links could not be rolled back")?;
    return Err(error);
  }
  catalog::finalize_links(&results)?;
  Ok(())
}

fn resolve_package(
  cwd: &Path,
  name: &str,
  local_root: Option<&Path>,
) -> Result<(UpdateSource, PackagePayload)> {
  if let Some(root) = local_root {
    let root = if root.is_absolute() {
      root.to_path_buf()
    } else {
      cwd.join(root)
    };
    let package = package_from_directory(&root.join("skills").join(name), name)?;
    let revision = git_revision(&root)
      .map(|commit| format!("{commit}+{}", package.digest))
      .unwrap_or_else(|| format!("local+{}", package.digest));
    return Ok((
      UpdateSource {
        kind: "local".to_owned(),
        locator: path_string(&root),
        revision,
      },
      package,
    ));
  }
  package_from_github(name)
}

fn package_from_github(name: &str) -> Result<(UpdateSource, PackagePayload)> {
  validate_simple_name(name)?;
  let agent: ureq::Agent = ureq::Agent::config_builder()
    .timeout_global(Some(Duration::from_secs(20)))
    .build()
    .into();
  let commit: serde_json::Value = get_json(
    &agent,
    &format!("https://api.github.com/repos/{GITHUB_REPOSITORY}/commits/{GITHUB_BRANCH}"),
  )?;
  let revision = commit["sha"]
    .as_str()
    .context("GitHub commit response omitted sha")?
    .to_owned();
  let tree: serde_json::Value = get_json(
    &agent,
    &format!("https://api.github.com/repos/{GITHUB_REPOSITORY}/git/trees/{revision}?recursive=1"),
  )?;
  if tree["truncated"] == true {
    bail!("GitHub returned a truncated repository tree.");
  }
  let prefix = format!("skills/{name}/");
  let mut paths = tree["tree"]
    .as_array()
    .context("GitHub tree response omitted tree")?
    .iter()
    .filter(|entry| entry["type"] == "blob")
    .filter_map(|entry| entry["path"].as_str())
    .filter_map(|path| path.strip_prefix(&prefix).map(str::to_owned))
    .collect::<Vec<_>>();
  paths.sort();
  if paths.is_empty() {
    bail!("Skill is not present on the official {GITHUB_BRANCH} branch: {name}");
  }
  let temporary = tempfile::tempdir()?;
  let directory = temporary.path().join(name);
  let mut total = 0_u64;
  for relative in paths {
    validate_relative(&relative)?;
    let url = format!(
      "https://raw.githubusercontent.com/{GITHUB_REPOSITORY}/{revision}/{prefix}{relative}"
    );
    let bytes = get_bytes(&agent, &url)?;
    total += bytes.len() as u64;
    if bytes.len() as u64 > MAX_FILE_BYTES || total > MAX_PACKAGE_BYTES {
      bail!("Remote skill package exceeds the allowed size.");
    }
    let target = directory.join(&relative);
    fs::create_dir_all(target.parent().context("Package file has no parent")?)?;
    fs::write(target, bytes)?;
  }
  let package = package_from_directory(&directory, name)?;
  Ok((
    UpdateSource {
      kind: "github".to_owned(),
      locator: format!("https://github.com/{GITHUB_REPOSITORY}/tree/{GITHUB_BRANCH}"),
      revision,
    },
    package,
  ))
}

fn get_json(agent: &ureq::Agent, url: &str) -> Result<serde_json::Value> {
  Ok(
    agent
      .get(url)
      .header("User-Agent", "arcantry-cli/1.0.0")
      .header("Accept", "application/vnd.github+json")
      .call()?
      .body_mut()
      .read_json()?,
  )
}

fn get_bytes(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>> {
  let mut response = agent
    .get(url)
    .header("User-Agent", "arcantry-cli/1.0.0")
    .call()?;
  let bytes = response
    .body_mut()
    .with_config()
    .limit(MAX_FILE_BYTES + 1)
    .read_to_vec()?;
  Ok(bytes)
}

fn package_from_directory(directory: &Path, name: &str) -> Result<PackagePayload> {
  let schema_root = embedded::materialize_catalog()?.join("schemas");
  let manifest = catalog::validate_package_directory(directory, name, &schema_root)?;
  let mut files = Vec::new();
  for file in &manifest.files {
    let bytes = fs::read(directory.join(&file.path))?;
    files.push(PackageFile {
      path: file.path.clone(),
      sha256: file.sha256.clone(),
      content_hex: hex_encode(&bytes),
    });
  }
  Ok(PackagePayload {
    version: manifest.version,
    role: manifest.role,
    digest: manifest.digest,
    files,
  })
}

fn verify_payload(name: &str, package: &PackagePayload) -> Result<()> {
  let temporary = tempfile::tempdir()?;
  write_payload(temporary.path(), package)?;
  let schema_root = embedded::materialize_catalog()?.join("schemas");
  let manifest = catalog::validate_package_directory(temporary.path(), name, &schema_root)?;
  if manifest.version != package.version
    || manifest.role != package.role
    || manifest.digest != package.digest
  {
    bail!("Skill update payload identity does not match its content.");
  }
  Ok(())
}

fn materialize_snapshot(data_root: &Path, name: &str, package: &PackagePayload) -> Result<PathBuf> {
  let parent = data_root.join("snapshots").join(name);
  let target = parent.join(&package.digest);
  fs::create_dir_all(&parent)?;
  if target.exists() {
    let schema_root = embedded::materialize_catalog()?.join("schemas");
    let manifest = catalog::validate_package_directory(&target, name, &schema_root)?;
    if manifest.digest == package.digest {
      return Ok(target);
    }
    bail!("An immutable skill snapshot exists with unexpected content.");
  }
  let temporary = tempfile::Builder::new()
    .prefix(".snapshot-")
    .tempdir_in(&parent)?;
  write_payload(temporary.path(), package)?;
  let schema_root = embedded::materialize_catalog()?.join("schemas");
  let manifest = catalog::validate_package_directory(temporary.path(), name, &schema_root)?;
  if manifest.digest != package.digest {
    bail!("Materialized skill snapshot digest mismatch.");
  }
  let persisted = temporary.keep();
  fs::rename(persisted, &target)?;
  Ok(target)
}

fn write_payload(directory: &Path, package: &PackagePayload) -> Result<()> {
  for file in &package.files {
    validate_relative(&file.path)?;
    let bytes = hex_decode(&file.content_hex)?;
    if hex_digest(&bytes) != file.sha256 {
      bail!("Skill update file digest mismatch: {}", file.path);
    }
    let target = directory.join(&file.path);
    fs::create_dir_all(target.parent().context("Package file has no parent")?)?;
    fs::write(target, bytes)?;
  }
  Ok(())
}

fn installed_manifest(
  target_roots: &[PathBuf],
  name: &str,
) -> Result<Option<SkillPackageManifest>> {
  let mut found = None;
  let mut missing = 0_usize;
  let schema_root = embedded::materialize_catalog()?.join("schemas");
  for root in target_roots {
    let target = root.join(name);
    if !target.exists() {
      missing += 1;
      continue;
    }
    let canonical = dunce::canonicalize(&target)?;
    let manifest = catalog::validate_package_directory(&canonical, name, &schema_root)?;
    if found
      .as_ref()
      .is_some_and(|existing: &SkillPackageManifest| existing.digest != manifest.digest)
    {
      bail!("Managed skill targets do not contain the same package revision.");
    }
    found = Some(manifest);
  }
  if found.is_some() && missing > 0 {
    bail!("Some managed skill targets are missing.");
  }
  Ok(found)
}

fn validate_status_options(options: &SkillStatusOptions) -> Result<()> {
  if options.target.is_some() && options.scope.is_some() {
    bail!("--target cannot be combined with --scope.");
  }
  if options
    .scope
    .as_deref()
    .is_some_and(|scope| !matches!(scope, "user" | "repo"))
  {
    bail!("--scope must be user or repo.");
  }
  Ok(())
}

fn validate_update_options(options: &SkillUpdateOptions) -> Result<()> {
  if options.target.is_some() && (options.scope.is_some() || options.compat.is_some()) {
    bail!("--target cannot be combined with --scope or --compat.");
  }
  if options
    .scope
    .as_deref()
    .is_some_and(|scope| !matches!(scope, "user" | "repo"))
  {
    bail!("--scope must be user or repo.");
  }
  if options
    .compat
    .as_deref()
    .is_some_and(|value| value != "claude")
  {
    bail!("Invalid compatibility: only claude is supported.");
  }
  Ok(())
}

fn status_targets(cwd: &Path, options: &SkillStatusOptions) -> Result<Vec<PathBuf>> {
  if let Some(target) = &options.target {
    return Ok(vec![absolute(cwd, target)]);
  }
  Ok(vec![if options.scope.as_deref() == Some("repo") {
    catalog::repo_target(&arcantry_core::repository::resolve_repository_root(cwd)?)
  } else {
    catalog::user_target()?
  }])
}

fn update_targets(cwd: &Path, options: &SkillUpdateOptions) -> Result<Vec<PathBuf>> {
  if let Some(target) = &options.target {
    return Ok(vec![absolute(cwd, target)]);
  }
  let mut targets = if options.scope.as_deref() == Some("repo") {
    let root = arcantry_core::repository::resolve_repository_root(cwd)?;
    vec![catalog::repo_target(&root)]
  } else {
    vec![catalog::user_target()?]
  };
  if options.compat.as_deref() == Some("claude") {
    targets.push(if options.scope.as_deref() == Some("repo") {
      catalog::repo_claude_target(&arcantry_core::repository::resolve_repository_root(cwd)?)
    } else {
      catalog::user_claude_target()?
    });
  }
  Ok(targets)
}

fn validate_name_and_paths(plan: &UpdatePlan) -> Result<()> {
  validate_simple_name(&plan.name)?;
  if plan.target_roots.is_empty()
    || plan
      .target_roots
      .iter()
      .any(|path| !Path::new(path).is_absolute())
  {
    bail!("Skill update plan requires absolute target roots.");
  }
  Ok(())
}

fn validate_simple_name(name: &str) -> Result<()> {
  if name.is_empty()
    || name.len() > 63
    || name.split('-').any(|part| {
      part.is_empty()
        || !part
          .chars()
          .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    })
  {
    bail!("Invalid skill name: {name}");
  }
  Ok(())
}

fn validate_relative(path: &str) -> Result<()> {
  let value = Path::new(path);
  if value.is_absolute()
    || path.contains('\\')
    || path
      .split('/')
      .any(|part| part.is_empty() || part == "." || part == "..")
  {
    bail!("Invalid skill package path: {path}");
  }
  Ok(())
}

fn data_root() -> Result<PathBuf> {
  Ok(
    ProjectDirs::from("dev", "MrMaxie", "Arcantry")
      .context("Could not resolve the Arcantry data directory")?
      .data_dir()
      .join("skills"),
  )
}

fn receipt_path(target_roots: &[PathBuf], name: &str) -> Result<PathBuf> {
  Ok(receipt_path_in(&data_root()?, target_roots, name))
}

fn receipt_path_in(data_root: &Path, target_roots: &[PathBuf], name: &str) -> PathBuf {
  let key = target_roots
    .iter()
    .map(|path| path_string(path))
    .collect::<Vec<_>>()
    .join("\n");
  data_root
    .join("receipts")
    .join(hex_digest(key.as_bytes()))
    .join(format!("{name}.json"))
}

fn read_receipt(path: &Path) -> Result<Option<Receipt>> {
  match fs::read_to_string(path) {
    Ok(value) => Ok(Some(serde_json::from_str(&value)?)),
    Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
    Err(error) => Err(error.into()),
  }
}

fn write_receipt(path: &Path, receipt: &Receipt) -> Result<()> {
  fs::create_dir_all(path.parent().context("Receipt has no parent")?)?;
  let temporary = path.with_extension("json.new");
  if temporary.exists() {
    bail!(
      "Interrupted receipt write requires inspection: {}",
      temporary.display()
    );
  }
  fs::write(
    &temporary,
    format!("{}\n", serde_json::to_string_pretty(receipt)?),
  )?;
  if path.exists() {
    let backup = path.with_extension("json.previous");
    if backup.exists() {
      bail!(
        "Interrupted receipt replacement requires inspection: {}",
        backup.display()
      );
    }
    fs::rename(path, &backup)?;
    if let Err(error) = fs::rename(&temporary, path) {
      fs::rename(&backup, path).ok();
      return Err(error.into());
    }
    fs::remove_file(backup)?;
  } else {
    fs::rename(temporary, path)?;
  }
  Ok(())
}

fn git_revision(root: &Path) -> Option<String> {
  let output = Command::new("git")
    .args(["rev-parse", "HEAD"])
    .current_dir(root)
    .output()
    .ok()?;
  output
    .status
    .success()
    .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn absolute(cwd: &Path, path: &Path) -> PathBuf {
  if path.is_absolute() {
    path.to_path_buf()
  } else {
    cwd.join(path)
  }
}

fn path_string(path: &Path) -> String {
  path.to_string_lossy().replace('\\', "/")
}

fn hex_digest(bytes: &[u8]) -> String {
  Sha256::digest(bytes)
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect()
}

fn hex_encode(bytes: &[u8]) -> String {
  bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn hex_decode(value: &str) -> Result<Vec<u8>> {
  if !value.len().is_multiple_of(2) || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
    bail!("Invalid hex-encoded package content.");
  }
  (0..value.len())
    .step_by(2)
    .map(|index| u8::from_str_radix(&value[index..index + 2], 16).map_err(Into::into))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use walkdir::WalkDir;

  fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
  }

  fn local_plan(name: &str, package: PackagePayload, target: &Path) -> UpdatePlan {
    UpdatePlan {
      schema: PLAN_SCHEMA.to_owned(),
      name: name.to_owned(),
      source: UpdateSource {
        kind: "local".to_owned(),
        locator: "fixture".to_owned(),
        revision: package.digest.clone(),
      },
      package,
      target_roots: vec![path_string(target)],
      receipt_preimage: None,
      installed_digest: None,
    }
  }

  fn changed_package(temporary: &Path, name: &str) -> PackagePayload {
    let source = repository_root().join("skills").join(name);
    let destination = temporary.join(name);
    for entry in WalkDir::new(&source) {
      let entry = entry.unwrap();
      let relative = entry.path().strip_prefix(&source).unwrap();
      let target = destination.join(relative);
      if entry.file_type().is_dir() {
        fs::create_dir_all(&target).unwrap();
      } else {
        fs::copy(entry.path(), &target).unwrap();
      }
    }
    let skill = destination.join("SKILL.md");
    let mut contents = fs::read_to_string(&skill).unwrap();
    contents.push_str("\n<!-- same-version update fixture -->\n");
    fs::write(skill, contents).unwrap();
    package_from_directory(&destination, name).unwrap()
  }

  #[test]
  fn applies_one_skill_snapshot_and_rejects_a_stale_plan() {
    let temporary = tempfile::tempdir().unwrap();
    let data = temporary.path().join("data");
    let target = temporary.path().join("agents");
    let package = package_from_directory(
      &repository_root().join("skills/assess-code-quality"),
      "assess-code-quality",
    )
    .unwrap();
    let plan = local_plan("assess-code-quality", package, &target);

    apply_locked(&data, std::slice::from_ref(&target), &plan).unwrap();

    let linked = dunce::canonicalize(target.join("assess-code-quality")).unwrap();
    let snapshot_root = dunce::canonicalize(data.join("snapshots/assess-code-quality")).unwrap();
    assert!(linked.starts_with(snapshot_root));
    let error = apply_locked(&data, std::slice::from_ref(&target), &plan).unwrap_err();
    assert!(error.to_string().contains("receipt changed"));
  }

  #[test]
  fn updating_one_skill_does_not_change_another_target() {
    let temporary = tempfile::tempdir().unwrap();
    let data = temporary.path().join("data");
    let target = temporary.path().join("agents");
    for name in ["assess-code-quality", "design-maintainable-code"] {
      let package =
        package_from_directory(&repository_root().join("skills").join(name), name).unwrap();
      let plan = local_plan(name, package, &target);
      apply_locked(&data, std::slice::from_ref(&target), &plan).unwrap();
    }
    let other_before = dunce::canonicalize(target.join("design-maintainable-code")).unwrap();
    let current_receipt = read_receipt(&receipt_path_in(
      &data,
      std::slice::from_ref(&target),
      "assess-code-quality",
    ))
    .unwrap();
    let current = installed_manifest(std::slice::from_ref(&target), "assess-code-quality").unwrap();
    let old = current.as_ref().unwrap().clone();
    let updated = changed_package(temporary.path(), "assess-code-quality");
    assert_eq!(old.version, updated.version);
    assert_ne!(old.digest, updated.digest);
    let mut plan = local_plan("assess-code-quality", updated.clone(), &target);
    plan.receipt_preimage = current_receipt;
    plan.installed_digest = current.map(|value| value.digest);

    apply_locked(&data, std::slice::from_ref(&target), &plan).unwrap();

    assert_eq!(
      other_before,
      dunce::canonicalize(target.join("design-maintainable-code")).unwrap()
    );
    assert_eq!(
      installed_manifest(std::slice::from_ref(&target), "assess-code-quality")
        .unwrap()
        .unwrap()
        .digest,
      updated.digest
    );
  }

  #[test]
  fn receipt_failure_restores_previous_link_and_receipt() {
    let temporary = tempfile::tempdir().unwrap();
    let data = temporary.path().join("data");
    let target = temporary.path().join("agents");
    let name = "assess-code-quality";
    let first = local_plan(
      name,
      package_from_directory(&repository_root().join("skills").join(name), name).unwrap(),
      &target,
    );
    apply_locked(&data, std::slice::from_ref(&target), &first).unwrap();
    let old_link = dunce::canonicalize(target.join(name)).unwrap();
    let receipt_path = receipt_path_in(&data, std::slice::from_ref(&target), name);
    let old_receipt = read_receipt(&receipt_path).unwrap();
    let current = installed_manifest(std::slice::from_ref(&target), name).unwrap();
    let mut update = local_plan(name, changed_package(temporary.path(), name), &target);
    update.receipt_preimage = old_receipt.clone();
    update.installed_digest = current.map(|value| value.digest);

    let error =
      apply_locked_with_receipt_writer(&data, std::slice::from_ref(&target), &update, |_, _| {
        bail!("injected receipt failure")
      })
      .unwrap_err();

    assert!(error.to_string().contains("injected receipt failure"));
    assert_eq!(old_link, dunce::canonicalize(target.join(name)).unwrap());
    assert_eq!(old_receipt, read_receipt(&receipt_path).unwrap());
  }

  #[test]
  fn apply_rejects_local_changes_after_plan_creation() {
    let temporary = tempfile::tempdir().unwrap();
    let data = temporary.path().join("data");
    let target = temporary.path().join("agents");
    let name = "assess-code-quality";
    let first = local_plan(
      name,
      package_from_directory(&repository_root().join("skills").join(name), name).unwrap(),
      &target,
    );
    apply_locked(&data, std::slice::from_ref(&target), &first).unwrap();
    let receipt =
      read_receipt(&receipt_path_in(&data, std::slice::from_ref(&target), name)).unwrap();
    let installed = installed_manifest(std::slice::from_ref(&target), name).unwrap();
    let mut update = local_plan(name, changed_package(temporary.path(), name), &target);
    update.receipt_preimage = receipt;
    update.installed_digest = installed.map(|value| value.digest);
    let linked_skill = dunce::canonicalize(target.join(name))
      .unwrap()
      .join("SKILL.md");
    let mut contents = fs::read_to_string(&linked_skill).unwrap();
    contents.push_str("\n<!-- local modification -->\n");
    fs::write(linked_skill, contents).unwrap();

    let error = apply_locked(&data, std::slice::from_ref(&target), &update).unwrap_err();

    assert!(error.to_string().contains("installed skill changed"));
  }

  #[test]
  fn installed_manifest_rejects_a_missing_compatibility_alias() {
    let temporary = tempfile::tempdir().unwrap();
    let data = temporary.path().join("data");
    let primary = temporary.path().join("agents");
    let compatibility = temporary.path().join("claude");
    let targets = vec![primary, compatibility.clone()];
    let name = "assess-code-quality";
    let package =
      package_from_directory(&repository_root().join("skills").join(name), name).unwrap();
    let plan = UpdatePlan {
      target_roots: targets.iter().map(|path| path_string(path)).collect(),
      ..local_plan(name, package, &targets[0])
    };
    apply_locked(&data, &targets, &plan).unwrap();
    let source = dunce::canonicalize(targets[0].join(name)).unwrap();
    catalog::unlink(&source, name, &[compatibility]).unwrap();

    let error = installed_manifest(&targets, name).unwrap_err();

    assert!(error.to_string().contains("targets are missing"));
  }

  #[test]
  fn payload_rejects_parent_traversal() {
    let mut package = package_from_directory(
      &repository_root().join("skills/refactor-safely"),
      "refactor-safely",
    )
    .unwrap();
    package.files[0].path = "../escape".to_owned();
    assert!(verify_payload("refactor-safely", &package).is_err());
  }

  #[test]
  fn payload_rejects_an_incomplete_package() {
    let mut package = package_from_directory(
      &repository_root().join("skills/refactor-safely"),
      "refactor-safely",
    )
    .unwrap();
    package.files.retain(|file| file.path != "SKILL.md");

    assert!(verify_payload("refactor-safely", &package).is_err());
  }

  #[test]
  fn transport_reports_an_offline_source() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    drop(listener);
    let agent: ureq::Agent = ureq::Agent::config_builder()
      .timeout_global(Some(Duration::from_millis(250)))
      .build()
      .into();

    assert!(get_json(&agent, &format!("http://{address}/offline")).is_err());
  }

  #[test]
  fn explicit_target_cannot_be_combined_with_explicit_scope() {
    let target_only = SkillStatusOptions {
      catalog_root: None,
      scope: None,
      target: Some(PathBuf::from("custom")),
    };
    assert!(validate_status_options(&target_only).is_ok());
    let target_and_scope = SkillStatusOptions {
      scope: Some("user".to_owned()),
      ..target_only
    };
    assert!(validate_status_options(&target_and_scope).is_err());
  }
}
