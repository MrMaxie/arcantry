use crate::native_targets::TARGETS;
use crate::repository_release;
use anyhow::{Context, Result, bail};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha512};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Stdio};

const MAIN_PACKAGE: &str = "arcantry";
const REPOSITORY_URL: &str = "https://github.com/MrMaxie/arcantry.git";

#[derive(Debug, Subcommand)]
pub enum Command {
  /// Verify the sealed release tag and public package identity.
  Check {
    #[arg(long)]
    tag: String,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Require an npm CLI version that supports trusted publishing.
  VerifyNpmCli,
  /// Verify retry-safe npm publication state.
  PreflightNpm {
    #[arg(long)]
    archives: PathBuf,
    #[arg(long)]
    output: PathBuf,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Publish missing platform packages followed by the main package.
  PublishNpm {
    #[arg(long)]
    archives: PathBuf,
    #[arg(long)]
    existing: PathBuf,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Create or refresh the draft GitHub Release from verified artifacts.
  CreateDraftRelease {
    #[arg(long)]
    tag: String,
    #[arg(long)]
    artifacts: PathBuf,
  },
  /// Make the draft GitHub Release public.
  PublishRelease {
    #[arg(long)]
    tag: String,
  },
}

#[derive(Clone, Debug, Deserialize)]
struct PackageMetadata {
  name: String,
  version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GitHubRelease {
  is_draft: bool,
  name: String,
  tag_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PublicationIdentity {
  package_name: String,
  version: String,
  tag: String,
  repository_url: String,
}

struct ProcessOutput {
  success: bool,
  stdout: String,
  stderr: String,
}

pub fn run(command: Command) -> Result<()> {
  match command {
    Command::Check { tag, root } => {
      println!(
        "{}",
        serde_json::to_string_pretty(&validate_publication(&root, &tag)?)?
      );
      Ok(())
    }
    Command::VerifyNpmCli => verify_npm_cli(),
    Command::PreflightNpm {
      archives,
      output,
      root,
    } => preflight_npm(&root, &archives, &output),
    Command::PublishNpm {
      archives,
      existing,
      root,
    } => publish_npm(&root, &archives, &existing),
    Command::CreateDraftRelease { tag, artifacts } => create_draft_release(&tag, &artifacts),
    Command::PublishRelease { tag } => {
      run_checked("gh", &["release", "edit", &tag, "--draft=false"])?;
      Ok(())
    }
  }
}

fn validate_publication(root: &Path, tag: &str) -> Result<PublicationIdentity> {
  if !is_stable_tag(tag) {
    bail!("invalid npm release tag: {tag}");
  }
  let project = repository_release::project(root)?;
  let version = arcantry_core::release::latest_standard_release_version(&project.root)?;
  if tag != format!("v{version}") {
    bail!("npm release tag {tag} does not match release {version}");
  }

  let package_path = project.root.join("packages/arcantry/package.json");
  let package: Value = serde_json::from_str(
    &fs::read_to_string(&package_path)
      .with_context(|| format!("failed to read {}", package_path.display()))?,
  )?;
  let package_name = package
    .get("name")
    .and_then(Value::as_str)
    .unwrap_or_default();
  if package_name != MAIN_PACKAGE {
    bail!("npm package name must be {MAIN_PACKAGE}");
  }
  if package.get("version").and_then(Value::as_str) != Some(version.as_str()) {
    bail!("npm package version does not match tag {tag}");
  }
  let repository_url = match package.get("repository") {
    Some(Value::String(value)) => value.as_str(),
    Some(Value::Object(value)) => value.get("url").and_then(Value::as_str).unwrap_or_default(),
    _ => "",
  };
  if repository_url != REPOSITORY_URL {
    bail!("npm package repository must match {REPOSITORY_URL}");
  }
  arcantry_core::release::validate_publication_state(
    &project,
    None,
    arcantry_core::release::github_pull_request_head().as_deref(),
  )?;
  let head = git(&project.root, &["rev-parse", "HEAD"])?;
  let tag_commit = git(&project.root, &["rev-list", "-n", "1", tag])?;
  if tag_commit != head {
    bail!("npm release tag {tag} does not point to repository HEAD");
  }
  Ok(PublicationIdentity {
    package_name: package_name.to_owned(),
    version,
    tag: tag.to_owned(),
    repository_url: repository_url.to_owned(),
  })
}

fn is_stable_tag(tag: &str) -> bool {
  let Some(version) = tag.strip_prefix('v') else {
    return false;
  };
  let parts = version.split('.').collect::<Vec<_>>();
  parts.len() == 3
    && parts.iter().all(|part| {
      !part.is_empty()
        && part.bytes().all(|byte| byte.is_ascii_digit())
        && (part == &"0" || !part.starts_with('0'))
    })
}

fn verify_npm_cli() -> Result<()> {
  let version = run_checked("npm", &["--version"])?;
  if !npm_version_supported(version.trim()) {
    bail!("npm 11.5.1 or newer is required.");
  }
  Ok(())
}

fn npm_version_supported(version: &str) -> bool {
  let parts = version
    .split('.')
    .take(3)
    .map(str::parse::<u64>)
    .collect::<std::result::Result<Vec<_>, _>>();
  let Ok(parts) = parts else {
    return false;
  };
  if parts.len() != 3 {
    return false;
  }
  (parts[0], parts[1], parts[2]) >= (11, 5, 1)
}

fn package_manifests(root: &Path) -> Result<Vec<PackageMetadata>> {
  let paths = TARGETS
    .iter()
    .map(|target| {
      root
        .join("packages")
        .join(target.package_directory)
        .join("package.json")
    })
    .chain(std::iter::once(root.join("packages/arcantry/package.json")));
  paths
    .map(|path| {
      serde_json::from_str(
        &fs::read_to_string(&path)
          .with_context(|| format!("failed to read npm package manifest {}", path.display()))?,
      )
      .with_context(|| format!("invalid npm package manifest {}", path.display()))
    })
    .collect()
}

fn archive_name(metadata: &PackageMetadata) -> String {
  format!(
    "{}-{}.tgz",
    metadata.name.trim_start_matches('@').replace('/', "-"),
    metadata.version
  )
}

fn preflight_npm(root: &Path, archives: &Path, output: &Path) -> Result<()> {
  let archives = absolutize(archives)?;
  let mut existing = Vec::new();
  for metadata in package_manifests(root)? {
    let name = archive_name(&metadata);
    let archive = archives.join(&name);
    let expected = format!(
      "sha512-{}",
      STANDARD.encode(Sha512::digest(fs::read(&archive)?))
    );
    let package_version = format!("{}@{}", metadata.name, metadata.version);
    let version = run_output(
      "npm",
      &["view", &package_version, "dist.integrity", "--json"],
    )?;
    if version.success {
      let actual: String = serde_json::from_str(&version.stdout)?;
      validate_existing_archive(&metadata, &expected, &actual)?;
      existing.push(name);
      continue;
    }
    if !version.stderr.contains("E404") {
      bail!(process_error(
        &version,
        &format!("npm view failed for {package_version}.")
      ));
    }

    let package = run_output("npm", &["view", &metadata.name, "name", "--json"])?;
    if !package.success {
      if package.stderr.contains("E404") {
        bail!(
          "{} requires the separately authorized 2FA bootstrap publication.",
          metadata.name
        );
      }
      bail!(process_error(
        &package,
        &format!("npm view failed for {}.", metadata.name)
      ));
    }
  }
  existing.sort();
  let content = if existing.is_empty() {
    String::new()
  } else {
    format!("{}\n", existing.join("\n"))
  };
  fs::write(absolutize(output)?, content)?;
  Ok(())
}

fn validate_existing_archive(
  metadata: &PackageMetadata,
  expected: &str,
  actual: &str,
) -> Result<()> {
  if actual != expected {
    bail!(
      "{}@{} exists with different integrity.",
      metadata.name,
      metadata.version
    );
  }
  Ok(())
}

fn publish_npm(root: &Path, archives: &Path, existing: &Path) -> Result<()> {
  let archives = absolutize(archives)?;
  let existing = fs::read_to_string(absolutize(existing)?)?
    .lines()
    .map(str::to_owned)
    .collect::<std::collections::HashSet<_>>();
  for metadata in package_manifests(root)? {
    let name = archive_name(&metadata);
    if existing.contains(&name) {
      println!("Already verified on npm: {name}");
    } else {
      let archive = archives.join(&name).to_string_lossy().into_owned();
      run_checked("npm", &["publish", &archive])?;
    }
  }
  Ok(())
}

fn create_draft_release(tag: &str, artifacts: &Path) -> Result<()> {
  let artifacts = absolutize(artifacts)?;
  let notes_directory = tempfile::tempdir()?;
  let notes_path = notes_directory.path().join("release-notes.md");
  fs::write(&notes_path, release_notes(&std::env::current_dir()?, tag)?)?;
  let notes_path = notes_path.to_string_lossy().into_owned();
  let release_artifacts = TARGETS
    .iter()
    .map(|target| artifacts.join(target.archive))
    .chain([
      artifacts.join("SHA256SUMS"),
      artifacts.join("arcantry-installer.sh"),
      artifacts.join("arcantry-installer.ps1"),
    ])
    .map(|path| path.to_string_lossy().into_owned())
    .collect::<Vec<_>>();
  let existing = run_output(
    "gh",
    &["release", "view", tag, "--json", "isDraft,name,tagName"],
  )?;
  if existing.success {
    let release: GitHubRelease = serde_json::from_str(&existing.stdout)?;
    validate_draft_identity(tag, &release)?;
    run_checked("gh", &["release", "edit", tag, "--notes-file", &notes_path])?;
    let mut arguments = vec!["release".to_owned(), "upload".to_owned(), tag.to_owned()];
    arguments.extend(release_artifacts);
    arguments.push("--clobber".to_owned());
    run_checked_owned("gh", &arguments)?;
    return Ok(());
  }
  if !format!("{}\n{}", existing.stdout, existing.stderr)
    .to_lowercase()
    .contains("release not found")
  {
    bail!(process_error(
      &existing,
      &format!("Unable to inspect GitHub Release {tag}.")
    ));
  }
  let mut arguments = vec![
    "release".to_owned(),
    "create".to_owned(),
    tag.to_owned(),
    "--draft".to_owned(),
    "--verify-tag".to_owned(),
    "--title".to_owned(),
    tag.to_owned(),
    "--notes-file".to_owned(),
    notes_path,
  ];
  arguments.extend(release_artifacts);
  run_checked_owned("gh", &arguments)?;
  Ok(())
}

fn release_notes(root: &Path, tag: &str) -> Result<String> {
  let version = tag
    .strip_prefix('v')
    .with_context(|| format!("invalid release tag: {tag}"))?;
  let changelog = fs::read_to_string(root.join("CHANGELOG.md"))?.replace("\r\n", "\n");
  let heading = format!("## [{version}] - ");
  let start = changelog
    .find(&heading)
    .with_context(|| format!("CHANGELOG.md has no release section for {version}"))?;
  let section = &changelog[start..];
  let end = section[heading.len()..]
    .find("\n## [")
    .map(|index| index + heading.len())
    .unwrap_or(section.len());
  let section = section[..end].trim();
  if section.lines().count() < 2 {
    bail!("CHANGELOG.md release section for {version} is empty");
  }
  Ok(format!("{section}\n"))
}

fn validate_draft_identity(tag: &str, release: &GitHubRelease) -> Result<()> {
  if !release.is_draft {
    bail!("GitHub Release {tag} is already public.");
  }
  if release.tag_name != tag || release.name != tag {
    bail!("Existing draft {tag} does not match the verified release identity.");
  }
  Ok(())
}

fn git(root: &Path, arguments: &[&str]) -> Result<String> {
  let mut command = ProcessCommand::new("git");
  command.current_dir(root);
  let output = command_output(&mut command, arguments)?;
  if !output.success {
    bail!("release sealing requires a readable Git repository");
  }
  Ok(output.stdout.trim().to_owned())
}

fn absolutize(path: &Path) -> Result<PathBuf> {
  if path.is_absolute() {
    Ok(path.to_owned())
  } else {
    Ok(std::env::current_dir()?.join(path))
  }
}

fn run_checked(command: &str, arguments: &[&str]) -> Result<String> {
  let output = run_output(command, arguments)?;
  if !output.success {
    bail!(process_error(&output, &format!("{command} failed.")));
  }
  Ok(output.stdout.trim_end().to_owned())
}

fn run_checked_owned(command: &str, arguments: &[String]) -> Result<String> {
  let references = arguments.iter().map(String::as_str).collect::<Vec<_>>();
  run_checked(command, &references)
}

fn run_output(command: &str, arguments: &[&str]) -> Result<ProcessOutput> {
  #[cfg(windows)]
  let program = if ["npm", "npx", "pnpm"].contains(&command) {
    format!("{command}.cmd")
  } else {
    command.to_owned()
  };
  #[cfg(not(windows))]
  let program = command.to_owned();
  let mut process = ProcessCommand::new(program);
  process.env_remove("NODE_OPTIONS");
  command_output(&mut process, arguments)
}

fn command_output(process: &mut ProcessCommand, arguments: &[&str]) -> Result<ProcessOutput> {
  let output = process
    .args(arguments)
    .stdin(Stdio::null())
    .output()
    .with_context(|| {
      format!(
        "failed to execute {}",
        process.get_program().to_string_lossy()
      )
    })?;
  Ok(ProcessOutput {
    success: output.status.success(),
    stdout: String::from_utf8(output.stdout).context("command stdout is not UTF-8")?,
    stderr: String::from_utf8(output.stderr).context("command stderr is not UTF-8")?,
  })
}

fn process_error(output: &ProcessOutput, fallback: &str) -> String {
  let stderr = output.stderr.trim();
  if stderr.is_empty() {
    fallback.to_owned()
  } else {
    stderr.to_owned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn accepts_only_stable_canonical_release_tags() {
    assert!(is_stable_tag("v1.0.0"));
    assert!(is_stable_tag("v11.25.3"));
    for tag in ["1.0.0", "v1.0", "v01.0.0", "v1.0.0-beta", "v1.0.0+build"] {
      assert!(!is_stable_tag(tag), "accepted {tag}");
    }
  }

  #[test]
  fn requires_the_trusted_publishing_npm_floor() {
    assert!(!npm_version_supported("11.5.0"));
    assert!(npm_version_supported("11.5.1"));
    assert!(npm_version_supported("12.0.0"));
    assert!(!npm_version_supported("invalid"));
  }

  #[test]
  fn derives_registry_archive_names_for_scoped_and_main_packages() {
    assert_eq!(
      archive_name(&PackageMetadata {
        name: "@arcantry/cli-linux-x64".to_owned(),
        version: "1.0.0".to_owned(),
      }),
      "arcantry-cli-linux-x64-1.0.0.tgz"
    );
    assert_eq!(
      archive_name(&PackageMetadata {
        name: MAIN_PACKAGE.to_owned(),
        version: "1.0.0".to_owned(),
      }),
      "arcantry-1.0.0.tgz"
    );
  }

  #[test]
  fn accepts_exact_existing_packages_and_rejects_integrity_drift() {
    let main = PackageMetadata {
      name: MAIN_PACKAGE.to_owned(),
      version: "1.0.0".to_owned(),
    };
    validate_existing_archive(&main, "sha512-same", "sha512-same").unwrap();
    let platform = PackageMetadata {
      name: "@arcantry/cli-linux-x64".to_owned(),
      version: "1.0.0".to_owned(),
    };
    assert!(
      validate_existing_archive(&platform, "sha512-expected", "sha512-other")
        .unwrap_err()
        .to_string()
        .contains("different integrity")
    );
    validate_existing_archive(&platform, "sha512-same", "sha512-same").unwrap();
  }

  #[test]
  fn accepts_only_a_matching_private_github_release_draft() {
    validate_draft_identity(
      "v1.0.0",
      &GitHubRelease {
        is_draft: true,
        name: "v1.0.0".to_owned(),
        tag_name: "v1.0.0".to_owned(),
      },
    )
    .unwrap();
    assert!(
      validate_draft_identity(
        "v1.0.0",
        &GitHubRelease {
          is_draft: false,
          name: "v1.0.0".to_owned(),
          tag_name: "v1.0.0".to_owned(),
        },
      )
      .unwrap_err()
      .to_string()
      .contains("already public")
    );
  }

  #[test]
  fn pins_every_github_artifact_action_to_a_full_commit_sha() {
    let workflow = fs::read_to_string(
      Path::new(env!("CARGO_MANIFEST_DIR")).join("../.github/workflows/release.yml"),
    )
    .unwrap();
    let actions = workflow
      .split_whitespace()
      .filter(|token| {
        token.starts_with("actions/upload-artifact@")
          || token.starts_with("actions/download-artifact@")
      })
      .collect::<Vec<_>>();
    assert_eq!(actions.len(), 6);
    assert!(actions.iter().all(|action| {
      action.rsplit_once('@').is_some_and(|(_, reference)| {
        reference.len() == 40 && reference.bytes().all(|byte| byte.is_ascii_hexdigit())
      })
    }));
  }

  #[test]
  fn keeps_release_publication_behind_verified_artifacts_and_environment_approval() {
    let workflow = fs::read_to_string(
      Path::new(env!("CARGO_MANIFEST_DIR")).join("../.github/workflows/release.yml"),
    )
    .unwrap();
    assert!(workflow.contains("name: release-${{ github.ref_name }}"));
    assert!(!workflow.contains("release-v1.0.0"));
    assert!(
      workflow.contains("draft-release:\n    needs:\n      - assemble\n      - installer-smoke")
    );
    assert!(workflow.contains("publish:\n    needs: draft-release"));
    assert!(workflow.contains("environment: npm"));
    assert!(workflow.contains("id-token: write"));
    let draft = workflow.find("draft-release:").unwrap();
    let publish = workflow.find("\n  publish:").unwrap();
    let create_draft = workflow.find("publish create-draft-release").unwrap();
    let publish_npm = workflow.find("publish publish-npm").unwrap();
    let publish_release = workflow.find("publish publish-release").unwrap();
    assert!(draft < create_draft && create_draft < publish);
    assert!(publish < publish_npm && publish_npm < publish_release);
  }

  #[test]
  fn extracts_exact_openspec_managed_release_notes() {
    let root = tempfile::tempdir().unwrap();
    fs::write(
      root.path().join("CHANGELOG.md"),
      "# Changelog\n\n## [Unreleased]\n\n## [1.0.0] - 2026-09-14\n\n### Added\n\nOne outcome.\n\n## [0.4.3] - 2026-08-18\n\nOlder.\n",
    )
    .unwrap();
    assert_eq!(
      release_notes(root.path(), "v1.0.0").unwrap(),
      "## [1.0.0] - 2026-09-14\n\n### Added\n\nOne outcome.\n"
    );
  }
}
