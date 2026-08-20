use anyhow::{Context, Result, bail};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{Array, DocumentMut, TableLike, value};

pub const PROJECT_CONFIG_VERSION: u8 = 1;
pub const PROJECT_CONFIG_FILENAME: &str = "arcantry.toml";
pub const PRIVATE_PROJECT_CONFIG_PATH: &str = ".local/arcantry.toml";
pub const PROJECT_CONFIG_SCHEMA_LOCATION: &str =
  "https://mrmaxie.github.io/arcantry/schemas/arcantry-config-v1.tosd";

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Management {
  Ignore,
  #[default]
  Observe,
  Validate,
  Manage,
}
impl Management {
  pub fn name(&self) -> &'static str {
    match self {
      Self::Ignore => "ignore",
      Self::Observe => "observe",
      Self::Validate => "validate",
      Self::Manage => "manage",
    }
  }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceKind {
  Openspec,
  Changelog,
  TodoTxt,
}
impl SourceKind {
  pub fn name(&self) -> &'static str {
    match self {
      Self::Openspec => "openspec",
      Self::Changelog => "changelog",
      Self::TodoTxt => "todo-txt",
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
  Shared,
  Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaReference {
  pub location: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolConfig {
  pub requires: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectRoot {
  pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseVersionSource {
  pub path: String,
  pub adapter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseConfig {
  pub adapter: String,
  pub manifests_path: String,
  pub changelog_source: String,
  #[serde(default = "default_tag_prefix")]
  pub tag_prefix: String,
  #[serde(default)]
  pub repository_url: Option<String>,
  pub version_sources: Vec<ReleaseVersionSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSourceConfig {
  pub kind: SourceKind,
  pub path: String,
  #[serde(default)]
  pub management: Management,
  pub adapter: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub from: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub managed_from: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub visibility: Option<Visibility>,
  #[serde(default = "default_scope", skip_serializing_if = "is_default_scope")]
  pub scope: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
  pub config_version: u8,
  #[serde(rename = "toml-schema", skip_serializing_if = "Option::is_none")]
  pub schema_reference: Option<SchemaReference>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tool: Option<ToolConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub project: Option<ProjectRoot>,
  #[serde(default)]
  pub sources: BTreeMap<String, RawSourceConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub release: Option<ReleaseConfig>,
  #[serde(default, flatten)]
  pub extra: BTreeMap<String, serde_json::Value>,
}

impl ProjectConfig {
  pub fn empty() -> Self {
    Self {
      config_version: PROJECT_CONFIG_VERSION,
      schema_reference: Some(SchemaReference {
        location: PROJECT_CONFIG_SCHEMA_LOCATION.to_owned(),
        version: Some("1.0.0".to_owned()),
      }),
      tool: None,
      project: None,
      sources: BTreeMap::new(),
      release: None,
      extra: BTreeMap::new(),
    }
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedProject {
  pub root: PathBuf,
  pub config_path: Option<PathBuf>,
  #[serde(skip_serializing)]
  pub config: Option<ProjectConfig>,
  pub mode: &'static str,
  pub scope: Option<&'static str>,
  pub shadowed_config_paths: Vec<PathBuf>,
}

pub fn parse_project_config(
  content: &str,
  tool_version: Option<&str>,
  allow_absolute_paths: bool,
) -> Result<ProjectConfig> {
  let config: ProjectConfig =
    toml_edit::de::from_str(content).context("Invalid Arcantry TOML configuration.")?;
  if config.config_version != PROJECT_CONFIG_VERSION {
    bail!("config_version must be 1.");
  }
  if !config.extra.is_empty() {
    bail!("Unrecognized top-level Arcantry configuration keys.");
  }
  if let Some(reference) = &config.schema_reference {
    if reference.location.trim().is_empty() {
      bail!("TOML Schema location must not be empty.");
    }
    if reference
      .version
      .as_ref()
      .is_some_and(|version| Version::parse(version).is_err())
    {
      bail!("TOML Schema version must be full SemVer.");
    }
  }
  if config
    .project
    .as_ref()
    .is_some_and(|project| project.root.trim().is_empty())
  {
    bail!("project.root must not be empty.");
  }
  if let Some(tool) = &config.tool {
    let requirement =
      VersionReq::parse(&tool.requires).context("tool.requires must be a valid SemVer range.")?;
    if let Some(tool_version) = tool_version
      && !requirement.matches(&Version::parse(tool_version)?)
    {
      bail!(
        "Arcantry {tool_version} does not satisfy configured range {}.",
        tool.requires
      );
    }
  }
  let ids: BTreeSet<_> = config.sources.keys().cloned().collect();
  for (id, source) in &config.sources {
    if id.is_empty()
      || !id
        .chars()
        .next()
        .is_some_and(|value| value.is_ascii_alphanumeric())
      || !id
        .chars()
        .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
    {
      bail!("source ids may contain letters, numbers, dot, underscore and hyphen.");
    }
    if !valid_adapter(&source.adapter) {
      bail!("adapter must use <name>@<integer-version>.");
    }
    if source.path.trim().is_empty() {
      bail!("Source {id} path must not be empty.");
    }
    if Path::new(&source.path).is_absolute() && !allow_absolute_paths {
      bail!("Absolute source path requires an explicit external configuration: {id}.");
    }
    if is_private_project_path(&source.path) && source.visibility == Some(Visibility::Shared) {
      bail!("Source {id} is inside .local and cannot be shared.");
    }
    for dependency in &source.from {
      if !ids.contains(dependency) {
        bail!("unknown source dependency: {dependency}");
      }
    }
    if source.kind == SourceKind::Changelog {
      if source.management == Management::Manage && source.from.is_empty() {
        bail!("managed changelog sources require at least one OpenSpec source.");
      }
      for dependency in &source.from {
        if let Some(authority) = config.sources.get(dependency) {
          if authority.kind != SourceKind::Openspec {
            bail!("managed changelog dependencies must be OpenSpec sources.");
          }
          if effective_visibility(source) == Visibility::Shared
            && effective_visibility(authority) == Visibility::Private
          {
            bail!("shared changelog sources cannot depend on private OpenSpec sources.");
          }
        }
      }
    }
    if let Some(version) = &source.managed_from
      && (source.kind != SourceKind::Changelog || Version::parse(version).is_err())
    {
      bail!("managed_from requires a changelog source and a full SemVer version.");
    }
  }
  validate_dependency_cycles(&config.sources)?;
  let managed_openspec = config
    .sources
    .iter()
    .filter(|(_, source)| {
      source.kind == SourceKind::Openspec && source.management == Management::Manage
    })
    .collect::<Vec<_>>();
  for (index, (id, source)) in managed_openspec.iter().enumerate() {
    if let Some((conflicting, _)) =
      managed_openspec
        .iter()
        .skip(index + 1)
        .find(|(_, candidate)| {
          effective_visibility(candidate) == effective_visibility(source)
            && scopes_overlap(&candidate.scope, &source.scope)
        })
    {
      bail!(
        "managed OpenSpec sources {id} and {conflicting} overlap at scope {}.",
        source.scope
      );
    }
  }
  if let Some(release) = &config.release {
    if release.adapter != "openspec-release@1" {
      bail!("release.adapter must be openspec-release@1.");
    }
    if release.manifests_path.trim().is_empty() {
      bail!("release.manifests_path must not be empty.");
    }
    if release.tag_prefix.is_empty() {
      bail!("release.tag_prefix must not be empty.");
    }
    if release.version_sources.is_empty() {
      bail!("release.version_sources must not be empty.");
    }
    if release
      .repository_url
      .as_ref()
      .is_some_and(|value| url::Url::parse(value).is_err())
    {
      bail!("release.repository_url must be a valid URL.");
    }
    let Some(changelog) = config.sources.get(&release.changelog_source) else {
      bail!(
        "unknown release changelog source: {}",
        release.changelog_source
      );
    };
    if changelog.kind != SourceKind::Changelog || changelog.management != Management::Manage {
      bail!("release changelog source must be a managed changelog source.");
    }
    if Path::new(&release.manifests_path).is_absolute() && !allow_absolute_paths {
      bail!("Absolute release manifests path requires an explicit external configuration.");
    }
    for source in &release.version_sources {
      if source.path.trim().is_empty() {
        bail!("release version source path must not be empty.");
      }
      if !matches!(
        source.adapter.as_str(),
        "json-package@1" | "cargo-workspace@1"
      ) {
        bail!(
          "release version source adapter is not supported: {}",
          source.adapter
        );
      }
      if Path::new(&source.path).is_absolute() && !allow_absolute_paths {
        bail!("Absolute release version source path requires an explicit external configuration.");
      }
    }
  }
  Ok(config)
}

fn validate_dependency_cycles(sources: &BTreeMap<String, RawSourceConfig>) -> Result<()> {
  fn visit<'a>(
    id: &'a str,
    sources: &'a BTreeMap<String, RawSourceConfig>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
  ) -> Result<()> {
    if visiting.contains(id) {
      bail!("source dependency cycle includes {id}.");
    }
    if visited.contains(id) {
      return Ok(());
    }
    visiting.insert(id);
    if let Some(source) = sources.get(id) {
      for dependency in &source.from {
        visit(dependency, sources, visiting, visited)?;
      }
    }
    visiting.remove(id);
    visited.insert(id);
    Ok(())
  }

  let mut visiting = BTreeSet::new();
  let mut visited = BTreeSet::new();
  for id in sources.keys() {
    visit(id, sources, &mut visiting, &mut visited)?;
  }
  Ok(())
}

fn scopes_overlap(left: &str, right: &str) -> bool {
  fn normalize(value: &str) -> String {
    let normalized = value.replace('\\', "/");
    let normalized = normalized.strip_prefix("./").unwrap_or(&normalized);
    let normalized = normalized.trim_matches('/');
    if normalized == "." {
      String::new()
    } else {
      normalized.to_owned()
    }
  }
  let left = normalize(left);
  let right = normalize(right);
  left.is_empty()
    || right.is_empty()
    || left == right
    || left.starts_with(&format!("{right}/"))
    || right.starts_with(&format!("{left}/"))
}

pub fn render_project_config(config: &ProjectConfig) -> Result<String> {
  toml_edit::ser::to_string_pretty(config).context("Could not render Arcantry TOML configuration.")
}

pub struct ProjectSourcePatch<'a> {
  pub id: &'a str,
  pub path: &'a str,
  pub management: &'a Management,
  pub adapter: &'a str,
  pub from: &'a [String],
  pub managed_from: Option<&'a str>,
  pub allow_absolute_paths: bool,
}

pub fn patch_project_source(content: &str, patch: ProjectSourcePatch<'_>) -> Result<String> {
  parse_project_config(content, Some(crate::VERSION), patch.allow_absolute_paths)?;
  let mut document = content
    .parse::<DocumentMut>()
    .context("Invalid Arcantry TOML configuration.")?;
  let table = document
    .get_mut("sources")
    .and_then(|sources| sources.get_mut(patch.id))
    .and_then(|source| source.as_table_like_mut())
    .with_context(|| format!("Configured source table is missing: {}.", patch.id))?;

  table.insert("path", value(patch.path));
  table.insert("management", value(patch.management.name()));
  table.insert("adapter", value(patch.adapter));
  let mut appended = false;
  if patch.from.is_empty() {
    table.remove("from");
  } else {
    let mut values = Array::new();
    values.extend(patch.from.iter().map(String::as_str));
    let is_new = !table.contains_key("from");
    table.insert("from", value(values));
    mark_appended(table, "from", is_new, &mut appended);
  }
  if let Some(version) = patch.managed_from {
    let is_new = !table.contains_key("managed_from");
    table.insert("managed_from", value(version));
    mark_appended(table, "managed_from", is_new, &mut appended);
  } else {
    table.remove("managed_from");
  }

  let mut updated = document.to_string();
  if appended {
    updated = updated
      .strip_suffix("\r\n")
      .or_else(|| updated.strip_suffix('\n'))
      .unwrap_or(&updated)
      .to_owned();
  }
  parse_project_config(&updated, Some(crate::VERSION), patch.allow_absolute_paths)?;
  Ok(updated)
}

fn mark_appended(table: &mut dyn TableLike, key: &str, is_new: bool, appended: &mut bool) {
  if is_new && !*appended {
    table
      .key_mut(key)
      .expect("new source keys are present")
      .leaf_decor_mut()
      .set_prefix("\n");
  }
  *appended |= is_new;
}

pub fn resolve_project(
  cwd: &Path,
  config_path: Option<&Path>,
  cwd_explicit: bool,
  tool_version: Option<&str>,
) -> Result<ResolvedProject> {
  let cwd = absolute(cwd)?;
  let discovered = discover_project_config(&cwd);
  let explicit = config_path.map(|path| {
    if path.is_absolute() {
      path.to_path_buf()
    } else {
      cwd.join(path)
    }
  });
  let active = explicit.clone().or(discovered.0.clone());
  let Some(active) = active else {
    return Ok(ResolvedProject {
      root: cwd,
      config_path: None,
      config: None,
      mode: "wild",
      scope: None,
      shadowed_config_paths: Vec::new(),
    });
  };
  let content =
    fs::read_to_string(&active).with_context(|| format!("Could not read {}.", active.display()))?;
  let mut config = parse_project_config(&content, tool_version, explicit.is_some())?;
  let scope = if explicit.is_some() && discovered.0.as_ref() != Some(&active) {
    "external"
  } else if is_private_config_path(&active) {
    "private"
  } else {
    "shared"
  };
  let config_root = if scope == "private" {
    active.parent().and_then(Path::parent).unwrap_or(&cwd)
  } else {
    active.parent().unwrap_or(&cwd)
  };
  let root = if cwd_explicit {
    cwd.clone()
  } else if let Some(project) = &config.project {
    config_root.join(&project.root)
  } else if explicit.is_some() {
    cwd.clone()
  } else {
    config_root.to_path_buf()
  };
  if explicit.is_some() && active.starts_with(&root) {
    config = parse_project_config(&content, tool_version, false)?;
  }
  let shadowed = if explicit.is_none() {
    discovered.1
  } else {
    discovered
      .0
      .into_iter()
      .chain(discovered.1)
      .filter(|path| path != &active)
      .collect()
  };
  Ok(ResolvedProject {
    root: absolute(&root)?,
    config_path: Some(active),
    config: Some(config),
    mode: "configured",
    scope: Some(scope),
    shadowed_config_paths: shadowed,
  })
}

pub fn effective_visibility(source: &RawSourceConfig) -> Visibility {
  source.visibility.unwrap_or_else(|| {
    if is_private_project_path(&source.path) {
      Visibility::Private
    } else {
      Visibility::Shared
    }
  })
}
fn discover_project_config(start: &Path) -> (Option<PathBuf>, Vec<PathBuf>) {
  for directory in start.ancestors() {
    let private = directory.join(".local").join(PROJECT_CONFIG_FILENAME);
    let shared = directory.join(PROJECT_CONFIG_FILENAME);
    let has_private = private.is_file();
    let has_shared = shared.is_file();
    if has_private || has_shared {
      return (
        Some(if has_private { private } else { shared.clone() }),
        if has_private && has_shared {
          vec![shared]
        } else {
          Vec::new()
        },
      );
    }
  }
  (None, Vec::new())
}
fn default_scope() -> String {
  ".".to_owned()
}
fn is_default_scope(value: &str) -> bool {
  value == "."
}
fn default_tag_prefix() -> String {
  "v".to_owned()
}
fn valid_adapter(adapter: &str) -> bool {
  adapter.split_once('@').is_some_and(|(name, version)| {
    !name.is_empty()
      && name
        .chars()
        .next()
        .is_some_and(|value| value.is_ascii_lowercase())
      && name
        .chars()
        .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit() || value == '-')
      && version.parse::<u32>().is_ok()
  })
}
pub fn is_private_project_path(path: &str) -> bool {
  let normalized = path.replace('\\', "/");
  let normalized = normalized.strip_prefix("./").unwrap_or(&normalized);
  let first = normalized.split('/').next().unwrap_or_default();
  if cfg!(windows) {
    first.eq_ignore_ascii_case(".local")
  } else {
    first == ".local"
  }
}
fn is_private_config_path(path: &Path) -> bool {
  path
    .file_name()
    .is_some_and(|name| name.eq_ignore_ascii_case(PROJECT_CONFIG_FILENAME))
    && path
      .parent()
      .and_then(Path::file_name)
      .is_some_and(|name| name.eq_ignore_ascii_case(".local"))
}
fn absolute(path: &Path) -> Result<PathBuf> {
  if path.is_absolute() {
    Ok(dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf()))
  } else {
    let path = std::env::current_dir()?.join(path);
    Ok(dunce::canonicalize(&path).unwrap_or(path))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const CONFIGURED: &str = r#"config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]
"#;

  #[test]
  fn validates_dependency_authority_and_cycles() {
    parse_project_config(CONFIGURED, Some("1.0.0"), false).unwrap();
    let cycle = CONFIGURED.replace(
      "adapter = \"openspec@1\"",
      "adapter = \"openspec@1\"\nfrom = [\"history\"]",
    );
    assert!(
      parse_project_config(&cycle, None, false)
        .unwrap_err()
        .to_string()
        .contains("dependency cycle")
    );
    let missing = CONFIGURED.replace("from = [\"intent\"]", "from = []");
    assert!(
      parse_project_config(&missing, None, false)
        .unwrap_err()
        .to_string()
        .contains("require at least one OpenSpec")
    );
  }

  #[test]
  fn rejects_private_authority_for_shared_changelog() {
    let content = CONFIGURED.replace("path = \"openspec\"", "path = \".local/openspec\"");
    assert!(
      parse_project_config(&content, None, false)
        .unwrap_err()
        .to_string()
        .contains("cannot depend on private OpenSpec")
    );
  }

  #[cfg(windows)]
  #[test]
  fn treats_local_paths_case_insensitively_on_windows() {
    let content = r#"config_version = 1

[sources.tasks]
kind = "todo-txt"
path = ".LOCAL/todo.txt"
visibility = "shared"
adapter = "todo-txt@1"
"#;

    assert!(
      parse_project_config(content, None, false)
        .unwrap_err()
        .to_string()
        .contains("inside .local")
    );
  }

  #[test]
  fn rejects_overlapping_managed_openspec_scopes() {
    let content = r#"config_version = 1

[sources.workspace]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"
scope = "."

[sources.package]
kind = "openspec"
path = "packages/app/openspec"
management = "manage"
adapter = "openspec@1"
scope = "packages/app"
"#;
    assert!(
      parse_project_config(content, None, false)
        .unwrap_err()
        .to_string()
        .contains("overlap")
    );
  }

  #[test]
  fn patches_one_source_through_the_toml_document() {
    let content = CONFIGURED.replace(
      "[sources.history]",
      "# Keep this source comment.\n[sources.history]",
    );
    let updated = patch_project_source(
      &content,
      ProjectSourcePatch {
        id: "history",
        path: "docs/CHANGELOG.md",
        management: &Management::Observe,
        adapter: "keep-a-changelog@1",
        from: &[],
        managed_from: Some("1.0.0"),
        allow_absolute_paths: false,
      },
    )
    .unwrap();

    assert!(updated.contains("# Keep this source comment."));
    assert!(updated.contains("path = \"docs/CHANGELOG.md\""));
    assert!(updated.contains("management = \"observe\""));
    assert!(updated.contains("managed_from = \"1.0.0\""));
    assert!(!updated.contains("from = [\"intent\"]"));
  }
}
