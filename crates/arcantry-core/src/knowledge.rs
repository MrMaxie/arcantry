use crate::config::{
  Management, RawSourceConfig, ResolvedProject, SourceKind, Visibility, effective_visibility,
  normalize_path_lexically,
};
use crate::repository::{LocalBoundaryInspection, inspect_local_boundary};
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSource {
  pub id: String,
  pub kind: SourceKind,
  pub path: String,
  pub management: Management,
  pub adapter: String,
  pub from: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub managed_from: Option<String>,
  pub visibility: Visibility,
  pub scope: String,
  pub absolute_path: PathBuf,
  pub exists: bool,
  pub origin: &'static str,
  pub confidence: &'static str,
  pub adapter_status: &'static str,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeInspection {
  pub schema_version: u32,
  pub root: PathBuf,
  pub mode: &'static str,
  pub config_path: Option<PathBuf>,
  pub config_scope: Option<&'static str>,
  pub shadowed_config_paths: Vec<PathBuf>,
  pub sources: Vec<ProjectSource>,
  pub methodologies: Vec<MethodologyInspection>,
  pub local_boundary: LocalBoundaryInspection,
  pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MethodologyInspection {
  pub id: &'static str,
  pub active: bool,
  pub evidence: Vec<String>,
}

pub fn inspect(project: &ResolvedProject) -> Result<KnowledgeInspection> {
  let mut sources = Vec::new();
  let mut diagnostics = Vec::new();
  let mut configured_paths = BTreeSet::new();
  if let Some(config) = &project.config {
    for (id, source) in &config.sources {
      let absolute_path = resolve_source(&project.root, &source.path);
      configured_paths.insert(normalize(&absolute_path));
      let item = finalize(
        id,
        source,
        absolute_path.clone(),
        absolute_path.exists(),
        "configured",
        "high",
      );
      append_diagnostic(&item, &mut diagnostics);
      sources.push(item);
    }
  }
  for (id, kind, path, visibility, directory, default_adapter) in [
    (
      "openspec",
      SourceKind::Openspec,
      "openspec",
      Visibility::Shared,
      true,
      "openspec@1",
    ),
    (
      "openspec-local",
      SourceKind::Openspec,
      ".local/openspec",
      Visibility::Private,
      true,
      "openspec@1",
    ),
    (
      "changelog",
      SourceKind::Changelog,
      "CHANGELOG.md",
      Visibility::Shared,
      false,
      "keep-a-changelog@2",
    ),
    (
      "changelog-local",
      SourceKind::Changelog,
      ".local/CHANGELOG.md",
      Visibility::Private,
      false,
      "keep-a-changelog@2",
    ),
    (
      "todo-root",
      SourceKind::TodoTxt,
      "todo.txt",
      Visibility::Shared,
      false,
      "todo-txt@1",
    ),
    (
      "todo-local",
      SourceKind::TodoTxt,
      ".local/todo.txt",
      Visibility::Private,
      false,
      "todo-txt@1",
    ),
  ] {
    let absolute = project.root.join(path);
    if configured_paths.contains(&normalize(&absolute)) {
      continue;
    }
    let exists = if directory {
      absolute.is_dir()
    } else {
      absolute.is_file()
    };
    let (adapter, confidence) = if exists {
      detect_adapter(&kind, &absolute)?
    } else {
      (default_adapter.to_owned(), "none")
    };
    let source = RawSourceConfig {
      kind,
      path: path.to_owned(),
      management: Management::Observe,
      adapter,
      from: Vec::new(),
      managed_from: None,
      visibility: Some(visibility),
      scope: ".".to_owned(),
    };
    let unique = unique_id(id, &sources);
    let item = finalize(
      &unique,
      &source,
      absolute,
      exists,
      if exists { "discovered" } else { "standard" },
      confidence,
    );
    append_diagnostic(&item, &mut diagnostics);
    sources.push(item);
  }
  sources.sort_by(|left, right| left.id.cmp(&right.id));
  let methodologies = inspect_methodologies(&project.root, &sources);
  Ok(KnowledgeInspection {
    schema_version: 1,
    root: project.root.clone(),
    mode: project.mode,
    config_path: project.config_path.clone(),
    config_scope: project.scope,
    shadowed_config_paths: project.shadowed_config_paths.clone(),
    sources,
    methodologies,
    local_boundary: inspect_local_boundary(&project.root)?,
    diagnostics,
  })
}

fn inspect_methodologies(root: &Path, sources: &[ProjectSource]) -> Vec<MethodologyInspection> {
  let source_methodology = |id, kind| MethodologyInspection {
    id,
    active: sources
      .iter()
      .any(|source| source.kind == kind && source.exists),
    evidence: sources
      .iter()
      .filter(|source| source.kind == kind && source.exists)
      .map(|source| source.path.clone())
      .collect(),
  };
  let marker_methodology = |id, paths: &[&str]| MethodologyInspection {
    id,
    active: paths.iter().any(|path| root.join(path).exists()),
    evidence: paths
      .iter()
      .filter(|path| root.join(path).exists())
      .map(|path| (*path).to_owned())
      .collect(),
  };
  vec![
    source_methodology("openspec", SourceKind::Openspec),
    source_methodology("todo-txt", SourceKind::TodoTxt),
    source_methodology("keep-a-changelog", SourceKind::Changelog),
    marker_methodology("agent-guidance", &["AGENTS.md", ".local/AGENTS.md"]),
    marker_methodology(
      "agent-skills",
      &["skills", ".agents/skills", ".local/skills"],
    ),
  ]
}

pub fn adapter_status(kind: &SourceKind, adapter: &str) -> &'static str {
  match (kind, adapter) {
    (SourceKind::Openspec, "openspec@1")
    | (SourceKind::Changelog, "keep-a-changelog@1" | "keep-a-changelog@2")
    | (SourceKind::TodoTxt, "todo-txt@1") => "supported",
    (_, "openspec@1" | "keep-a-changelog@1" | "keep-a-changelog@2" | "todo-txt@1") => "wrong-kind",
    _ => "unsupported",
  }
}

