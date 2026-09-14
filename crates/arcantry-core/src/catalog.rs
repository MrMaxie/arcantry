use anyhow::{Context, Result, bail};
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
  pub name: String,
  pub family: String,
  pub tags: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub skills: Vec<CatalogEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillScenario {
  pub title: String,
  pub prompt: String,
  pub outcome: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub version: String,
  pub role: String,
  pub summary: String,
  pub scenarios: Vec<SkillScenario>,
  #[serde(default)]
  pub compatibility: Option<SkillCompatibility>,
  #[serde(default)]
  pub learning: Option<SkillLearning>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillCompatibility {
  pub source_kinds: Vec<String>,
  #[serde(default)]
  pub adapters: Option<Vec<SkillAdapter>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAdapter {
  pub name: String,
  pub versions: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillLearning {
  #[serde(default)]
  pub prerequisites: Option<Vec<String>>,
  pub outcomes: Vec<String>,
}
#[derive(Debug, Clone, Deserialize)]
struct SkillAgentDocument {
  interface: SkillAgent,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAgent {
  pub display_name: String,
  pub short_description: String,
  pub default_prompt: String,
}
#[derive(Debug, Clone)]
pub struct SkillInspection {
  pub entry: CatalogEntry,
  pub metadata: SkillMetadata,
  pub directory: PathBuf,
}
#[derive(Debug, Clone)]
pub struct PrivateSkillInspection {
  pub name: String,
  pub description: String,
  pub directory: PathBuf,
}
#[derive(Debug, Clone)]
pub struct LinkResult {
  pub status: &'static str,
  pub source: PathBuf,
  pub target: PathBuf,
  pub backup: Option<PathBuf>,
  created_directories: Vec<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillPackageFile {
  pub path: String,
  pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkillPackageManifest {
  pub name: String,
  pub version: String,
  pub role: String,
  pub digest: String,
  pub files: Vec<SkillPackageFile>,
}

pub fn load(root: &Path) -> Result<Catalog> {
  let value: serde_json::Value =
    serde_json::from_str(&fs::read_to_string(root.join("catalog.json"))?)?;
  validate_json_document(
    &root.join("schemas").join("catalog.schema.json"),
    &value,
    "catalog.json",
  )?;
  Ok(serde_json::from_value(value)?)
}

pub fn inspect(root: &Path, name: &str) -> Result<SkillInspection> {
  let catalog = load(root)?;
  let entry = catalog
    .skills
    .into_iter()
    .find(|entry| entry.name == name)
    .with_context(|| format!("Skill is not present in catalog.json: {name}"))?;
  let directory = root.join("skills").join(name);
  let metadata = load_skill_metadata(root, name)?;
  Ok(SkillInspection {
    entry,
    metadata,
    directory,
  })
}

pub fn package_manifest(root: &Path, name: &str) -> Result<SkillPackageManifest> {
  let inspection = inspect(root, name)?;
  package_manifest_for_directory(&inspection.directory, name, &inspection.metadata)
}

pub fn package_manifest_for_directory(
  directory: &Path,
  name: &str,
  metadata: &SkillMetadata,
) -> Result<SkillPackageManifest> {
  validate_name(name)?;
  let mut files = Vec::new();
  for entry in walkdir::WalkDir::new(directory).follow_links(false) {
    let entry = entry?;
    if entry.file_type().is_symlink() {
      bail!(
        "Skill packages cannot contain symbolic links: {}",
        entry.path().display()
      );
    }
    if !entry.file_type().is_file() {
      continue;
    }
    let path = entry
      .path()
      .strip_prefix(directory)?
      .to_string_lossy()
      .replace('\\', "/");
    if path.is_empty()
      || path
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
      bail!("Skill package contains an invalid relative path: {path}");
    }
    files.push(SkillPackageFile {
      path,
      sha256: hex_digest(&fs::read(entry.path())?),
    });
  }
  files.sort_by(|left, right| left.path.cmp(&right.path));
  let mut digest_input = Vec::new();
  for file in &files {
    digest_input.extend_from_slice(file.path.as_bytes());
    digest_input.push(0);
    digest_input.extend_from_slice(file.sha256.as_bytes());
    digest_input.push(b'\n');
  }
  Ok(SkillPackageManifest {
    name: name.to_owned(),
    version: metadata.version.clone(),
    role: metadata.role.clone(),
    digest: hex_digest(&digest_input),
    files,
  })
}

pub fn validate_package_directory(
  directory: &Path,
  name: &str,
  schema_directory: &Path,
) -> Result<SkillPackageManifest> {
  validate_source(directory, name)?;
  let metadata_path = directory.join("arcantry.json");
  let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(&metadata_path)?)?;
  validate_json_document(
    &schema_directory.join("skill-metadata.schema.json"),
    &value,
    &format!("skills/{name}/arcantry.json"),
  )?;
  let metadata: SkillMetadata = serde_json::from_value(value)?;
  let agent = load_skill_agent_file(&directory.join("agents/openai.yaml"))?;
  if !agent.default_prompt.contains(&format!("${name}")) {
    bail!("skills/{name}/agents/openai.yaml must mention ${name}.");
  }
  let mut errors = Vec::new();
  validate_markdown_links(directory, directory, &mut errors);
  if !errors.is_empty() {
    bail!("{}", errors.join("; "));
  }
  package_manifest_for_directory(directory, name, &metadata)
}

fn hex_digest(bytes: &[u8]) -> String {
  use sha2::{Digest, Sha256};
  Sha256::digest(bytes)
    .iter()
    .map(|byte| format!("{byte:02x}"))
    .collect()
}

pub fn inspect_private(root: &Path, name: &str) -> Result<PrivateSkillInspection> {
  validate_name(name)?;
  let directory = root.join(".local").join("skills").join(name);
  let skill_file = directory.join("SKILL.md");
  if !directory.is_dir() || !skill_file.is_file() {
    bail!("Private skill is missing: .local/skills/{name}/SKILL.md");
  }
  let (frontmatter_name, description) = read_frontmatter(&fs::read_to_string(skill_file)?)?;
  if frontmatter_name != name {
    bail!("Private skill frontmatter name must match .local/skills/{name}.");
  }
  Ok(PrivateSkillInspection {
    name: name.to_owned(),
    description,
    directory,
  })
}

pub fn list_private(root: &Path) -> Result<Vec<PrivateSkillInspection>> {
  let directory = root.join(".local").join("skills");
  if !directory.exists() {
    return Ok(Vec::new());
  }
  if !directory.is_dir() {
    bail!(".local/skills must be a directory.");
  }
  let mut names: Vec<_> = fs::read_dir(directory)?
    .filter_map(|entry| entry.ok())
    .filter(|entry| {
      entry
        .file_type()
        .is_ok_and(|kind| kind.is_dir() || kind.is_symlink())
    })
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .collect();
  names.sort();
  names
    .iter()
    .map(|name| inspect_private(root, name))
    .collect()
}

pub fn find_root(start: &Path) -> Result<PathBuf> {
  for directory in start.ancestors() {
    if directory.join("catalog.json").is_file() && directory.join("skills").is_dir() {
      return Ok(directory.to_path_buf());
    }
  }
  bail!(
    "No Arcantry catalog found from {}. Use --catalog-root.",
    start.display()
  )
}

pub fn validate(root: &Path) -> (bool, Vec<String>, Option<Catalog>) {
  let catalog = match load(root) {
    Ok(catalog) => catalog,
    Err(error) => return (false, vec![error.to_string()], None),
  };
  let mut errors = Vec::new();
  let mut descriptions = BTreeMap::new();
  let mut summaries = BTreeMap::new();
  let names: Vec<_> = catalog
    .skills
    .iter()
    .map(|entry| entry.name.clone())
    .collect();
  let mut sorted = names.clone();
  sorted.sort();
  if names != sorted {
    errors.push("catalog.json skills must be sorted by name.".to_owned());
  }
  if names.iter().collect::<BTreeSet<_>>().len() != names.len() {
    errors.push("catalog.json skill names must be unique.".to_owned());
  }
  let skill_root = root.join("skills");
  match fs::read_dir(&skill_root) {
    Ok(entries) => {
      let mut directories: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
      directories.sort();
      if directories != sorted {
        errors.push("catalog.json membership must exactly match skills/ directories.".to_owned());
      }
    }
    Err(error) => errors.push(format!("skills/: {error}")),
  }
  for entry in &catalog.skills {
    if validate_name(&entry.name).is_err() {
      errors.push(format!("skills/{} has an invalid name.", entry.name));
    }
    if entry.tags.is_empty() || entry.tags.iter().collect::<BTreeSet<_>>().len() != entry.tags.len()
    {
      errors.push(format!(
        "skills/{} tags must be non-empty and unique.",
        entry.name
      ));
    }
    let directory = root.join("skills").join(&entry.name);
    for required in ["SKILL.md", "arcantry.json", "agents/openai.yaml"] {
      if !directory.join(required).is_file() {
        errors.push(format!("skills/{} is missing {required}.", entry.name));
      }
    }
    match fs::read_to_string(directory.join("SKILL.md"))
      .and_then(|source| read_frontmatter(&source).map_err(std::io::Error::other))
    {
      Ok((name, description)) => {
        if name != entry.name {
          errors.push(format!(
            "skills/{} frontmatter name must match its directory.",
            entry.name
          ));
        }
        if description.len() < 30 {
          errors.push(format!("skills/{} description is too short.", entry.name));
        }
        let normalized = normalize_text(&description);
        if let Some(existing) = descriptions.insert(normalized, entry.name.clone()) {
          errors.push(format!(
            "skills/{} description duplicates skills/{existing}.",
            entry.name
          ));
        }
      }
      Err(error) => errors.push(format!("skills/{}/SKILL.md: {error}", entry.name)),
    }
    match load_skill_metadata(root, &entry.name) {
      Ok(metadata) => {
        let normalized = normalize_text(&metadata.summary);
        if let Some(existing) = summaries.insert(normalized, entry.name.clone()) {
          errors.push(format!(
            "skills/{} summary duplicates skills/{existing}.",
            entry.name
          ));
        }
      }
      Err(error) => errors.push(error.to_string()),
    }
    match load_skill_agent_file(&directory.join("agents").join("openai.yaml")) {
      Ok(agent) => {
        if agent.display_name.trim().len() < 3 {
          errors.push(format!("skills/{} display_name is too short.", entry.name));
        }
        if agent.short_description.trim().len() < 15 || agent.short_description.len() > 80 {
          errors.push(format!(
            "skills/{} short_description must contain 15-80 characters.",
            entry.name
          ));
        }
        if !agent.default_prompt.contains(&format!("${}", entry.name)) {
          errors.push(format!(
            "skills/{}/agents/openai.yaml must mention ${}.",
            entry.name, entry.name
          ));
        }
      }
      Err(error) => errors.push(format!("skills/{}/agents/openai.yaml: {error}", entry.name)),
    }
    validate_markdown_links(root, &directory, &mut errors);
  }
  (errors.is_empty(), errors, Some(catalog))
}

fn normalize_text(value: &str) -> String {
  value
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
    .to_lowercase()
}

fn load_skill_agent_file(path: &Path) -> Result<SkillAgent> {
  let document: SkillAgentDocument = serde_saphyr::from_str(&fs::read_to_string(path)?)?;
  Ok(document.interface)
}

pub fn load_skill_agent(root: &Path, name: &str) -> Result<SkillAgent> {
  load_skill_agent_file(&root.join("skills").join(name).join("agents/openai.yaml"))
}

pub fn load_skill_frontmatter(root: &Path, name: &str) -> Result<(String, String)> {
  read_frontmatter(&fs::read_to_string(
    root.join("skills").join(name).join("SKILL.md"),
  )?)
}

fn validate_markdown_links(root: &Path, directory: &Path, errors: &mut Vec<String>) {
  for entry in walkdir::WalkDir::new(directory) {
    let entry = match entry {
      Ok(entry) => entry,
      Err(error) => {
        errors.push(error.to_string());
        continue;
      }
    };
    let path = entry.path();
    if !entry.file_type().is_file()
      || path.extension().and_then(|value| value.to_str()) != Some("md")
    {
      continue;
    }
    let source = match fs::read_to_string(path) {
      Ok(source) => source,
      Err(error) => {
        errors.push(format!("{}: {error}", project_path(root, path)));
        continue;
      }
    };
    for target in markdown_link_targets(&source) {
      let relative = target.split('#').next().unwrap_or_default();
      if relative.is_empty()
        || relative.starts_with("http:")
        || relative.starts_with("https:")
        || relative.starts_with("mailto:")
      {
        continue;
      }
      if !path.parent().unwrap_or(directory).join(relative).exists() {
        errors.push(format!(
          "{} references missing {relative}.",
          project_path(root, path)
        ));
      }
    }
  }
}

fn markdown_link_targets(source: &str) -> Vec<&str> {
  let mut targets = Vec::new();
  let mut remaining = source;
  while let Some(start) = remaining.find("](") {
    let target = &remaining[start + 2..];
    let Some(end) = target.find(')') else {
      break;
    };
    targets.push(&target[..end]);
    remaining = &target[end + 1..];
  }
  targets
}

fn project_path(root: &Path, path: &Path) -> String {
  path
    .strip_prefix(root)
    .unwrap_or(path)
    .to_string_lossy()
    .replace('\\', "/")
}

pub fn load_skill_metadata(root: &Path, name: &str) -> Result<SkillMetadata> {
  let path = root.join("skills").join(name).join("arcantry.json");
  let value: serde_json::Value = serde_json::from_str(&fs::read_to_string(&path)?)?;
  validate_json_document(
    &root.join("schemas").join("skill-metadata.schema.json"),
    &value,
    &format!("skills/{name}/arcantry.json"),
  )?;
  Ok(serde_json::from_value(value)?)
}

fn validate_json_document(
  schema_path: &Path,
  value: &serde_json::Value,
  label: &str,
) -> Result<()> {
  let schema: serde_json::Value = serde_json::from_str(&fs::read_to_string(schema_path)?)?;
  let validator = jsonschema::validator_for(&schema)?;
  let errors: Vec<_> = validator
    .iter_errors(value)
    .map(|error| error.to_string())
    .collect();
  if !errors.is_empty() {
    bail!("{label}: {}", errors.join("; "));
  }
  Ok(())
}

pub fn link(
  source: &Path,
  name: &str,
  target_roots: &[PathBuf],
  replace: bool,
) -> Result<Vec<LinkResult>> {
  link_with_checkpoint(source, name, target_roots, replace, |_, _| Ok(()))
}

fn link_with_checkpoint(
  source: &Path,
  name: &str,
  target_roots: &[PathBuf],
  replace: bool,
  mut checkpoint: impl FnMut(&Path, usize) -> Result<()>,
) -> Result<Vec<LinkResult>> {
  validate_source(source, name)?;
  let source = dunce::canonicalize(source)?;
  let roots = unique_paths(target_roots);
  for root in &roots {
    preflight_link(&source, name, root, replace)?;
  }
  let mut results = Vec::new();
  for (index, root) in roots.into_iter().enumerate() {
    let target = root.join(name);
    let result = (|| -> Result<LinkResult> {
      if target.exists() || fs::symlink_metadata(&target).is_ok() {
        if points_to(&target, &source) {
          return Ok(LinkResult {
            status: "unchanged",
            source: source.clone(),
            target: target.clone(),
            backup: None,
            created_directories: Vec::new(),
          });
        }
        let backup = next_backup(&target);
        fs::rename(&target, &backup)?;
        if let Err(error) = create_directory_link(&source, &target) {
          fs::rename(&backup, &target).ok();
          return Err(error);
        }
        Ok(LinkResult {
          status: "linked",
          source: source.clone(),
          target: target.clone(),
          backup: Some(backup),
          created_directories: Vec::new(),
        })
      } else {
        let mut created_directories = missing_directories(
          target
            .parent()
            .context("Skill link target has no parent directory.")?,
        );
        if let Err(error) = create_directory_link(&source, &target) {
          retain_created_directories(&mut created_directories);
          remove_created_directories(&created_directories)?;
          return Err(error);
        }
        retain_created_directories(&mut created_directories);
        Ok(LinkResult {
          status: "linked",
          source: source.clone(),
          target: target.clone(),
          backup: None,
          created_directories,
        })
      }
    })();
    match result {
      Ok(result) => {
        results.push(result);
        if let Err(error) = checkpoint(&target, index) {
          rollback_links(&results).with_context(|| {
            format!("Linking failed and created skill links could not be rolled back: {error}")
          })?;
          return Err(error);
        }
      }
      Err(error) => {
        rollback_links(&results).with_context(|| {
          format!("Linking failed and earlier skill links could not be rolled back: {error}")
        })?;
        return Err(error);
      }
    }
  }
  Ok(results)
}

pub fn rollback_links(results: &[LinkResult]) -> Result<()> {
  for result in results.iter().rev() {
    if result.status != "linked" {
      continue;
    }
    if result.target.exists() || fs::symlink_metadata(&result.target).is_ok() {
      if !points_to(&result.target, &result.source) {
        bail!(
          "{} changed before the failed skill link operation could be rolled back.",
          result.target.display()
        );
      }
      remove_directory_link(&result.target)?;
    }
    if let Some(backup) = &result.backup {
      fs::rename(backup, &result.target)?;
    }
    remove_created_directories(&result.created_directories)?;
  }
  Ok(())
}

pub fn finalize_links(results: &[LinkResult]) -> Result<()> {
  for result in results {
    if let Some(backup) = &result.backup {
      #[cfg(windows)]
      if junction::exists(backup)? {
        remove_directory_link(backup)?;
        continue;
      }
      let metadata = fs::symlink_metadata(backup)?;
      if metadata.is_dir() && !metadata.file_type().is_symlink() {
        fs::remove_dir_all(backup)?;
      } else {
        fs::remove_file(backup)?;
      }
    }
  }
  Ok(())
}

pub fn unlink(source: &Path, name: &str, target_roots: &[PathBuf]) -> Result<Vec<LinkResult>> {
  unlink_with_checkpoint(source, name, target_roots, |_, _, _| Ok(()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnlinkPhase {
  Stage,
  Finalize,
}

fn unlink_with_checkpoint(
  source: &Path,
  name: &str,
  target_roots: &[PathBuf],
  mut checkpoint: impl FnMut(UnlinkPhase, &Path, usize) -> Result<()>,
) -> Result<Vec<LinkResult>> {
  validate_source(source, name)?;
  let source = dunce::canonicalize(source)?;
  let roots = unique_paths(target_roots);
  for root in &roots {
    let target = root.join(name);
    if (target.exists() || fs::symlink_metadata(&target).is_ok()) && !points_to(&target, &source) {
      bail!(
        "{} is not an exact link to the selected Arcantry skill; nothing was removed.",
        target.display()
      );
    }
  }
  let mut results = Vec::new();
  let mut staged = Vec::new();
  for (index, root) in roots.into_iter().enumerate() {
    let target = root.join(name);
    if target.exists() || fs::symlink_metadata(&target).is_ok() {
      let parent = target
        .parent()
        .context("Skill link target has no parent directory.")?;
      let placeholder = tempfile::Builder::new()
        .prefix(".arcantry-unlink-")
        .tempfile_in(parent)?;
      let backup = placeholder.path().to_path_buf();
      placeholder.close()?;
      if let Err(error) = fs::rename(&target, &backup) {
        restore_unlinked(&source, &staged).with_context(|| {
          format!("Unlinking failed and earlier skill links could not be restored: {error}")
        })?;
        return Err(error.into());
      }
      staged.push((target.clone(), backup));
      results.push(LinkResult {
        status: "unlinked",
        source: source.clone(),
        target: target.clone(),
        backup: None,
        created_directories: Vec::new(),
      });
      if let Err(error) = checkpoint(UnlinkPhase::Stage, &target, index) {
        restore_unlinked(&source, &staged).with_context(|| {
          format!("Unlinking failed and removed skill links could not be restored: {error}")
        })?;
        return Err(error);
      }
    } else {
      results.push(LinkResult {
        status: "unchanged",
        source: source.clone(),
        target,
        backup: None,
        created_directories: Vec::new(),
      });
    }
  }
  for (index, (target, backup)) in staged.iter().enumerate() {
    if let Err(error) = checkpoint(UnlinkPhase::Finalize, target, index) {
      restore_unlinked(&source, &staged).with_context(|| {
        format!("Unlink finalization failed and skill links could not be restored: {error}")
      })?;
      return Err(error);
    }
    if let Err(error) = remove_directory_link(backup) {
      restore_unlinked(&source, &staged).with_context(|| {
        format!("Unlink finalization failed and skill links could not be restored: {error}")
      })?;
      return Err(error);
    }
  }
  Ok(results)
}

fn missing_directories(parent: &Path) -> Vec<PathBuf> {
  let mut missing = parent
    .ancestors()
    .take_while(|path| !path.exists())
    .map(Path::to_path_buf)
    .collect::<Vec<_>>();
  missing.reverse();
  missing
}

fn retain_created_directories(directories: &mut Vec<PathBuf>) {
  directories.retain(|path| path.is_dir());
}

fn remove_created_directories(directories: &[PathBuf]) -> Result<()> {
  for directory in directories.iter().rev() {
    match fs::remove_dir(directory) {
      Ok(()) => {}
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
      Err(error) => return Err(error.into()),
    }
  }
  Ok(())
}

fn restore_unlinked(source: &Path, staged: &[(PathBuf, PathBuf)]) -> Result<()> {
  for (target, backup) in staged.iter().rev() {
    if target.exists() || fs::symlink_metadata(target).is_ok() {
      bail!(
        "{} changed before the failed skill unlink operation could be rolled back.",
        target.display()
      );
    }
    if backup.exists() || fs::symlink_metadata(backup).is_ok() {
      fs::rename(backup, target)?;
    } else {
      create_directory_link(source, target)?;
    }
  }
  Ok(())
}

pub fn user_target() -> Result<PathBuf> {
  Ok(
    UserDirs::new()
      .context("Could not resolve the user home directory.")?
      .home_dir()
      .join(".agents")
      .join("skills"),
  )
}
pub fn user_claude_target() -> Result<PathBuf> {
  Ok(
    UserDirs::new()
      .context("Could not resolve the user home directory.")?
      .home_dir()
      .join(".claude")
      .join("skills"),
  )
}
pub fn repo_target(root: &Path) -> PathBuf {
  root.join(".agents").join("skills")
}
pub fn repo_claude_target(root: &Path) -> PathBuf {
  root.join(".claude").join("skills")
}

fn validate_source(source: &Path, name: &str) -> Result<()> {
  let skill = source.join("SKILL.md");
  if !skill.is_file() {
    bail!("Skill package is missing SKILL.md: {}", source.display());
  }
  let (frontmatter_name, _) = read_frontmatter(&fs::read_to_string(skill)?)?;
  if frontmatter_name != name {
    bail!("Skill frontmatter name must match {name}.");
  }
  Ok(())
}
fn read_frontmatter(source: &str) -> Result<(String, String)> {
  let body = source
    .strip_prefix("---\n")
    .or_else(|| source.strip_prefix("---\r\n"))
    .context("Missing YAML frontmatter.")?;
  let end = body.find("\n---").context("Missing YAML frontmatter.")?;
  let frontmatter = &body[..end];
  let scalar = |key: &str| -> Result<String> {
    let prefix = format!("{key}:");
    let value = frontmatter
      .lines()
      .find_map(|line| line.strip_prefix(&prefix))
      .context(format!("Missing {key} in YAML frontmatter."))?
      .trim()
      .trim_matches(['\'', '"'])
      .to_owned();
    Ok(value)
  };
  Ok((scalar("name")?, scalar("description")?))
}
fn validate_name(name: &str) -> Result<()> {
  if name.is_empty()
    || name.len() > 63
    || name.split('-').any(|part| {
      part.is_empty()
        || !part
          .chars()
          .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit())
    })
  {
    bail!("Invalid skill name: {name}");
  }
  Ok(())
}
fn points_to(target: &Path, source: &Path) -> bool {
  dunce::canonicalize(target)
    .ok()
    .zip(dunce::canonicalize(source).ok())
    .is_some_and(|(left, right)| {
      if cfg!(windows) {
        left
          .to_string_lossy()
          .eq_ignore_ascii_case(&right.to_string_lossy())
      } else {
        left == right
      }
    })
}
fn unique_paths(paths: &[PathBuf]) -> Vec<PathBuf> {
  let mut seen = BTreeSet::new();
  paths
    .iter()
    .filter_map(|path| {
      let absolute = if path.is_absolute() {
        path.clone()
      } else {
        std::env::current_dir().ok()?.join(path)
      };
      let key = if cfg!(windows) {
        absolute.to_string_lossy().to_lowercase()
      } else {
        absolute.to_string_lossy().into_owned()
      };
      seen.insert(key).then_some(absolute)
    })
    .collect()
}
fn preflight_link(source: &Path, name: &str, root: &Path, replace: bool) -> Result<()> {
  let target = root.join(name);
  if target.exists() || fs::symlink_metadata(&target).is_ok() {
    if points_to(&target, source) || replace {
      return Ok(());
    }
    bail!(
      "{} is not an exact link. Use --replace to back it up before linking.",
      target.display()
    );
  }
  if let Some(existing) = root
    .ancestors()
    .find(|path| path.exists() || fs::symlink_metadata(path).is_ok())
    && !existing.is_dir()
  {
    bail!(
      "Skill link destination parent is not a directory: {}",
      existing.display()
    );
  }
  Ok(())
}
fn next_backup(target: &Path) -> PathBuf {
  let base = PathBuf::from(format!("{}.backup", target.display()));
  if !base.exists() {
    return base;
  }
  (2..)
    .map(|index| PathBuf::from(format!("{}.backup-{index}", target.display())))
    .find(|path| !path.exists())
    .unwrap()
}
#[cfg(windows)]
fn create_directory_link(source: &Path, target: &Path) -> Result<()> {
  if let Some(parent) = target.parent() {
    fs::create_dir_all(parent)?;
  }
  junction::create(source, target)?;
  Ok(())
}
#[cfg(not(windows))]
fn create_directory_link(source: &Path, target: &Path) -> Result<()> {
  if let Some(parent) = target.parent() {
    fs::create_dir_all(parent)?;
  }
  std::os::unix::fs::symlink(source, target)?;
  Ok(())
}
#[cfg(windows)]
fn remove_directory_link(target: &Path) -> Result<()> {
  junction::delete(target)?;
  fs::remove_dir(target)?;
  Ok(())
}
#[cfg(not(windows))]
fn remove_directory_link(target: &Path) -> Result<()> {
  fs::remove_file(target)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn skill_fixture() -> (tempfile::TempDir, PathBuf) {
    let fixture = tempfile::tempdir().unwrap();
    let source = fixture.path().join("source").join("example");
    fs::create_dir_all(&source).unwrap();
    fs::write(
      source.join("SKILL.md"),
      "---\nname: example\ndescription: Example skill package for atomic link testing.\n---\n",
    )
    .unwrap();
    (fixture, source)
  }

  fn catalog_fixture() -> tempfile::TempDir {
    let fixture = tempfile::tempdir().unwrap();
    let schemas = fixture.path().join("schemas");
    let skill = fixture.path().join("skills").join("example-skill");
    fs::create_dir_all(skill.join("agents")).unwrap();
    fs::create_dir_all(&schemas).unwrap();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for schema in ["catalog.schema.json", "skill-metadata.schema.json"] {
      fs::copy(workspace.join("schemas").join(schema), schemas.join(schema)).unwrap();
    }
    fs::write(
      fixture.path().join("catalog.json"),
      serde_json::to_string_pretty(&serde_json::json!({
        "$schema": "./schemas/catalog.schema.json",
        "skills": [{
          "name": "example-skill",
          "family": "repo-safely",
          "tags": ["example"]
        }]
      }))
      .unwrap(),
    )
    .unwrap();
    fs::write(
      skill.join("SKILL.md"),
      "---\nname: example-skill\ndescription: Use this example skill for a concrete catalog validation task.\n---\n\n# Example\n",
    )
    .unwrap();
    fs::write(
      skill.join("arcantry.json"),
      serde_json::to_string_pretty(&serde_json::json!({
        "$schema": "../../schemas/skill-metadata.schema.json",
        "version": "1.0.0",
        "role": "primary",
        "summary": "Validate one complete skill package in the Arcantry catalog.",
        "scenarios": [
          {
            "title": "First case",
            "prompt": "Use the example skill for the first task.",
            "outcome": "The first task is complete."
          },
          {
            "title": "Second case",
            "prompt": "Use the example skill for the second task.",
            "outcome": "The second task is complete."
          }
        ]
      }))
      .unwrap(),
    )
    .unwrap();
    fs::write(
      skill.join("agents").join("openai.yaml"),
      "interface:\n  display_name: Example\n  short_description: Validate an example package\n  default_prompt: Use $example-skill to validate this example.\n",
    )
    .unwrap();
    fixture
  }

  #[test]
  fn link_preflights_every_target_before_creating_any_link() {
    let fixture = tempfile::tempdir().unwrap();
    let source = fixture.path().join("source").join("example");
    fs::create_dir_all(&source).unwrap();
    fs::write(
      source.join("SKILL.md"),
      "---\nname: example\ndescription: Example skill package for atomic link testing.\n---\n",
    )
    .unwrap();
    let standard = fixture.path().join(".agents").join("skills");
    let blocked = fixture.path().join(".claude").join("skills");
    fs::create_dir_all(blocked.parent().unwrap()).unwrap();
    fs::write(&blocked, "not a directory").unwrap();

    assert!(
      link(&source, "example", &[standard.clone(), blocked], false)
        .unwrap_err()
        .to_string()
        .contains("not a directory")
    );
    assert!(!standard.join("example").exists());
  }

  #[test]
  fn rollback_removes_a_created_link() {
    let fixture = tempfile::tempdir().unwrap();
    let source = fixture.path().join("source").join("example");
    fs::create_dir_all(&source).unwrap();
    fs::write(
      source.join("SKILL.md"),
      "---\nname: example\ndescription: Example skill package for atomic link testing.\n---\n",
    )
    .unwrap();
    let target = fixture.path().join(".agents").join("skills");
    let results = link(&source, "example", std::slice::from_ref(&target), false).unwrap();

    rollback_links(&results).unwrap();

    assert!(!target.join("example").exists());
    assert!(fs::symlink_metadata(target.join("example")).is_err());
  }

  #[test]
  fn injected_multi_target_link_failure_removes_links_and_created_directories() {
    let (fixture, source) = skill_fixture();
    let standard = fixture.path().join(".agents").join("skills");
    let compatible = fixture.path().join(".claude").join("skills");

    let error = link_with_checkpoint(
      &source,
      "example",
      &[standard.clone(), compatible.clone()],
      false,
      |_, index| {
        if index == 0 {
          bail!("injected link failure");
        }
        Ok(())
      },
    )
    .unwrap_err();

    assert!(error.to_string().contains("injected link failure"));
    assert!(fs::symlink_metadata(standard.join("example")).is_err());
    assert!(fs::symlink_metadata(compatible.join("example")).is_err());
    assert!(!fixture.path().join(".agents").exists());
    assert!(!fixture.path().join(".claude").exists());
  }

  #[test]
  fn injected_multi_target_unlink_failure_restores_every_link() {
    let (fixture, source) = skill_fixture();
    let standard = fixture.path().join(".agents").join("skills");
    let compatible = fixture.path().join(".claude").join("skills");
    let targets = [standard.clone(), compatible.clone()];
    link(&source, "example", &targets, false).unwrap();

    let error = unlink_with_checkpoint(&source, "example", &targets, |phase, _, index| {
      if phase == UnlinkPhase::Stage && index == 0 {
        bail!("injected unlink failure");
      }
      Ok(())
    })
    .unwrap_err();

    assert!(error.to_string().contains("injected unlink failure"));
    for target in targets {
      assert!(points_to(&target.join("example"), &source));
      assert_eq!(fs::read_dir(target).unwrap().count(), 1);
    }
  }

  #[test]
  fn injected_unlink_finalization_failure_restores_every_link() {
    let (fixture, source) = skill_fixture();
    let standard = fixture.path().join(".agents").join("skills");
    let compatible = fixture.path().join(".claude").join("skills");
    let targets = [standard.clone(), compatible.clone()];
    link(&source, "example", &targets, false).unwrap();

    let error = unlink_with_checkpoint(&source, "example", &targets, |phase, _, index| {
      if phase == UnlinkPhase::Finalize && index == 1 {
        bail!("injected unlink finalization failure");
      }
      Ok(())
    })
    .unwrap_err();

    assert!(
      error
        .to_string()
        .contains("injected unlink finalization failure")
    );
    for target in targets {
      assert!(points_to(&target.join("example"), &source));
      assert_eq!(fs::read_dir(target).unwrap().count(), 1);
    }
  }

  #[test]
  fn injected_replacement_failure_restores_the_original_directory() {
    let (fixture, source) = skill_fixture();
    let target_root = fixture.path().join(".agents").join("skills");
    let target = target_root.join("example");
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join("owned-by-user.txt"), "preserve\n").unwrap();

    let error = link_with_checkpoint(
      &source,
      "example",
      std::slice::from_ref(&target_root),
      true,
      |_, _| bail!("injected replacement failure"),
    )
    .unwrap_err();

    assert!(error.to_string().contains("injected replacement failure"));
    assert_eq!(
      fs::read_to_string(target.join("owned-by-user.txt")).unwrap(),
      "preserve\n"
    );
    assert_eq!(fs::read_dir(&target_root).unwrap().count(), 1);
  }

  #[test]
  fn schema_validation_rejects_unknown_catalog_fields() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let value = serde_json::json!({
      "$schema": "./schemas/catalog.schema.json",
      "skills": [{
        "name": "example",
        "family": "repo-safely",
        "tags": ["example"],
        "unsupported": true
      }]
    });

    assert!(
      validate_json_document(
        &root.join("schemas/catalog.schema.json"),
        &value,
        "catalog.json"
      )
      .unwrap_err()
      .to_string()
      .contains("Additional properties")
    );
  }

  #[test]
  fn schema_validation_rejects_invalid_skill_metadata() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let value = serde_json::json!({
      "$schema": "../../schemas/skill-metadata.schema.json",
      "version": "1.0.0",
      "role": "primary",
      "summary": "Too short",
      "scenarios": []
    });

    assert!(
      validate_json_document(
        &root.join("schemas/skill-metadata.schema.json"),
        &value,
        "skills/example/arcantry.json"
      )
      .is_err()
    );
  }

  #[test]
  fn catalog_validation_accepts_a_complete_canonical_package() {
    let fixture = catalog_fixture();

    let (valid, errors, _) = validate(fixture.path());

    assert!(valid, "{errors:?}");
    assert!(errors.is_empty());
  }

  #[test]
  fn catalog_validation_rejects_membership_prompt_and_resource_drift() {
    let fixture = catalog_fixture();
    fs::create_dir_all(fixture.path().join("skills").join("unlisted-skill")).unwrap();
    fs::write(
      fixture
        .path()
        .join("skills/example-skill/agents/openai.yaml"),
      "interface:\n  display_name: Example\n  short_description: Validate an example package\n  default_prompt: Validate this example.\n",
    )
    .unwrap();
    fs::write(
      fixture.path().join("skills/example-skill/SKILL.md"),
      "---\nname: example-skill\ndescription: Use this example skill for a concrete catalog validation task.\n---\n\n[Missing](references/missing.md)\n",
    )
    .unwrap();

    let (valid, errors, _) = validate(fixture.path());

    assert!(!valid);
    assert!(errors.iter().any(|error| error.contains("membership")));
    assert!(
      errors
        .iter()
        .any(|error| error.contains("must mention $example-skill"))
    );
    assert!(
      errors
        .iter()
        .any(|error| error.contains("references missing references/missing.md"))
    );
  }
}
