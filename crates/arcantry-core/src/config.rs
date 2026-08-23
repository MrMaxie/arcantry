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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseTopology {
  #[default]
  Single,
  Independent,
  Composed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseUnitSelector {
  pub source: String,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub components: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseUnitConfig {
  pub manifests_path: String,
  pub changelog_source: String,
  pub tag_prefix: String,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub dependencies: Vec<String>,
  pub version_sources: Vec<ReleaseVersionSource>,
  pub selectors: Vec<ReleaseUnitSelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseConfig {
  pub adapter: String,
  #[serde(default, skip_serializing_if = "is_single_topology")]
  pub topology: ReleaseTopology,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub manifests_path: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub changelog_source: Option<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub tag_prefix: Option<String>,
  #[serde(default)]
  pub repository_url: Option<String>,
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub version_sources: Vec<ReleaseVersionSource>,
  #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
  pub units: BTreeMap<String, ReleaseUnitConfig>,
}

pub type ReleaseSystem = ReleaseConfig;

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
  let document = content
    .parse::<DocumentMut>()
    .context("Invalid Arcantry TOML configuration.")?;
  let mut config: ProjectConfig =
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
    if path_escapes_project(&source.path) {
      bail!("Source {id} path must stay within the project.");
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
    if !matches!(
      release.adapter.as_str(),
      "openspec-release@1" | "openspec-release@2"
    ) {
      bail!("release.adapter must be openspec-release@1 or openspec-release@2.");
    }
    if release.adapter == "openspec-release@1"
      && document
        .get("release")
        .and_then(toml_edit::Item::as_table_like)
        .is_some_and(|table| table.contains_key("topology") || table.contains_key("units"))
    {
      bail!("openspec-release@1 does not accept topology or units.");
    }
    if release
      .repository_url
      .as_ref()
      .is_some_and(|value| url::Url::parse(value).is_err())
    {
      bail!("release.repository_url must be a valid URL.");
    }
    if release.adapter == "openspec-release@1" || release.topology == ReleaseTopology::Single {
      if release.adapter == "openspec-release@1" && release.topology != ReleaseTopology::Single {
        bail!("openspec-release@1 supports only the single topology.");
      }
      if !release.units.is_empty() {
        bail!("single release topology cannot define release.units.");
      }
      validate_release_paths(
        &config.sources,
        release.manifests_path.as_deref(),
        release.changelog_source.as_deref(),
        release.tag_prefix.as_deref().unwrap_or("v"),
        &release.version_sources,
        allow_absolute_paths,
      )?;
    } else {
      validate_release_units(&config, release, allow_absolute_paths)?;
    }
  }
  if let Some(release) = config.release.as_mut()
    && release.units.is_empty()
    && release.tag_prefix.is_none()
  {
    release.tag_prefix = Some("v".to_owned());
  }
  Ok(config)
}

fn validate_release_paths(
  sources: &BTreeMap<String, RawSourceConfig>,
  manifests_path: Option<&str>,
  changelog_source: Option<&str>,
  tag_prefix: &str,
  version_sources: &[ReleaseVersionSource],
  allow_absolute_paths: bool,
) -> Result<()> {
  let manifests_path = manifests_path.context("release.manifests_path must not be empty.")?;
  if manifests_path.trim().is_empty() {
    bail!("release.manifests_path must not be empty.");
  }
  let changelog_source = changelog_source.context("release.changelog_source must not be empty.")?;
  let Some(changelog) = sources.get(changelog_source) else {
    bail!("unknown release changelog source: {changelog_source}");
  };
  if changelog.kind != SourceKind::Changelog || changelog.management != Management::Manage {
    bail!("release changelog source must be a managed changelog source.");
  }
  if tag_prefix.is_empty() {
    bail!("release.tag_prefix must not be empty.");
  }
  if version_sources.is_empty() {
    bail!("release.version_sources must not be empty.");
  }
  validate_release_path(manifests_path, "manifests", allow_absolute_paths)?;
  for source in version_sources {
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
    validate_release_path(&source.path, "version source", allow_absolute_paths)?;
  }
  Ok(())
}

fn validate_release_path(path: &str, label: &str, allow_absolute_paths: bool) -> Result<()> {
  if Path::new(path).is_absolute() && !allow_absolute_paths {
    bail!("Absolute release {label} path requires an explicit external configuration.");
  }
  if path_escapes_project(path) {
    let label = if label == "manifests" {
      "manifests"
    } else {
      "version source"
    };
    bail!("Release {label} path must stay within the project.");
  }
  Ok(())
}

fn validate_release_units(
  config: &ProjectConfig,
  release: &ReleaseConfig,
  allow_absolute_paths: bool,
) -> Result<()> {
  if release.units.is_empty() {
    bail!("multi-unit release configuration requires at least one unit.");
  }
  if release.manifests_path.is_some()
    || release.changelog_source.is_some()
    || !release.version_sources.is_empty()
  {
    bail!(
      "multi-unit release configuration must use release.units instead of flat release fields."
    );
  }
  let ids: BTreeSet<_> = release.units.keys().cloned().collect();
  let mut paths = BTreeMap::<String, String>::new();
  let mut changelogs = BTreeMap::<String, String>::new();
  let mut prefixes = BTreeMap::<String, String>::new();
  let mut whole_sources = BTreeMap::<String, String>::new();
  let mut component_owners = BTreeMap::<String, String>::new();
  let mut edges = 0usize;
  for (id, unit) in &release.units {
    if !valid_source_id(id) {
      bail!("release unit ids may contain letters, numbers, dot, underscore and hyphen.");
    }
    validate_release_paths(
      &config.sources,
      Some(&unit.manifests_path),
      Some(&unit.changelog_source),
      &unit.tag_prefix,
      &unit.version_sources,
      allow_absolute_paths,
    )?;
    claim_unique(
      &mut paths,
      &unit.manifests_path,
      id,
      "release manifests path",
    )?;
    claim_unique(
      &mut changelogs,
      &unit.changelog_source,
      id,
      "release changelog source",
    )?;
    claim_unique(&mut prefixes, &unit.tag_prefix, id, "release tag prefix")?;
    for source in &unit.version_sources {
      claim_unique(&mut paths, &source.path, id, "release-owned path")?;
    }
    if release.topology == ReleaseTopology::Independent && !unit.dependencies.is_empty() {
      bail!("independent release units cannot declare dependencies.");
    }
    if unit.dependencies.iter().collect::<BTreeSet<_>>().len() != unit.dependencies.len() {
      bail!("release unit dependencies must be unique: {id}");
    }
    edges += unit.dependencies.len();
    for dependency in &unit.dependencies {
      if !ids.contains(dependency) {
        bail!("unknown release unit dependency: {dependency}");
      }
    }
    if unit.selectors.is_empty() {
      bail!("release unit selectors must not be empty: {id}");
    }
    let changelog = &config.sources[&unit.changelog_source];
    for selector in &unit.selectors {
      let Some(source) = config.sources.get(&selector.source) else {
        bail!(
          "release selector source must be configured OpenSpec: {}",
          selector.source
        );
      };
      if source.kind != SourceKind::Openspec {
        bail!(
          "release selector source must be configured OpenSpec: {}",
          selector.source
        );
      }
      if !changelog.from.contains(&selector.source) {
        bail!(
          "release selector source must be an authority of {}: {}",
          unit.changelog_source,
          selector.source
        );
      }
      match &selector.components {
        None => {
          if whole_sources.contains_key(&selector.source)
            || component_owners
              .keys()
              .any(|key| key.starts_with(&format!("{}\0", selector.source)))
          {
            bail!(
              "release selector ownership overlaps for source {}.",
              selector.source
            );
          }
          whole_sources.insert(selector.source.clone(), id.clone());
        }
        Some(components) => {
          if components.is_empty()
            || components
              .iter()
              .any(|component| !valid_component_id(component))
          {
            bail!("release selector components must be non-empty stable component ids.");
          }
          if whole_sources.contains_key(&selector.source) {
            bail!(
              "release selector ownership overlaps for source {}.",
              selector.source
            );
          }
          for component in components {
            claim_unique(
              &mut component_owners,
              &format!("{}\0{component}", selector.source),
              id,
              "release selector component",
            )?;
          }
        }
      }
    }
  }
  if release.topology == ReleaseTopology::Composed {
    if edges == 0 {
      bail!("composed release topology requires at least one dependency edge.");
    }
    validate_release_unit_cycles(&release.units)?;
  }
  Ok(())
}

fn claim_unique(
  owners: &mut BTreeMap<String, String>,
  value: &str,
  unit: &str,
  label: &str,
) -> Result<()> {
  if owners.contains_key(value) {
    bail!("{label} must be unique across release units: {value}");
  }
  owners.insert(value.to_owned(), unit.to_owned());
  Ok(())
}

fn validate_release_unit_cycles(units: &BTreeMap<String, ReleaseUnitConfig>) -> Result<()> {
  fn visit<'a>(
    id: &'a str,
    units: &'a BTreeMap<String, ReleaseUnitConfig>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
  ) -> Result<()> {
    if visiting.contains(id) {
      bail!("release unit dependency cycle includes {id}.");
    }
    if visited.contains(id) {
      return Ok(());
    }
    visiting.insert(id);
    for dependency in &units[id].dependencies {
      visit(dependency, units, visiting, visited)?;
    }
    visiting.remove(id);
    visited.insert(id);
    Ok(())
  }
  let mut visiting = BTreeSet::new();
  let mut visited = BTreeSet::new();
  for id in units.keys() {
    visit(id, units, &mut visiting, &mut visited)?;
  }
  Ok(())
}

fn valid_source_id(id: &str) -> bool {
  !id.is_empty()
    && id
      .chars()
      .next()
      .is_some_and(|value| value.is_ascii_alphanumeric())
    && id
      .chars()
      .all(|value| value.is_ascii_alphanumeric() || matches!(value, '.' | '_' | '-'))
}

fn valid_component_id(id: &str) -> bool {
  let mut parts = id.split(':');
  let valid_part = |part: &str| {
    !part.is_empty()
      && part.split('-').all(|segment| {
        !segment.is_empty()
          && segment
            .chars()
            .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit())
      })
  };
  let first = parts.next().is_some_and(valid_part);
  first && parts.next().is_none_or(valid_part) && parts.next().is_none()
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
fn is_single_topology(value: &ReleaseTopology) -> bool {
  *value == ReleaseTopology::Single
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
  let portable = path.replace('\\', "/");
  let normalized = normalize_path_lexically(Path::new(&portable));
  let first = normalized
    .components()
    .find_map(|component| match component {
      std::path::Component::Normal(value) => value.to_str(),
      _ => None,
    })
    .unwrap_or_default();
  if cfg!(windows) {
    first.eq_ignore_ascii_case(".local")
  } else {
    first == ".local"
  }
}

pub(crate) fn normalize_path_lexically(path: &Path) -> PathBuf {
  let mut normalized = PathBuf::new();
  for component in path.components() {
    match component {
      std::path::Component::CurDir => {}
      std::path::Component::ParentDir => match normalized.components().next_back() {
        Some(std::path::Component::Normal(_)) => {
          normalized.pop();
        }
        Some(std::path::Component::RootDir | std::path::Component::Prefix(_)) => {}
        _ => normalized.push(component.as_os_str()),
      },
      _ => normalized.push(component.as_os_str()),
    }
  }
  normalized
}

fn path_escapes_project(path: &str) -> bool {
  let portable = path.replace('\\', "/");
  matches!(
    normalize_path_lexically(Path::new(&portable))
      .components()
      .next(),
    Some(std::path::Component::ParentDir)
  )
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
  fn rejects_shared_visibility_for_normalized_local_paths() {
    let content = r#"config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "public/../.local/todo.txt"
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
  fn rejects_paths_that_escape_the_project() {
    let source = CONFIGURED.replace("path = \"openspec\"", "path = \"../openspec\"");
    assert!(
      parse_project_config(&source, None, false)
        .unwrap_err()
        .to_string()
        .contains("must stay within the project")
    );

    let manifests = format!(
      r#"{CONFIGURED}
[release]
adapter = "openspec-release@1"
manifests_path = "history/../../releases"
changelog_source = "history"
tag_prefix = "v"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
"#,
    );
    assert!(
      parse_project_config(&manifests, None, false)
        .unwrap_err()
        .to_string()
        .contains("Release manifests path must stay within the project")
    );

    let version_source = manifests
      .replace("history/../../releases", "releases")
      .replace("path = \"package.json\"", "path = \"../package.json\"");
    assert!(
      parse_project_config(&version_source, None, false)
        .unwrap_err()
        .to_string()
        .contains("Release version source path must stay within the project")
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

  #[test]
  fn validates_composed_release_units() {
    let content = format!(
      r#"{CONFIGURED}
[sources.core_history]
kind = "changelog"
path = "packages/core/CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]

[release]
adapter = "openspec-release@2"
topology = "composed"

[release.units.core]
manifests_path = "releases/core"
changelog_source = "core_history"
tag_prefix = "core/v"

[[release.units.core.version_sources]]
path = "packages/core/package.json"
adapter = "json-package@1"

[[release.units.core.selectors]]
source = "intent"
components = ["product:core"]

[release.units.app]
manifests_path = "releases/app"
changelog_source = "history"
tag_prefix = "app/v"
dependencies = ["core"]

[[release.units.app.version_sources]]
path = "apps/app/package.json"
adapter = "json-package@1"

[[release.units.app.selectors]]
source = "intent"
components = ["product:app"]
"#
    );
    let parsed = parse_project_config(&content, Some("1.0.0"), false).unwrap();
    let release = parsed.release.unwrap();
    assert_eq!(release.topology, ReleaseTopology::Composed);
    assert_eq!(release.units["app"].dependencies, ["core"]);

    let overlapping = content.replace(
      "components = [\"product:app\"]",
      "components = [\"product:core\"]",
    );
    let overlap_error = parse_project_config(&overlapping, None, false)
      .unwrap_err()
      .to_string();
    assert!(
      overlap_error.contains("ownership overlaps") || overlap_error.contains("must be unique")
    );
    let cycle = content.replace(
      "tag_prefix = \"core/v\"",
      "tag_prefix = \"core/v\"\ndependencies = [\"app\"]",
    );
    assert!(
      parse_project_config(&cycle, None, false)
        .unwrap_err()
        .to_string()
        .contains("dependency cycle")
    );
  }
}
