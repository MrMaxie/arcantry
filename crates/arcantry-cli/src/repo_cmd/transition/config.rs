use anyhow::{Result, bail};
use arcantry_core::config::{
  Management, PROJECT_CONFIG_SCHEMA_LOCATION, RawSourceConfig, SchemaReference, Visibility,
  is_private_project_path, parse_project_config, render_project_config,
};
use arcantry_core::knowledge::ProjectSource;

pub(super) struct SourceUpdate {
  pub path: String,
  pub management: Management,
  pub adapter: String,
  pub from: Vec<String>,
  pub managed_from: Option<String>,
}

pub(super) fn add_configured_source(
  content: &str,
  source: &ProjectSource,
  update: SourceUpdate,
  allow_absolute_paths: bool,
) -> Result<String> {
  let mut config =
    parse_project_config(content, Some(arcantry_core::VERSION), allow_absolute_paths)?;
  if config.sources.contains_key(&source.id) {
    bail!("Configured source already exists: {}.", source.id);
  }
  if config.schema_reference.is_none() {
    config.schema_reference = Some(SchemaReference {
      location: PROJECT_CONFIG_SCHEMA_LOCATION.to_owned(),
      version: Some(arcantry_core::VERSION.to_owned()),
    });
  }
  let default_visibility = if is_private_project_path(&update.path) {
    Visibility::Private
  } else {
    Visibility::Shared
  };
  config.sources.insert(
    source.id.clone(),
    RawSourceConfig {
      kind: source.kind.clone(),
      path: update.path,
      management: update.management,
      adapter: update.adapter,
      from: update.from,
      managed_from: update.managed_from,
      visibility: if source.visibility == default_visibility {
        None
      } else {
        Some(source.visibility)
      },
      scope: source.scope.clone(),
    },
  );
  render_project_config(&config)
}

#[cfg(test)]
mod tests {
  use super::*;
  use arcantry_core::config::SourceKind;
  use std::path::PathBuf;

  fn source(id: &str, visibility: Visibility) -> ProjectSource {
    ProjectSource {
      id: id.to_owned(),
      kind: SourceKind::TodoTxt,
      path: "todo.txt".to_owned(),
      management: Management::Observe,
      adapter: "todo-txt@1".to_owned(),
      from: Vec::new(),
      managed_from: None,
      visibility,
      scope: ".".to_owned(),
      absolute_path: PathBuf::from("todo.txt"),
      exists: true,
      origin: "discovered",
      confidence: "high",
      adapter_status: "supported",
    }
  }

  #[test]
  fn adds_a_discovered_source_with_schema_and_inferred_visibility() {
    let rendered = add_configured_source(
      "config_version = 1\n",
      &source("tasks", Visibility::Private),
      SourceUpdate {
        path: ".local/todo.txt".to_owned(),
        management: Management::Manage,
        adapter: "todo-txt@1".to_owned(),
        from: vec!["legacy".to_owned()],
        managed_from: None,
      },
      false,
    )
    .unwrap();

    assert!(rendered.contains("[toml-schema]"));
    assert!(rendered.contains("[sources.tasks]"));
    assert!(rendered.contains("path = \".local/todo.txt\""));
    assert!(!rendered.contains("visibility ="));
  }

  #[test]
  fn rejects_an_existing_configured_source() {
    let configured = "config_version = 1\n\n[sources.tasks]\nkind = \"todo-txt\"\npath = \"todo.txt\"\nadapter = \"todo-txt@1\"\n";

    let error = add_configured_source(
      configured,
      &source("tasks", Visibility::Shared),
      SourceUpdate {
        path: "other.txt".to_owned(),
        management: Management::Manage,
        adapter: "todo-txt@1".to_owned(),
        from: Vec::new(),
        managed_from: None,
      },
      false,
    )
    .unwrap_err();

    assert!(error.to_string().contains("already exists"));
  }
}