fn finalize(
  id: &str,
  source: &RawSourceConfig,
  absolute_path: PathBuf,
  exists: bool,
  origin: &'static str,
  confidence: &'static str,
) -> ProjectSource {
  ProjectSource {
    id: id.to_owned(),
    kind: source.kind.clone(),
    path: source.path.clone(),
    management: source.management.clone(),
    adapter: source.adapter.clone(),
    from: source.from.clone(),
    managed_from: source.managed_from.clone(),
    visibility: effective_visibility(source),
    scope: source.scope.clone(),
    absolute_path,
    exists,
    origin,
    confidence,
    adapter_status: adapter_status(&source.kind, &source.adapter),
  }
}

fn append_diagnostic(source: &ProjectSource, diagnostics: &mut Vec<String>) {
  if source.adapter_status == "unsupported" {
    diagnostics.push(format!(
      "Source {} requires unsupported adapter {}.",
      source.id, source.adapter
    ));
  }
  if source.adapter_status == "wrong-kind" {
    diagnostics.push(format!(
      "Adapter {} cannot handle {:?} source {}.",
      source.adapter, source.kind, source.id
    ));
  }
  if !source.exists && !matches!(source.management, Management::Ignore | Management::Observe) {
    diagnostics.push(format!(
      "Configured {:?} source {} is missing at {}.",
      source.management, source.id, source.path
    ));
  }
}

fn detect_adapter(kind: &SourceKind, path: &Path) -> Result<(String, &'static str)> {
  match kind {
    SourceKind::Openspec => Ok(("openspec@1".to_owned(), "high")),
    SourceKind::TodoTxt => Ok(("todo-txt@1".to_owned(), "high")),
    SourceKind::Changelog => {
      let content = fs::read_to_string(path)?
        .trim_start_matches('\u{feff}')
        .to_owned();
      if content.contains("keepachangelog.com/en/2.0.0") {
        Ok(("keep-a-changelog@2".to_owned(), "high"))
      } else if content.contains("keepachangelog.com/en/1.") {
        Ok(("keep-a-changelog@1".to_owned(), "high"))
      } else if content.lines().any(|line| {
        line.starts_with("## [Unreleased]")
          || line.strip_prefix("## [").is_some_and(|rest| {
            rest
              .chars()
              .next()
              .is_some_and(|value| value.is_ascii_digit())
          })
      }) {
        Ok(("keep-a-changelog@1".to_owned(), "medium"))
      } else {
        Ok(("changelog@0".to_owned(), "low"))
      }
    }
  }
}

fn unique_id(candidate: &str, sources: &[ProjectSource]) -> String {
  if !sources.iter().any(|source| source.id == candidate) {
    return candidate.to_owned();
  }
  (2..)
    .map(|suffix| format!("{candidate}-{suffix}"))
    .find(|id| !sources.iter().any(|source| source.id == *id))
    .unwrap()
}
fn resolve_source(root: &Path, path: &str) -> PathBuf {
  let path = Path::new(path);
  if path.is_absolute() {
    path.to_path_buf()
  } else {
    root.join(path)
  }
}
fn normalize(path: &Path) -> String {
  let value = normalize_path_lexically(path)
    .to_string_lossy()
    .into_owned();
  if cfg!(windows) {
    value.to_lowercase()
  } else {
    value
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reports_absent_standard_sources_and_bounded_methodologies() {
    let directory = tempfile::tempdir().unwrap();
    fs::write(directory.path().join("AGENTS.md"), "# Guidance\n").unwrap();
    let project = ResolvedProject {
      root: directory.path().to_path_buf(),
      config_path: None,
      config: None,
      mode: "wild",
      scope: None,
      shadowed_config_paths: Vec::new(),
    };

    let inspection = inspect(&project).unwrap();

    assert_eq!(inspection.schema_version, 1);
    assert_eq!(inspection.sources.len(), 6);
    assert!(inspection.sources.iter().all(|source| !source.exists));
    assert!(
      inspection
        .methodologies
        .iter()
        .any(|item| item.id == "agent-guidance" && item.active)
    );
    assert_eq!(inspection.local_boundary.status, "not-git");
  }

  #[test]
  fn reports_present_sources_without_enumerating_unrelated_content() {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir(directory.path().join("openspec")).unwrap();
    fs::write(directory.path().join("todo.txt"), "Inspect context\n").unwrap();
    fs::write(directory.path().join("unrelated.txt"), "private detail\n").unwrap();
    let project = ResolvedProject {
      root: directory.path().to_path_buf(),
      config_path: None,
      config: None,
      mode: "wild",
      scope: None,
      shadowed_config_paths: Vec::new(),
    };

    let inspection = inspect(&project).unwrap();

    assert_eq!(
      inspection
        .sources
        .iter()
        .filter(|source| source.exists)
        .count(),
      2
    );
    assert!(
      inspection
        .sources
        .iter()
        .all(|source| source.path != "unrelated.txt")
    );
  }
}
