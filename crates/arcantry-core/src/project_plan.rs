use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone)]
pub struct ApplyAuthority {
  root: PathBuf,
  allowed_external_paths: BTreeSet<PathBuf>,
}

impl ApplyAuthority {
  pub fn new(root: &Path) -> Result<Self> {
    Ok(Self {
      root: canonicalize_candidate(root)
        .with_context(|| format!("Could not resolve project root {}.", root.display()))?,
      allowed_external_paths: BTreeSet::new(),
    })
  }

  pub fn allow_exact(mut self, path: &Path) -> Result<Self> {
    self
      .allowed_external_paths
      .insert(canonicalize_candidate(path)?);
    Ok(self)
  }
}

#[derive(Debug, Clone)]
pub struct ApplyOutcome {
  pub operations: Vec<PlanOperation>,
  pub warnings: Vec<String>,
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

pub fn apply(plan: &ProjectPlan, authority: &ApplyAuthority) -> Result<ApplyOutcome> {
  apply_with_hooks(plan, authority, |_, _| Ok(()), remove_any)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TransactionPhase {
  Stage,
  Commit,
  Verify,
  Finalize,
}

struct Staged {
  operation: PlanOperation,
  target: PathBuf,
  staged: Option<PathBuf>,
  backup: Option<PathBuf>,
  committed: bool,
}

#[cfg(test)]
pub(crate) fn apply_with_checkpoint(
  plan: &ProjectPlan,
  authority: &ApplyAuthority,
  checkpoint: impl FnMut(TransactionPhase, usize) -> Result<()>,
) -> Result<ApplyOutcome> {
  apply_with_hooks(plan, authority, checkpoint, remove_any)
}

fn apply_with_hooks(
  plan: &ProjectPlan,
  authority: &ApplyAuthority,
  mut checkpoint: impl FnMut(TransactionPhase, usize) -> Result<()>,
  mut cleanup: impl FnMut(&Path) -> Result<()>,
) -> Result<ApplyOutcome> {
  validate(plan)?;
  ensure_plan_authority(plan, authority)?;
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

  let mut staged = Vec::with_capacity(plan.operations.len());
  let mut created_directories = Vec::new();
  let staging = (|| -> Result<()> {
    for (index, operation) in plan.operations.iter().enumerate() {
      checkpoint(TransactionPhase::Stage, index)?;
      let target = resolve_plan_path(&plan.root, &operation.path);
      let parent = target.parent().unwrap_or(&plan.root);
      if matches!(operation.action, Action::Write) {
        create_parent_directories(parent, &mut created_directories)?;
      }
      let temporary = if matches!(operation.action, Action::Write) {
        let placeholder = tempfile::Builder::new()
          .prefix(".arcantry-")
          .suffix(".tmp")
          .tempfile_in(parent)?;
        let path = placeholder.path().to_path_buf();
        let prepared_path = prepared[index]
          .as_ref()
          .context("Prepared write operation is missing its staged content.")?;
        fs::copy(prepared_path, &path)?;
        preserve_permissions(&target, &path)?;
        let (_file, persisted) = placeholder.keep()?;
        match hash_path(&persisted) {
          Ok(hash) if hash == operation.content_hash => {}
          Ok(_) => {
            remove_any(&persisted)?;
            bail!("Could not stage {}.", operation.path);
          }
          Err(error) => {
            let _ = remove_any(&persisted);
            return Err(error);
          }
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
    Ok(())
  })();
  if let Err(error) = staging {
    rollback_staged(&staged, &created_directories).with_context(|| {
      format!("Transaction staging failed and could not be rolled back: {error:#}")
    })?;
    return Err(error);
  }

  let commit = (|| -> Result<()> {
    for (index, item) in staged.iter_mut().enumerate() {
      checkpoint(TransactionPhase::Commit, index)?;
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
        let staged_path = item
          .staged
          .as_ref()
          .context("Staged write operation is missing its temporary file.")?;
        fs::rename(staged_path, &item.target)?;
        item.staged = None;
      }
      item.committed = true;
      let expected = if matches!(item.operation.action, Action::Write) {
        item.operation.content_hash.clone()
      } else {
        None
      };
      checkpoint(TransactionPhase::Verify, index)?;
      if hash_path(&item.target)? != expected {
        bail!("Verification failed for {}.", item.operation.path);
      }
    }
    Ok(())
  })();

  if let Err(error) = commit {
    rollback_staged(&staged, &created_directories).with_context(|| {
      format!("Transaction commit failed and could not be rolled back: {error:#}")
    })?;
    return Err(error);
  }

  for index in 0..staged.len() {
    if let Err(error) = checkpoint(TransactionPhase::Finalize, index) {
      rollback_staged(&staged, &created_directories).with_context(|| {
        format!("Transaction finalization failed and could not be rolled back: {error:#}")
      })?;
      return Err(error);
    }
  }

  let mut warnings = Vec::new();
  for item in &staged {
    if let Some(backup) = &item.backup
      && let Err(error) = cleanup(backup)
    {
      warnings.push(format!(
        "Applied {} but could not remove transaction backup {}: {error}",
        item.operation.path,
        backup.display()
      ));
    }
  }
  Ok(ApplyOutcome {
    operations: plan.operations.clone(),
    warnings,
  })
}

fn create_parent_directories(parent: &Path, created: &mut Vec<PathBuf>) -> Result<()> {
  let mut missing = parent
    .ancestors()
    .take_while(|path| !path.exists())
    .map(Path::to_path_buf)
    .collect::<Vec<_>>();
  missing.reverse();
  let result = fs::create_dir_all(parent);
  created.extend(missing.into_iter().filter(|path| path.is_dir()));
  result?;
  Ok(())
}

#[cfg(unix)]
fn preserve_permissions(source: &Path, target: &Path) -> Result<()> {
  if let Ok(metadata) = fs::metadata(source) {
    fs::set_permissions(target, metadata.permissions())?;
  }
  Ok(())
}

#[cfg(not(unix))]
fn preserve_permissions(_source: &Path, _target: &Path) -> Result<()> {
  Ok(())
}

fn rollback_staged(staged: &[Staged], created_directories: &[PathBuf]) -> Result<()> {
  for item in staged.iter().rev() {
    if let Some(path) = &item.staged {
      remove_any(path)?;
    }
    if let Some(backup) = &item.backup {
      remove_any(&item.target)?;
      fs::rename(backup, &item.target)?;
    } else if item.committed && matches!(item.operation.action, Action::Write) {
      remove_any(&item.target)?;
    }
  }
  for directory in created_directories.iter().rev() {
    match fs::remove_dir(directory) {
      Ok(()) => {}
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
      Err(error) => return Err(error.into()),
    }
  }
  Ok(())
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

fn ensure_plan_authority(plan: &ProjectPlan, authority: &ApplyAuthority) -> Result<()> {
  let plan_root = canonicalize_candidate(&plan.root)?;
  if plan_root != authority.root {
    bail!(
      "Plan root {} does not match the current project {}.",
      plan.root.display(),
      authority.root.display()
    );
  }
  for operation in &plan.operations {
    let target = canonicalize_candidate(&resolve_plan_path(&plan.root, &operation.path))?;
    if target.starts_with(&authority.root) {
      continue;
    }
    if !authority.allowed_external_paths.contains(&target) {
      bail!(
        "Plan operation path requires an exact --allow-outside authorization: {}",
        operation.path
      );
    }
  }
  Ok(())
}

fn canonicalize_candidate(path: &Path) -> Result<PathBuf> {
  let absolute = if path.is_absolute() {
    path.to_path_buf()
  } else {
    std::env::current_dir()?.join(path)
  };
  let normalized = crate::config::normalize_path_lexically(&absolute);
  let mut existing = normalized.as_path();
  let mut suffix = Vec::new();
  while fs::symlink_metadata(existing).is_err() {
    let name = existing
      .file_name()
      .context("Path has no existing ancestor.")?;
    suffix.push(name.to_os_string());
    existing = existing
      .parent()
      .context("Path has no existing ancestor.")?;
  }
  let mut resolved = dunce::canonicalize(existing)?;
  for component in suffix.iter().rev() {
    resolved.push(component);
  }
  Ok(crate::config::normalize_path_lexically(&resolved))
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
  use proptest::prelude::*;

  #[derive(Debug, PartialEq, Eq)]
  struct TreeEntry {
    path: String,
    kind: &'static str,
    content: Vec<u8>,
  }

  fn snapshot(root: &Path) -> Vec<TreeEntry> {
    fn visit(root: &Path, directory: &Path, entries: &mut Vec<TreeEntry>) {
      let mut children = fs::read_dir(directory)
        .unwrap()
        .collect::<std::io::Result<Vec<_>>>()
        .unwrap();
      children.sort_by_key(|entry| entry.file_name());
      for child in children {
        let path = child.path();
        let relative = path
          .strip_prefix(root)
          .unwrap()
          .to_string_lossy()
          .replace('\\', "/");
        let file_type = child.file_type().unwrap();
        if file_type.is_dir() {
          entries.push(TreeEntry {
            path: relative,
            kind: "directory",
            content: Vec::new(),
          });
          visit(root, &path, entries);
        } else {
          entries.push(TreeEntry {
            path: relative,
            kind: "file",
            content: fs::read(path).unwrap(),
          });
        }
      }
    }

    let mut entries = Vec::new();
    visit(root, root, &mut entries);
    entries
  }

  fn fixture_plan(actions: &[u8]) -> (tempfile::TempDir, ProjectPlan) {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let mut plan = ProjectPlan::new(root.to_path_buf(), "test", "adopt", "test@1");
    for (index, action) in actions.iter().enumerate() {
      match action % 3 {
        0 => {
          let path = format!("write-{index}/value.txt");
          if index % 2 == 0 {
            fs::create_dir_all(root.join(format!("write-{index}"))).unwrap();
            fs::write(root.join(&path), format!("before-{index}\n")).unwrap();
          }
          plan.operations.push(
            create_write_operation(
              root,
              &path,
              format!("after-{index}\n"),
              crate::config::Visibility::Shared,
            )
            .unwrap(),
          );
        }
        1 => {
          let path = format!("delete-{index}.txt");
          fs::write(root.join(&path), format!("remove-{index}\n")).unwrap();
          plan
            .operations
            .push(create_delete_operation(root, &path, crate::config::Visibility::Shared).unwrap());
        }
        _ => {
          let path = format!("tree-{index}");
          fs::create_dir_all(root.join(&path).join("nested")).unwrap();
          fs::write(
            root.join(&path).join("nested/value.txt"),
            format!("remove-tree-{index}\n"),
          )
          .unwrap();
          plan.operations.push(
            create_delete_tree_operation(root, &path, crate::config::Visibility::Shared).unwrap(),
          );
        }
      }
    }
    (directory, plan)
  }

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

  #[test]
  fn apply_requires_the_current_project_root() {
    let (directory, plan) = fixture_plan(&[0]);
    let other = tempfile::tempdir().unwrap();
    let authority = ApplyAuthority::new(other.path()).unwrap();

    let error = apply(&plan, &authority).unwrap_err();

    assert!(
      error
        .to_string()
        .contains("does not match the current project")
    );
    assert_eq!(
      fs::read_to_string(directory.path().join("write-0/value.txt")).unwrap(),
      "before-0\n"
    );
  }

  #[cfg(unix)]
  #[test]
  fn apply_preserves_existing_file_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let path = root.join("script.sh");
    fs::write(&path, "before\n").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    let mut plan = ProjectPlan::new(root.to_path_buf(), "test", "update", "test@1");
    plan.operations.push(
      create_write_operation(
        root,
        "script.sh",
        "after\n".to_owned(),
        crate::config::Visibility::Shared,
      )
      .unwrap(),
    );

    apply(&plan, &ApplyAuthority::new(root).unwrap()).unwrap();

    assert_eq!(
      fs::metadata(path).unwrap().permissions().mode() & 0o777,
      0o755
    );
  }

  #[test]
  fn external_authority_is_exact_and_does_not_cover_related_paths() {
    let project = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let exact = outside.path().join("exact.txt");
    let mut plan = ProjectPlan::new(project.path().to_path_buf(), "test", "adopt", "test@1");
    plan.operations.push(
      create_write_operation(
        project.path(),
        &exact.to_string_lossy(),
        "allowed\n".to_owned(),
        crate::config::Visibility::Private,
      )
      .unwrap(),
    );
    let authority = ApplyAuthority::new(project.path())
      .unwrap()
      .allow_exact(&exact)
      .unwrap();
    apply(&plan, &authority).unwrap();
    assert_eq!(fs::read_to_string(&exact).unwrap(), "allowed\n");

    for rejected in [
      outside.path().join("sibling.txt"),
      outside.path().to_path_buf(),
      exact.join("child.txt"),
    ] {
      let mut rejected_plan =
        ProjectPlan::new(project.path().to_path_buf(), "test", "adopt", "test@1");
      rejected_plan.operations.push(
        create_write_operation(
          project.path(),
          &rejected.to_string_lossy(),
          "rejected\n".to_owned(),
          crate::config::Visibility::Private,
        )
        .unwrap(),
      );
      assert!(
        apply(&rejected_plan, &authority)
          .unwrap_err()
          .to_string()
          .contains("exact --allow-outside")
      );
    }
  }

  #[test]
  fn project_relative_path_cannot_escape_through_a_directory_link() {
    let project = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let linked = project.path().join("linked");
    #[cfg(windows)]
    junction::create(outside.path(), &linked).unwrap();
    #[cfg(not(windows))]
    std::os::unix::fs::symlink(outside.path(), &linked).unwrap();
    let mut plan = ProjectPlan::new(project.path().to_path_buf(), "test", "adopt", "test@1");
    plan.operations.push(
      create_write_operation(
        project.path(),
        "linked/escape.txt",
        "rejected\n".to_owned(),
        crate::config::Visibility::Shared,
      )
      .unwrap(),
    );

    assert!(
      apply(&plan, &ApplyAuthority::new(project.path()).unwrap())
        .unwrap_err()
        .to_string()
        .contains("exact --allow-outside")
    );
    assert!(!outside.path().join("escape.txt").exists());
  }

  #[test]
  fn cleanup_failure_after_commit_returns_success_with_a_warning() {
    let (directory, plan) = fixture_plan(&[0]);
    let authority = ApplyAuthority::new(directory.path()).unwrap();

    let outcome = apply_with_hooks(
      &plan,
      &authority,
      |_, _| Ok(()),
      |_| bail!("injected cleanup failure"),
    )
    .unwrap();

    assert_eq!(outcome.operations.len(), 1);
    assert_eq!(outcome.warnings.len(), 1);
    assert!(outcome.warnings[0].contains("injected cleanup failure"));
    assert_eq!(
      fs::read_to_string(directory.path().join("write-0/value.txt")).unwrap(),
      "after-0\n"
    );
  }

  #[test]
  fn every_staging_commit_and_verification_failure_restores_the_tree() {
    let actions = [0, 0, 1, 2];
    for phase in [
      TransactionPhase::Stage,
      TransactionPhase::Commit,
      TransactionPhase::Verify,
      TransactionPhase::Finalize,
    ] {
      for failed_index in 0..actions.len() {
        let (directory, plan) = fixture_plan(&actions);
        let authority = ApplyAuthority::new(directory.path()).unwrap();
        let before = snapshot(directory.path());
        let result = apply_with_checkpoint(&plan, &authority, |current_phase, current_index| {
          if current_phase == phase && current_index == failed_index {
            bail!("injected {phase:?} failure at {failed_index}");
          }
          Ok(())
        });
        assert!(result.is_err());
        assert_eq!(snapshot(directory.path()), before);
      }
    }
  }

  proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn generated_operation_sequences_rollback_at_every_selected_checkpoint(
      actions in prop::collection::vec(0_u8..3, 1..8),
      failure_seed in any::<usize>(),
    ) {
      let (directory, plan) = fixture_plan(&actions);
      let authority = ApplyAuthority::new(directory.path()).unwrap();
      let before = snapshot(directory.path());
      let failure = failure_seed % (actions.len() * 4);
      let mut checkpoint = 0;
      let result = apply_with_checkpoint(&plan, &authority, |_, _| {
        let current = checkpoint;
        checkpoint += 1;
        if current == failure {
          bail!("injected transaction failure at {failure}");
        }
        Ok(())
      });
      prop_assert!(result.is_err());
      prop_assert_eq!(snapshot(directory.path()), before);
    }
  }
}
