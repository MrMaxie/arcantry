use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Action {
  Write,
  Delete,
  DeleteTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanOperation {
  pub action: Action,
  pub path: String,
  pub expected_hash: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub content_hash: Option<String>,
  pub visibility: crate::config::Visibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPlan {
  pub plan_version: u8,
  pub tool_version: String,
  pub root: PathBuf,
  pub source_id: String,
  pub transition: String,
  pub adapter: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub target_adapter: Option<String>,
  pub operations: Vec<PlanOperation>,
  pub notes: Vec<String>,
  pub conflicts: Vec<String>,
}

impl ProjectPlan {
  pub fn new(
    root: PathBuf,
    source_id: impl Into<String>,
    transition: impl Into<String>,
    adapter: impl Into<String>,
  ) -> Self {
    Self {
      plan_version: 1,
      tool_version: crate::VERSION.to_owned(),
      root,
      source_id: source_id.into(),
      transition: transition.into(),
      adapter: adapter.into(),
      target_adapter: None,
      operations: Vec::new(),
      notes: Vec::new(),
      conflicts: Vec::new(),
    }
  }
}

pub fn create_write_operation(
  root: &Path,
  path: &str,
  content: String,
  visibility: crate::config::Visibility,
) -> Result<PlanOperation> {
  Ok(PlanOperation {
    action: Action::Write,
    path: path.to_owned(),
    expected_hash: hash_path(&resolve_plan_path(root, path))?,
    content_hash: Some(hash_content(&content)),
    content: Some(content),
    visibility,
  })
}
pub fn create_delete_operation(
  root: &Path,
  path: &str,
  visibility: crate::config::Visibility,
) -> Result<PlanOperation> {
  Ok(PlanOperation {
    action: Action::Delete,
    path: path.to_owned(),
    expected_hash: hash_path(&resolve_plan_path(root, path))?,
    content: None,
    content_hash: None,
    visibility,
  })
}
pub fn create_delete_tree_operation(
  root: &Path,
  path: &str,
  visibility: crate::config::Visibility,
) -> Result<PlanOperation> {
  Ok(PlanOperation {
    action: Action::DeleteTree,
    path: path.to_owned(),
    expected_hash: hash_path(&resolve_plan_path(root, path))?,
    content: None,
    content_hash: None,
    visibility,
  })
}
pub fn serialize(plan: &ProjectPlan) -> Result<String> {
  Ok(format!("{}\n", serde_json::to_string_pretty(plan)?))
}
pub fn parse(content: &str) -> Result<ProjectPlan> {
  let plan: ProjectPlan = serde_json::from_str(content)?;
  validate(&plan)?;
  Ok(plan)
}

pub fn render(plan: &ProjectPlan) -> String {
  let mut lines = vec![
    format!("Transition: {}", plan.transition),
    format!("Source: {}", plan.source_id),
    format!(
      "Adapter: {}{}",
      plan.adapter,
      plan
        .target_adapter
        .as_ref()
        .map_or_else(String::new, |value| format!(" -> {value}"))
    ),
  ];
  for operation in &plan.operations {
    lines.push(format!(
      "{}: {}{}",
      action_name(&operation.action),
      operation.path,
      if operation.visibility == crate::config::Visibility::Private {
        " (private)"
      } else {
        ""
      }
    ));
  }
  for note in &plan.notes {
    lines.push(format!("Note: {note}"));
  }
  for conflict in &plan.conflicts {
    lines.push(format!("Conflict: {conflict}"));
  }
  if plan.operations.is_empty() && plan.conflicts.is_empty() {
    lines.push("No file changes.".to_owned());
  }
  format!("{}\n", lines.join("\n"))
}

pub fn apply(plan: &ProjectPlan) -> Result<Vec<PlanOperation>> {
  validate(plan)?;
  if plan.tool_version != crate::VERSION {
    bail!(
      "Plan requires Arcantry {}; current version is {}.",
      plan.tool_version,
      crate::VERSION
    );
  }
  if !plan.conflicts.is_empty() {
    bail!("Cannot apply plan: {}", plan.conflicts.join("; "));
  }
  for operation in &plan.operations {
    ensure_plan_path_boundary(&plan.root, &operation.path)?;
    if hash_path(&resolve_plan_path(&plan.root, &operation.path))? != operation.expected_hash {
      bail!(
        "Refusing to change {}; it changed after the plan was created.",
        operation.path
      );
    }
    if matches!(operation.action, Action::Write)
      && operation
        .content
        .as_ref()
        .map(|content| hash_content(content))
        != operation.content_hash
    {
      bail!(
        "Refusing to apply {}; planned content is corrupt.",
        operation.path
      );
    }
  }
  let preparation = tempfile::tempdir()?;
  let mut prepared = Vec::with_capacity(plan.operations.len());
  for (index, operation) in plan.operations.iter().enumerate() {
    if matches!(operation.action, Action::Write) {
      let path = preparation.path().join(format!("{index}.tmp"));
      fs::write(&path, operation.content.as_deref().unwrap_or_default())?;
      if hash_path(&path)? != operation.content_hash {
        bail!("Could not prepare {}.", operation.path);
      }
      prepared.push(Some(path));
    } else {
      prepared.push(None);
    }
  }

  struct Staged {
    operation: PlanOperation,
    target: PathBuf,
    staged: Option<PathBuf>,
    backup: Option<PathBuf>,
    committed: bool,
  }

  (|| -> Result<Vec<PlanOperation>> {
    let mut staged = Vec::with_capacity(plan.operations.len());
    for (index, operation) in plan.operations.iter().enumerate() {
      let target = resolve_plan_path(&plan.root, &operation.path);
      let parent = target.parent().unwrap_or(&plan.root);
      fs::create_dir_all(parent)?;
      let temporary = if matches!(operation.action, Action::Write) {
        let placeholder = tempfile::Builder::new()
          .prefix(".arcantry-")
          .suffix(".tmp")
          .tempfile_in(parent)?;
        let path = placeholder.path().to_path_buf();
        fs::copy(prepared[index].as_ref().unwrap(), &path)?;
        let (_file, persisted) = placeholder.keep()?;
        if hash_path(&persisted)? != operation.content_hash {
          bail!("Could not stage {}.", operation.path);
        }
        Some(persisted)
      } else {
        None
      };
      staged.push(Staged {
        operation: operation.clone(),
        target,
        staged: temporary,
        backup: None,
        committed: false,
      });
    }

    let commit = (|| -> Result<()> {
      for item in &mut staged {
        if hash_path(&item.target)? != item.operation.expected_hash {
          bail!(
            "Refusing to change {}; it changed during apply.",
            item.operation.path
          );
        }
        if item.operation.expected_hash.is_some() {
          let parent = item.target.parent().unwrap_or(&plan.root);
          let placeholder = tempfile::Builder::new()
            .prefix(".arcantry-")
            .suffix(".bak")
            .tempfile_in(parent)?;
          let backup = placeholder.path().to_path_buf();
          placeholder.close()?;
          fs::rename(&item.target, &backup)?;
          item.backup = Some(backup);
        }
        if matches!(item.operation.action, Action::Write) {
          fs::rename(item.staged.as_ref().unwrap(), &item.target)?;
          item.staged = None;
        }
        item.committed = true;
      }

      for item in &staged {
        let expected = if matches!(item.operation.action, Action::Write) {
          item.operation.content_hash.clone()
        } else {
          None
        };
        if hash_path(&item.target)? != expected {
          bail!("Verification failed for {}.", item.operation.path);
        }
      }
      Ok(())
    })();

    if let Err(error) = commit {
      for item in staged.iter().rev() {
        if let Some(path) = &item.staged {
          let _ = remove_any(path);
        }
        if let Some(backup) = &item.backup {
          let _ = remove_any(&item.target);
          let _ = fs::rename(backup, &item.target);
        } else if item.committed && matches!(item.operation.action, Action::Write) {
          let _ = remove_any(&item.target);
        }
      }
      return Err(error);
    }

    for item in &staged {
      if let Some(backup) = &item.backup {
        remove_any(backup)?;
      }
    }
    Ok(plan.operations.clone())
  })()
}

pub fn hash_content(content: &str) -> String {
  hex_digest(Sha256::digest(content.as_bytes()).as_slice())
}
pub fn hash_path(path: &Path) -> Result<Option<String>> {
  if !path.exists() {
    return Ok(None);
  }
  if path.is_file() {
    return Ok(Some(hex_digest(Sha256::digest(fs::read(path)?).as_slice())));
  }
  if !path.is_dir() {
    bail!("Unsupported plan input type: {}", path.display());
  }
  let mut digest = Sha256::new();
  let mut entries = fs::read_dir(path)?.collect::<std::result::Result<Vec<_>, _>>()?;
  entries.sort_by_key(|entry| entry.file_name());
  for entry in entries {
    let file_type = entry.file_type()?;
    digest.update(if file_type.is_dir() {
      b"directory\0".as_slice()
    } else {
      b"file\0".as_slice()
    });
    digest.update(entry.file_name().to_string_lossy().as_bytes());
    digest.update([0]);
    digest.update(
      hash_path(&entry.path())?
        .as_deref()
        .unwrap_or("missing")
        .as_bytes(),
    );
    digest.update([0]);
  }
  Ok(Some(hex_digest(digest.finalize().as_slice())))
}

fn remove_any(path: &Path) -> Result<()> {
  if !path.exists() {
    return Ok(());
  }
  if path.is_dir() {
    fs::remove_dir_all(path)?;
  } else {
    fs::remove_file(path)?;
  }
  Ok(())
}

fn validate(plan: &ProjectPlan) -> Result<()> {
  if plan.plan_version != 1 {
    bail!("planVersion must be 1.");
  }
  let mut paths = BTreeSet::new();
  for operation in &plan.operations {
    if relative_path_escapes(&operation.path) {
      bail!(
        "plan operation path must stay within the project: {}",
        operation.path
      );
    }
    if !paths.insert(&operation.path) {
      bail!("plan contains duplicate operation path: {}", operation.path);
    }
    if matches!(operation.action, Action::Write)
      && (operation.content.is_none() || operation.content_hash.is_none())
    {
      bail!("write operations require content and contentHash.");
    }
    if !matches!(operation.action, Action::Write)
      && (operation.content.is_some() || operation.content_hash.is_some())
    {
      bail!("delete operations cannot contain content.");
    }
  }
  Ok(())
}
fn resolve_plan_path(root: &Path, path: &str) -> PathBuf {
  let path = Path::new(path);
  if path.is_absolute() {
    path.to_path_buf()
  } else {
    root.join(path)
  }
}

fn relative_path_escapes(path: &str) -> bool {
  let portable = path.replace('\\', "/");
  let path = Path::new(&portable);
  !path.is_absolute()
    && matches!(
      crate::config::normalize_path_lexically(path)
        .components()
        .next(),
      Some(std::path::Component::ParentDir)
    )
}

fn ensure_plan_path_boundary(root: &Path, path: &str) -> Result<()> {
  if Path::new(path).is_absolute() {
    return Ok(());
  }
  let canonical_root = dunce::canonicalize(root)
    .with_context(|| format!("Could not resolve project root {}.", root.display()))?;
  let target = resolve_plan_path(root, path);
  let mut existing = target.as_path();
  while !existing.exists() {
    existing = existing
      .parent()
      .context("Plan operation has no existing project ancestor.")?;
  }
  let canonical_existing = dunce::canonicalize(existing)?;
  if !canonical_existing.starts_with(&canonical_root) {
    bail!("plan operation path resolves outside the project: {path}");
  }
  Ok(())
}
fn action_name(action: &Action) -> &'static str {
  match action {
    Action::Write => "write",
    Action::Delete => "delete",
    Action::DeleteTree => "delete-tree",
  }
}

fn hex_digest(bytes: &[u8]) -> String {
  bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rejects_relative_plan_paths_that_escape_the_project() {
    for path in [
      "../outside.txt",
      "nested/../../outside.txt",
      "..\\outside.txt",
    ] {
      assert!(relative_path_escapes(path));
    }
    assert!(!relative_path_escapes("nested/../inside.txt"));
  }
}
