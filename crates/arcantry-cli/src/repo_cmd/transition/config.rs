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
