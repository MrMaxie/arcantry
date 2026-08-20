use crate::config::{ProjectConfig, parse_project_config, render_project_config, resolve_project};
use crate::managed_content::{
  ManagedSectionResult, contains_managed_section, remove_managed_section, upsert_managed_section,
};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
  Shared,
  Private,
}
impl Scope {
  pub fn parse(value: &str) -> Result<Self> {
    match value {
      "shared" => Ok(Self::Shared),
      "private" => Ok(Self::Private),
      _ => bail!("Invalid option: expected one of \"shared\"|\"private\""),
    }
  }
  pub fn name(self) -> &'static str {
    match self {
      Self::Shared => "shared",
      Self::Private => "private",
    }
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryChange {
  pub action: &'static str,
  pub path: String,
}

#[derive(Debug, Clone)]
struct PlannedChange {
  action: &'static str,
  path: PathBuf,
  display_path: String,
  content: Option<String>,
  expected: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
enum LocalTrackingPolicy {
  Private,
  RemoteTracked(String),
  IndexOnly,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepositoryDiagnostic {
  pub severity: &'static str,
  pub path: String,
  pub message: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub repair: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryReport {
  pub root: PathBuf,
  pub valid: bool,
  pub diagnostics: Vec<RepositoryDiagnostic>,
  pub config_path: Option<PathBuf>,
  pub scope: Option<&'static str>,
}

const SHARED_GUIDANCE: &str = "## Arcantry\n\nUse `arcantry.toml` for shared Arcantry configuration.\nTreat configured OpenSpec sources as accepted product and engineering intent.\nUse configured todo.txt sources for quick intake and changelog sources for consumer-facing release history.";
const PRIVATE_GUIDANCE: &str = "## Arcantry local context\n\nTreat `.local/` as private operational state and read `.local/arcantry.toml` for private Arcantry configuration.\nKeep private and shared sources independent. Promote or relocate content only through an explicit reviewed operation.";

pub fn resolve_repository_root(cwd: &Path) -> Result<PathBuf> {
  let output = duct::cmd("git", ["rev-parse", "--show-toplevel"])
    .dir(cwd)
    .stderr_null()
    .read()
    .with_context(|| format!("No Git repository found from {}.", cwd.display()))?;
  let root = PathBuf::from(output.trim());
  Ok(dunce::canonicalize(&root).unwrap_or(root))
}

pub fn init(cwd: &Path, scope: Scope, compatibility: bool) -> Result<Vec<RepositoryChange>> {
  let root = resolve_repository_root(cwd)?;
  if scope == Scope::Private {
    enforce_private_local_policy(&root)?;
  }
  let mut changes = Vec::new();
  let config_relative = config_path(scope);
  let config_absolute = root.join(config_relative);
  match read_optional(&config_absolute)? {
    None => changes.push(PlannedChange {
      action: "create",
      path: config_absolute,
      display_path: native_relative(config_relative),
      content: Some(render_project_config(&ProjectConfig::empty())?),
      expected: None,
    }),
    Some(content) => {
      parse_project_config(&content, None, false)
        .context("Existing Arcantry configuration is invalid and will not be replaced.")?;
    }
  }
  plan_section(&root, guidance_path(scope), guidance(scope), &mut changes)?;
  if compatibility {
    plan_section(
      &root,
      claude_path(scope),
      &format!("@{}", guidance_path(scope).replace('\\', "/")),
      &mut changes,
    )?;
  }
  if scope == Scope::Private {
    plan_git_exclude(
      &root,
      if compatibility {
        &[".local/", "CLAUDE.local.md"]
      } else {
        &[".local/"]
      },
      &mut changes,
    )?;
  }
  apply_changes(changes)
}

pub fn update(cwd: &Path, scope: Scope, compatibility: bool) -> Result<Vec<RepositoryChange>> {
  let root = resolve_repository_root(cwd)?;
  if scope == Scope::Private {
    enforce_private_local_policy(&root)?;
  }
  let config_relative = config_path(scope);
  let Some(config) = read_optional(&root.join(config_relative))? else {
    bail!("Run repo init --scope {} before repo update.", scope.name());
  };
  parse_project_config(&config, None, false)
    .context("Configuration is invalid and will not be replaced.")?;
  let mut changes = Vec::new();
  plan_section(&root, guidance_path(scope), guidance(scope), &mut changes)?;
  if compatibility {
    plan_section(
      &root,
      claude_path(scope),
      &format!("@{}", guidance_path(scope).replace('\\', "/")),
      &mut changes,
    )?;
  }
  if scope == Scope::Private {
    plan_git_exclude(
      &root,
      if compatibility {
        &[".local/", "CLAUDE.local.md"]
      } else {
        &[".local/"]
      },
      &mut changes,
    )?;
  }
  apply_changes(changes)
}

pub fn remove(cwd: &Path, scope: Scope) -> Result<Vec<RepositoryChange>> {
  let root = resolve_repository_root(cwd)?;
  let mut changes = Vec::new();
  let config_relative = config_path(scope);
  let config_absolute = root.join(config_relative);
  if let Some(content) = read_optional(&config_absolute)? {
    parse_project_config(&content, None, false)
      .context("Invalid configuration is preserved because ownership cannot be verified.")?;
    changes.push(PlannedChange {
      action: "remove",
      path: config_absolute,
      display_path: native_relative(config_relative),
      content: None,
      expected: Some(content),
    });
  }
  plan_section_removal(&root, guidance_path(scope), &mut changes)?;
  plan_section_removal(&root, claude_path(scope), &mut changes)?;
  apply_changes(changes)
}

pub fn validate(
  cwd: &Path,
  config_path_override: Option<&Path>,
  doctor: bool,
) -> Result<RepositoryReport> {
  let repository_root = resolve_repository_root(cwd).ok();
  let root = repository_root.clone().unwrap_or_else(|| cwd.to_path_buf());
  let project = resolve_project(&root, config_path_override, true, Some(crate::VERSION))?;
  let mut diagnostics = Vec::new();
  let Some(config_path) = project.config_path.clone() else {
    return Ok(RepositoryReport {
      root,
      valid: true,
      diagnostics,
      config_path: None,
      scope: None,
    });
  };
  for shadowed in &project.shadowed_config_paths {
    diagnostics.push(RepositoryDiagnostic {
      severity: "warning",
      path: display_path(&root, shadowed),
      message: format!(
        "Configuration is shadowed by {}.",
        display_path(&root, &config_path)
      ),
      repair: None,
    });
  }
  if let Some(scope_name @ ("shared" | "private")) = project.scope {
    let scope = Scope::parse(scope_name)?;
    validate_section(
      &root,
      guidance_path(scope),
      guidance(scope),
      scope,
      doctor,
      &mut diagnostics,
    )?;
    validate_claude(&root, scope, doctor, &mut diagnostics)?;
    if scope == Scope::Private && repository_root.is_some() {
      validate_git_exclude(&root, doctor, &mut diagnostics)?;
    }
  }
  Ok(RepositoryReport {
    root,
    valid: diagnostics
      .iter()
      .all(|diagnostic| diagnostic.severity != "error"),
    diagnostics,
    config_path: Some(config_path),
    scope: project.scope,
  })
}

fn apply_changes(changes: Vec<PlannedChange>) -> Result<Vec<RepositoryChange>> {
  for change in &changes {
    if read_optional(&change.path)? != change.expected {
      bail!(
        "Refusing to change {}; it changed after the plan was created.",
        change.display_path
      );
    }
  }
  for change in &changes {
    if let Some(content) = &change.content {
      if let Some(parent) = change.path.parent() {
        fs::create_dir_all(parent)?;
      }
      fs::write(&change.path, content)?;
    } else if change.path.exists() {
      fs::remove_file(&change.path)?;
    }
  }
  Ok(
    changes
      .into_iter()
      .map(|change| RepositoryChange {
        action: change.action,
        path: change.display_path,
      })
      .collect(),
  )
}

fn plan_section(
  root: &Path,
  relative: &str,
  body: &str,
  changes: &mut Vec<PlannedChange>,
) -> Result<()> {
  let path = root.join(relative);
  let existing = read_optional(&path)?;
  match upsert_managed_section(existing.as_deref().unwrap_or_default(), body) {
    ManagedSectionResult::Unchanged(_) => {}
    ManagedSectionResult::Changed(content) => changes.push(PlannedChange {
      action: if existing.is_some() {
        "update"
      } else {
        "create"
      },
      path,
      display_path: native_relative(relative),
      content: Some(content),
      expected: existing,
    }),
    ManagedSectionResult::Conflict { reason, .. } => {
      bail!("Cannot update {relative}: {reason}")
    }
  }
  Ok(())
}

fn plan_section_removal(
  root: &Path,
  relative: &str,
  changes: &mut Vec<PlannedChange>,
) -> Result<()> {
  let path = root.join(relative);
  let Some(existing) = read_optional(&path)? else {
    return Ok(());
  };
  match remove_managed_section(&existing) {
    ManagedSectionResult::Unchanged(_) => {}
    ManagedSectionResult::Changed(content) => changes.push(PlannedChange {
      action: if content.is_empty() {
        "remove"
      } else {
        "update"
      },
      path,
      display_path: native_relative(relative),
      content: if content.is_empty() {
        None
      } else {
        Some(content)
      },
      expected: Some(existing),
    }),
    ManagedSectionResult::Conflict { reason, .. } => {
      bail!("Cannot remove {relative}: {reason}")
    }
  }
  Ok(())
}

fn plan_git_exclude(
  root: &Path,
  required: &[&str],
  changes: &mut Vec<PlannedChange>,
) -> Result<()> {
  let output = duct::cmd("git", ["rev-parse", "--git-path", "info/exclude"])
    .dir(root)
    .read()?;
  let path = {
    let candidate = PathBuf::from(output.trim());
    if candidate.is_absolute() {
      candidate
    } else {
      root.join(candidate)
    }
  };
  let existing = read_optional(&path)?;
  let current = existing.as_deref().unwrap_or_default();
  let lines: std::collections::BTreeSet<_> = current.lines().collect();
  let missing: Vec<_> = required
    .iter()
    .filter(|entry| !lines.contains(**entry))
    .collect();
  if !missing.is_empty() {
    let separator = if !current.is_empty() && !current.ends_with('\n') {
      "\n"
    } else {
      ""
    };
    let addition = missing
      .iter()
      .map(|entry| format!("{entry}\n"))
      .collect::<String>();
    changes.push(PlannedChange {
      action: if existing.is_some() {
        "update"
      } else {
        "create"
      },
      path,
      display_path: ".git/info/exclude".to_owned(),
      content: Some(format!("{current}{separator}{addition}")),
      expected: existing,
    });
  }
  Ok(())
}

fn enforce_private_local_policy(root: &Path) -> Result<()> {
  match local_tracking_policy(root)? {
    LocalTrackingPolicy::Private => Ok(()),
    LocalTrackingPolicy::RemoteTracked(reference) => bail!(
      "Configured default remote branch {reference} tracks .local; preserving that repository policy instead of applying Arcantry's private-local convention."
    ),
    LocalTrackingPolicy::IndexOnly => bail!(
      "The current Git index tracks .local while the configured default remote branch does not. Remove it from the index in a separate explicitly authorized operation before private adoption."
    ),
  }
}

fn local_tracking_policy(root: &Path) -> Result<LocalTrackingPolicy> {
  let default_remote = configured_default_remote_reference(root)?;
  if let Some(reference) = &default_remote {
    let tracked = git_read(
      root,
      &["ls-tree", "-r", "--name-only", reference, "--", ".local"],
    )?;
    if !tracked.trim().is_empty() {
      return Ok(LocalTrackingPolicy::RemoteTracked(reference.clone()));
    }
  }
  let indexed = git_read(root, &["ls-files", "--", ".local"])?;
  if !indexed.trim().is_empty() {
    return Ok(LocalTrackingPolicy::IndexOnly);
  }
  Ok(LocalTrackingPolicy::Private)
}

fn configured_default_remote_reference(root: &Path) -> Result<Option<String>> {
  let remotes = git_read(root, &["remote"])?;
  let mut remotes: Vec<_> = remotes
    .lines()
    .filter(|remote| !remote.is_empty())
    .collect();
  remotes.sort_by_key(|remote| if *remote == "origin" { 0 } else { 1 });
  for remote in remotes {
    let candidate = format!("refs/remotes/{remote}/HEAD");
    let reference = duct::cmd("git", ["symbolic-ref", "--quiet", &candidate])
      .dir(root)
      .stderr_null()
      .unchecked()
      .read()?;
    if !reference.trim().is_empty() {
      return Ok(Some(reference.trim().to_owned()));
    }
  }
  Ok(None)
}

fn git_read(root: &Path, arguments: &[&str]) -> Result<String> {
  duct::cmd("git", arguments)
    .dir(root)
    .read()
    .map_err(Into::into)
}

fn validate_section(
  root: &Path,
  relative: &str,
  body: &str,
  scope: Scope,
  doctor: bool,
  diagnostics: &mut Vec<RepositoryDiagnostic>,
) -> Result<()> {
  let content = read_optional(&root.join(relative))?;
  if content.as_deref().is_none_or(|content| {
    !matches!(
      upsert_managed_section(content, body),
      ManagedSectionResult::Unchanged(_)
    )
  }) {
    diagnostics.push(RepositoryDiagnostic {
      severity: "error",
      path: relative.to_owned(),
      message: "Arcantry managed section is missing or outdated.".to_owned(),
      repair: doctor.then(|| format!("Run `arcantry repo update --scope {}`.", scope.name())),
    });
  }
  Ok(())
}

fn validate_claude(
  root: &Path,
  scope: Scope,
  doctor: bool,
  diagnostics: &mut Vec<RepositoryDiagnostic>,
) -> Result<()> {
  let relative = claude_path(scope);
  let content = read_optional(&root.join(relative))?;
  if let Some(content) = content.filter(|content| contains_managed_section(content)) {
    let desired = format!("@{}", guidance_path(scope).replace('\\', "/"));
    if !matches!(
      upsert_managed_section(&content, &desired),
      ManagedSectionResult::Unchanged(_)
    ) {
      diagnostics.push(RepositoryDiagnostic {
        severity: "error",
        path: relative.to_owned(),
        message: "Arcantry managed Claude compatibility import is outdated.".to_owned(),
        repair: doctor.then(|| {
          format!(
            "Run `arcantry repo update --scope {} --compat claude`.",
            scope.name()
          )
        }),
      });
    }
  }
  Ok(())
}

fn validate_git_exclude(
  root: &Path,
  doctor: bool,
  diagnostics: &mut Vec<RepositoryDiagnostic>,
) -> Result<()> {
  match local_tracking_policy(root)? {
    LocalTrackingPolicy::RemoteTracked(reference) => {
      diagnostics.push(RepositoryDiagnostic {
        severity: "error",
        path: ".local/".to_owned(),
        message: format!(
          "Configured default remote branch {reference} tracks .local; this conflicts with Arcantry's private-local convention."
        ),
        repair: None,
      });
      return Ok(());
    }
    LocalTrackingPolicy::IndexOnly => {
      diagnostics.push(RepositoryDiagnostic {
        severity: "error",
        path: ".local/".to_owned(),
        message: "The current Git index tracks .local while the configured default remote branch does not. Remove it from the index only through a separate explicitly authorized operation.".to_owned(),
        repair: None,
      });
      return Ok(());
    }
    LocalTrackingPolicy::Private => {}
  }
  let output = duct::cmd("git", ["rev-parse", "--git-path", "info/exclude"])
    .dir(root)
    .read()?;
  let candidate = PathBuf::from(output.trim());
  let path = if candidate.is_absolute() {
    candidate
  } else {
    root.join(candidate)
  };
  let content = read_optional(&path)?.unwrap_or_default();
  let lines: std::collections::BTreeSet<_> = content.lines().collect();
  let private_claude = read_optional(&root.join(claude_path(Scope::Private)))?;
  let mut required = vec![".local/"];
  if private_claude.is_some_and(|content| contains_managed_section(&content)) {
    required.push("CLAUDE.local.md");
  }
  for entry in required {
    if !lines.contains(entry) {
      diagnostics.push(RepositoryDiagnostic {
        severity: "error",
        path: ".git/info/exclude".to_owned(),
        message: format!("{entry} must be excluded locally."),
        repair: doctor.then(|| {
          format!(
            "Run `arcantry repo update --scope private{}`.",
            if entry == "CLAUDE.local.md" {
              " --compat claude"
            } else {
              ""
            }
          )
        }),
      });
    }
  }
  Ok(())
}

fn config_path(scope: Scope) -> &'static str {
  if scope == Scope::Private {
    ".local/arcantry.toml"
  } else {
    "arcantry.toml"
  }
}
fn guidance_path(scope: Scope) -> &'static str {
  if scope == Scope::Private {
    ".local/AGENTS.md"
  } else {
    "AGENTS.md"
  }
}
fn claude_path(scope: Scope) -> &'static str {
  if scope == Scope::Private {
    "CLAUDE.local.md"
  } else {
    "CLAUDE.md"
  }
}
fn guidance(scope: Scope) -> &'static str {
  if scope == Scope::Private {
    PRIVATE_GUIDANCE
  } else {
    SHARED_GUIDANCE
  }
}
fn read_optional(path: &Path) -> Result<Option<String>> {
  match fs::read_to_string(path) {
    Ok(content) => Ok(Some(content)),
    Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
    Err(error) => Err(error.into()),
  }
}
fn display_path(root: &Path, path: &Path) -> String {
  path.strip_prefix(root).map_or_else(
    |_| path.display().to_string(),
    |value| value.to_string_lossy().replace('\\', "/"),
  )
}

