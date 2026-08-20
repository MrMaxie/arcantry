use anyhow::Result;
use arcantry_core::config::{Management, SourceKind, Visibility, is_private_project_path};
use arcantry_core::knowledge::{KnowledgeInspection, ProjectSource};
use arcantry_core::project_plan::{
  PlanOperation, create_delete_operation, create_delete_tree_operation, create_write_operation,
  hash_content, hash_path,
};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(super) fn standard_missing_source(root: &Path, id: &str) -> Option<ProjectSource> {
  let (kind, path, adapter, visibility) = match id {
    "openspec" => (
      SourceKind::Openspec,
      "openspec",
      "openspec@1",
      Visibility::Shared,
    ),
    "changelog" => (
      SourceKind::Changelog,
      "CHANGELOG.md",
      "keep-a-changelog@2",
      Visibility::Shared,
    ),
    "todo-root" => (
      SourceKind::TodoTxt,
      "todo.txt",
      "todo-txt@1",
      Visibility::Shared,
    ),
    "todo-local" => (
      SourceKind::TodoTxt,
      ".local/todo.txt",
      "todo-txt@1",
      Visibility::Private,
    ),
    _ => return None,
  };
  Some(ProjectSource {
    id: id.to_owned(),
    kind,
    path: path.to_owned(),
    management: Management::Observe,
    adapter: adapter.to_owned(),
    from: Vec::new(),
    managed_from: None,
    visibility,
    scope: ".".to_owned(),
    absolute_path: root.join(path),
    exists: false,
    origin: "discovered",
    confidence: "high",
    adapter_status: "supported",
  })
}

pub(super) fn desired_adapter_for(
  source: &ProjectSource,
  fallback: &str,
  current: String,
) -> String {
  if current == source.adapter {
    fallback.to_owned()
  } else {
    current
  }
}

pub(super) fn plan_source_initialization(
  inspection: &KnowledgeInspection,
  source: &ProjectSource,
  operations: &mut Vec<PlanOperation>,
) -> Result<()> {
  match source.kind {
    SourceKind::Openspec => {
      let source_root = resolve_source_path(&inspection.root, &source.path);
      operations.push(create_write_operation(
        &inspection.root,
        &plan_path(&inspection.root, &source_root.join("config.yaml")),
        "# arcantry:generated\nschema: arcantry\n\ncontext: |\n  OpenSpec is the only source of product and engineering specifications in this project.\n"
          .to_owned(),
        source.visibility,
      )?);
      for (path, content) in crate::embedded::openspec_asset_files()? {
        operations.push(create_write_operation(
          &inspection.root,
          &plan_path(&inspection.root, &source_root.join(path)),
          content,
          source.visibility,
        )?);
      }
    }
    SourceKind::Changelog => operations.push(create_write_operation(
      &inspection.root,
      &source.path,
      arcantry_core::changelog::render_empty(),
      source.visibility,
    )?),
    SourceKind::TodoTxt => operations.push(create_write_operation(
      &inspection.root,
      &source.path,
      String::new(),
      source.visibility,
    )?),
  }
  Ok(())
}

pub(super) fn plan_file_relocation(
  inspection: &KnowledgeInspection,
  source: &ProjectSource,
  target: &Path,
  delete_source: bool,
  operations: &mut Vec<PlanOperation>,
  conflicts: &mut Vec<String>,
) -> Result<()> {
  let content = fs::read_to_string(&source.absolute_path)?;
  let target_hash = hash_path(target)?;
  let desired_hash = hash_content(&content);
  if target_hash
    .as_deref()
    .is_some_and(|value| value != desired_hash)
  {
    conflicts.push(format!(
      "Relocate target already contains different content: {}.",
      plan_path(&inspection.root, target)
    ));
    return Ok(());
  }
  if target_hash.is_none() {
    operations.push(create_write_operation(
      &inspection.root,
      &plan_path(&inspection.root, target),
      content,
      source.visibility,
    )?);
  }
  if delete_source {
    operations.push(create_delete_operation(
      &inspection.root,
      &source.path,
      source.visibility,
    )?);
  }
  Ok(())
}

pub(super) fn plan_directory_relocation(
  inspection: &KnowledgeInspection,
  source: &ProjectSource,
  target: &Path,
  delete_source: bool,
  operations: &mut Vec<PlanOperation>,
  conflicts: &mut Vec<String>,
) -> Result<()> {
  let mut files = WalkDir::new(&source.absolute_path)
    .min_depth(1)
    .into_iter()
    .collect::<std::result::Result<Vec<_>, _>>()?;
  files.retain(|entry| entry.file_type().is_file());
  files.sort_by_key(|entry| entry.path().to_path_buf());
  for entry in files {
    let child = entry.path().strip_prefix(&source.absolute_path)?;
    let target_file = target.join(child);
    let content = fs::read_to_string(entry.path())?;
    let target_hash = hash_path(&target_file)?;
    let desired_hash = hash_content(&content);
    if target_hash
      .as_deref()
      .is_some_and(|value| value != desired_hash)
    {
      conflicts.push(format!(
        "Relocate target already contains different content: {}.",
        plan_path(&inspection.root, &target_file)
      ));
    } else if target_hash.is_none() {
      operations.push(create_write_operation(
        &inspection.root,
        &plan_path(&inspection.root, &target_file),
        content,
        source.visibility,
      )?);
    }
  }
  if delete_source && conflicts.is_empty() {
    operations.push(create_delete_tree_operation(
      &inspection.root,
      &source.path,
      source.visibility,
    )?);
  }
  Ok(())
}

pub(super) fn source_path_exists(root: &Path, path: &str, kind: &SourceKind) -> bool {
  let target = resolve_source_path(root, path);
  if *kind == SourceKind::Openspec {
    target.is_dir()
  } else {
    target.is_file()
  }
}

pub(super) fn resolve_source_path(root: &Path, path: &str) -> PathBuf {
  let path = Path::new(path);
  if path.is_absolute() {
    path.to_path_buf()
  } else {
    root.join(path)
  }
}

pub(super) fn plan_path(root: &Path, path: &Path) -> String {
  path.strip_prefix(root).map_or_else(
    |_| path.to_string_lossy().to_string(),
    |relative| relative.to_string_lossy().replace('\\', "/"),
  )
}

pub(super) fn config_visibility(root: &Path, path: &Path) -> Visibility {
  let relative = plan_path(root, path);
  if !is_within(root, path) || is_private_project_path(&relative) {
    Visibility::Private
  } else {
    Visibility::Shared
  }
}

pub(super) fn is_local_plan_path(root: &Path, path: &str) -> bool {
  let value = plan_path(root, &resolve_source_path(root, path));
  is_private_project_path(&value)
}

pub(super) fn is_within(parent: &Path, child: &Path) -> bool {
  let parent = dunce::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf());
  child.starts_with(parent)
}

pub(super) fn same_path(left: &Path, right: &Path) -> bool {
  let normalize = |path: &Path| path.to_string_lossy().replace('\\', "/").to_lowercase();
  normalize(left) == normalize(right)
}