fn native_relative(path: &str) -> String {
  path.replace('/', std::path::MAIN_SEPARATOR_STR)
}

#[cfg(test)]
mod tests {
  use super::*;

  fn git(root: &Path, arguments: &[&str]) {
    duct::cmd("git", arguments).dir(root).run().unwrap();
  }

  fn repository() -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    git(directory.path(), &["init", "--quiet"]);
    git(directory.path(), &["config", "user.name", "Arcantry Tests"]);
    git(
      directory.path(),
      &["config", "user.email", "tests@arcantry.invalid"],
    );
    directory
  }

  fn configure_remote_head(root: &Path) {
    git(
      root,
      &[
        "remote",
        "add",
        "origin",
        "https://example.invalid/repository.git",
      ],
    );
    git(root, &["update-ref", "refs/remotes/origin/master", "HEAD"]);
    git(
      root,
      &[
        "symbolic-ref",
        "refs/remotes/origin/HEAD",
        "refs/remotes/origin/master",
      ],
    );
  }

  #[test]
  fn detects_local_state_tracked_by_the_default_remote() {
    let directory = repository();
    let root = directory.path();
    fs::create_dir(root.join(".local")).unwrap();
    fs::write(root.join(".local/tracked.txt"), "tracked\n").unwrap();
    git(root, &["add", ".local/tracked.txt"]);
    git(
      root,
      &["commit", "--quiet", "-m", "test: track local state"],
    );
    configure_remote_head(root);

    assert!(matches!(
      local_tracking_policy(root).unwrap(),
      LocalTrackingPolicy::RemoteTracked(_)
    ));
    assert!(
      init(root, Scope::Private, false)
        .unwrap_err()
        .to_string()
        .contains("preserving that repository policy")
    );
    assert!(!root.join(".local/arcantry.toml").exists());
  }

  #[test]
  fn detects_local_state_tracked_only_by_the_index() {
    let directory = repository();
    let root = directory.path();
    fs::write(root.join("tracked.txt"), "tracked\n").unwrap();
    git(root, &["add", "tracked.txt"]);
    git(root, &["commit", "--quiet", "-m", "test: add baseline"]);
    configure_remote_head(root);
    fs::create_dir(root.join(".local")).unwrap();
    fs::write(root.join(".local/indexed.txt"), "indexed\n").unwrap();
    git(root, &["add", ".local/indexed.txt"]);

    assert_eq!(
      local_tracking_policy(root).unwrap(),
      LocalTrackingPolicy::IndexOnly
    );
    assert!(
      init(root, Scope::Private, false)
        .unwrap_err()
        .to_string()
        .contains("separate explicitly authorized operation")
    );
    assert!(!root.join(".local/arcantry.toml").exists());
  }
}
