use crate::config::{ResolvedProject, Visibility, effective_visibility};
use crate::project_plan::{ProjectPlan, create_write_operation};
use anyhow::{Context, Result, bail};
use chrono::NaiveDate;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseManifest {
  pub version: String,
  pub date: String,
  pub changes: Vec<String>,
  #[serde(default, skip_serializing_if = "Option::is_none")]
  pub baseline: Option<bool>,
}
#[derive(Debug, Clone, Serialize)]
pub struct ReleasePlan {
  pub current: String,
  pub next: String,
  pub impact: String,
  pub changes: Vec<String>,
}
#[derive(Debug, Clone)]
struct Artifact {
  category: String,
  impact: String,
  visibility: String,
  title: String,
  body: String,
}
#[derive(Debug)]
struct Configuration {
  root: PathBuf,
  releases: String,
  changelog: String,
  changelog_visibility: Visibility,
  openspec: Vec<String>,
  repository_url: Option<String>,
  tag_prefix: String,
  version_sources: Vec<(String, String)>,
}
#[derive(Debug)]
struct State {
  archived: BTreeMap<String, Artifact>,
  manifests: Vec<ReleaseManifest>,
  assigned: BTreeSet<String>,
}

pub fn baseline(project: &ResolvedProject, version: &str, date: &str) -> Result<ProjectPlan> {
  plan_result(project, || {
    Version::parse(version)
      .with_context(|| format!("Release version must be full stable SemVer: {version}."))?;
    validate_date(date)?;
    let configuration = configuration(project)?;
    if !read_manifests(&configuration)?.is_empty() {
      bail!("Release baseline requires a project without release manifests.");
    }
    validate_versions(&configuration, version)?;
    let state = state(&configuration)?;
    let manifest = ReleaseManifest {
      version: version.to_owned(),
      date: date.to_owned(),
      changes: Vec::new(),
      baseline: Some(true),
    };
    let mut plan = ProjectPlan::new(
      configuration.root.clone(),
      "release",
      "adopt",
      "openspec-release@1",
    );
    plan.operations.push(create_write_operation(
      &configuration.root,
      &format!(
        "{}/{}.yaml",
        configuration.releases.trim_end_matches(['/', '\\']),
        version
      ),
      render_manifest(&manifest)?,
      path_visibility(&configuration.root, &configuration.releases),
    )?);
    plan.operations.push(create_write_operation(
      &configuration.root,
      &configuration.changelog,
      render_changelog(
        &configuration,
        &State {
          archived: state.archived,
          manifests: vec![manifest],
          assigned: BTreeSet::new(),
        },
      ),
      configuration.changelog_visibility,
    )?);
    Ok(plan)
  })
}

pub fn inspect(project: &ResolvedProject) -> Result<ReleasePlan> {
  inspect_configuration(&configuration(project)?)
}

pub fn cut(project: &ResolvedProject, date: &str) -> Result<ProjectPlan> {
  plan_result(project, || {
    validate_date(date)?;
    let configuration = configuration(project)?;
    let state = state(&configuration)?;
    let release = inspect_configuration(&configuration)?;
    if release.changes.is_empty() {
      bail!("No unassigned archived changes to release.");
    }
    validate_versions(&configuration, &release.current)?;
    let manifest = ReleaseManifest {
      version: release.next,
      date: date.to_owned(),
      changes: release.changes,
      baseline: None,
    };
    let mut plan = ProjectPlan::new(
      configuration.root.clone(),
      "release",
      "adopt",
      "openspec-release@1",
    );
    plan.operations.push(create_write_operation(
      &configuration.root,
      &format!(
        "{}/{}.yaml",
        configuration.releases.trim_end_matches(['/', '\\']),
        manifest.version
      ),
      render_manifest(&manifest)?,
      path_visibility(&configuration.root, &configuration.releases),
    )?);
    for (path, adapter) in &configuration.version_sources {
      let content = update_version(&configuration.root, path, adapter, &manifest.version)?;
      plan.operations.push(create_write_operation(
        &configuration.root,
        path,
        content,
        path_visibility(&configuration.root, path),
      )?);
    }
    let mut manifests = state.manifests;
    manifests.push(manifest);
    plan.operations.push(create_write_operation(
      &configuration.root,
      &configuration.changelog,
      render_changelog(
        &configuration,
        &State {
          archived: state.archived,
          manifests,
          assigned: state.assigned,
        },
      ),
      configuration.changelog_visibility,
    )?);
    Ok(plan)
  })
}

pub fn render(project: &ResolvedProject) -> Result<ProjectPlan> {
  plan_result(project, || {
    let configuration = configuration(project)?;
    let desired = render_changelog(&configuration, &state(&configuration)?);
    let mut plan = ProjectPlan::new(
      configuration.root.clone(),
      "release",
      "adopt",
      "openspec-release@1",
    );
    let operation = create_write_operation(
      &configuration.root,
      &configuration.changelog,
      desired,
      configuration.changelog_visibility,
    )?;
    if operation.expected_hash != operation.content_hash {
      plan.operations.push(operation);
    }
    Ok(plan)
  })
}

pub fn check(project: &ResolvedProject, sealed: bool) -> Result<()> {
  let configuration = configuration(project)?;
  let state = state(&configuration)?;
  validate_versions(
    &configuration,
    state
      .manifests
      .last()
      .map_or("0.0.0", |manifest| &manifest.version),
  )?;
  let changelog = fs::read_to_string(configuration.root.join(&configuration.changelog))
    .context("CHANGELOG.md is missing")?;
  if changelog != render_changelog(&configuration, &state) {
    bail!("CHANGELOG.md is stale; run the configured release render command");
  }
  if sealed {
    let active = configuration
      .openspec
      .iter()
      .flat_map(|path| {
        fs::read_dir(configuration.root.join(path).join("changes"))
          .into_iter()
          .flatten()
          .flatten()
      })
      .filter(|entry| {
        entry.file_type().is_ok_and(|kind| kind.is_dir()) && entry.file_name() != "archive"
      })
      .count();
    if active > 0 {
      bail!("active OpenSpec changes are not release-complete");
    }
    let output = duct::cmd("git", ["status", "--porcelain=v1", "--untracked-files=all"])
      .dir(&configuration.root)
      .read()?;
    if !output.trim().is_empty() {
      bail!("repository contains unsealed working tree changes");
    }
  }
  Ok(())
}

fn plan_result(
  project: &ResolvedProject,
  build: impl FnOnce() -> Result<ProjectPlan>,
) -> Result<ProjectPlan> {
  match build() {
    Ok(plan) => Ok(plan),
    Err(error) => {
      let mut plan = ProjectPlan::new(
        project.root.clone(),
        "release",
        "adopt",
        "openspec-release@1",
      );
      plan.conflicts.push(error.to_string());
      Ok(plan)
    }
  }
}

fn configuration(project: &ResolvedProject) -> Result<Configuration> {
  let config = project
    .config
    .as_ref()
    .context("Project has no [release] configuration.")?;
  let release = config
    .release
    .as_ref()
    .context("Project has no [release] configuration.")?;
  let changelog = config
    .sources
    .get(&release.changelog_source)
    .with_context(|| {
      format!(
        "Release changelog source is not managed: {}.",
        release.changelog_source
      )
    })?;
  if changelog.kind != crate::config::SourceKind::Changelog
    || changelog.management != crate::config::Management::Manage
  {
    bail!(
      "Release changelog source is not managed: {}.",
      release.changelog_source
    );
  }
  let openspec = changelog
    .from
    .iter()
    .map(|id| {
      config
        .sources
        .get(id)
        .filter(|source| source.kind == crate::config::SourceKind::Openspec)
        .map(|source| source.path.clone())
        .with_context(|| format!("Release changelog dependency is not OpenSpec: {id}."))
    })
    .collect::<Result<Vec<_>>>()?;
  Ok(Configuration {
    root: project.root.clone(),
    releases: release.manifests_path.clone(),
    changelog: changelog.path.clone(),
    changelog_visibility: effective_visibility(changelog),
    openspec,
    repository_url: release.repository_url.clone(),
    tag_prefix: release.tag_prefix.clone(),
    version_sources: release
      .version_sources
      .iter()
      .map(|source| (source.path.clone(), source.adapter.clone()))
      .collect(),
  })
}

fn inspect_configuration(configuration: &Configuration) -> Result<ReleasePlan> {
  let state = state(configuration)?;
  let changes: Vec<_> = state
    .archived
    .keys()
    .filter(|id| !state.assigned.contains(*id))
    .cloned()
    .collect();
  let impacts: Vec<_> = changes
    .iter()
    .map(|id| state.archived[id].impact.as_str())
    .collect();
  let impact = highest_impact(&impacts).to_owned();
  if impact == "none" && !changes.is_empty() {
    bail!(
      "completed changes must declare a SemVer impact: {}",
      changes.join(", ")
    );
  }
  let current = state
    .manifests
    .last()
    .map_or_else(|| "0.0.0".to_owned(), |manifest| manifest.version.clone());
  let next = bump(&current, &impact)?;
  Ok(ReleasePlan {
    current,
    next,
    impact,
    changes,
  })
}

fn state(configuration: &Configuration) -> Result<State> {
  let archived = read_archived(configuration)?;
  let manifests = read_manifests(configuration)?;
  let mut assigned = BTreeSet::new();
  for manifest in &manifests {
    for change in &manifest.changes {
      if !archived.contains_key(change) {
        bail!(
          "release {} references unknown archived change: {change}",
          manifest.version
        );
      }
      if !assigned.insert(change.clone()) {
        bail!("archived change assigned more than once: {change}");
      }
    }
  }
  Ok(State {
    archived,
    manifests,
    assigned,
  })
}

fn read_archived(configuration: &Configuration) -> Result<BTreeMap<String, Artifact>> {
  let mut artifacts = BTreeMap::new();
  for path in &configuration.openspec {
    let archive = configuration.root.join(path).join("changes/archive");
    if !archive.exists() {
      continue;
    }
    for entry in fs::read_dir(archive)? {
      let entry = entry?;
      if !entry.file_type()?.is_dir() {
        continue;
      }
      let directory = entry.file_name().to_string_lossy().into_owned();
      let id = directory
        .get(11..)
        .context(format!("invalid OpenSpec archive directory: {directory}"))?
        .to_owned();
      if artifacts.contains_key(&id) {
        bail!("duplicate archived change id: {id}");
      }
      artifacts.insert(
        id.clone(),
        parse_artifact(
          &fs::read_to_string(entry.path().join("release.md"))
            .with_context(|| format!("archived change {id} has no release.md"))?,
        )?,
      );
    }
  }
  Ok(artifacts)
}

#[derive(Deserialize)]
struct ArtifactMetadata {
  category: String,
  impact: String,
  visibility: String,
  components: Vec<String>,
}
fn parse_artifact(source: &str) -> Result<Artifact> {
  let body = source
    .strip_prefix("---\n")
    .or_else(|| source.strip_prefix("---\r\n"))
    .context("release.md must start with YAML frontmatter")?;
  let end = body
    .find("\n---")
    .context("release.md must start with YAML frontmatter")?;
  let metadata: ArtifactMetadata = serde_saphyr::from_str(&body[..end])?;
  if !matches!(
    metadata.category.as_str(),
    "added" | "changed" | "fixed" | "deprecated" | "removed" | "security"
  ) {
    bail!("invalid release category: {}", metadata.category);
  }
  if !matches!(
    metadata.impact.as_str(),
    "none" | "patch" | "minor" | "major"
  ) {
    bail!("invalid release impact: {}", metadata.impact);
  }
  if !matches!(metadata.visibility.as_str(), "public" | "internal")
    || metadata.components.is_empty()
  {
    bail!("invalid release visibility or components");
  }
  let content = body[end + 4..].trim();
  let mut lines = content.lines();
  let title = lines
    .next()
    .and_then(|line| line.strip_prefix("# "))
    .context("release.md must contain a level-one title")?
    .trim()
    .to_owned();
  let body = lines.collect::<Vec<_>>().join("\n").trim().to_owned();
  if body.is_empty() {
    bail!("release.md must describe the delivered outcome");
  }
  Ok(Artifact {
    category: metadata.category,
    impact: metadata.impact,
    visibility: metadata.visibility,
    title,
    body,
  })
}

fn read_manifests(configuration: &Configuration) -> Result<Vec<ReleaseManifest>> {
  let directory = configuration.root.join(&configuration.releases);
  if !directory.exists() {
    return Ok(Vec::new());
  }
  let mut manifests = Vec::new();
  for entry in fs::read_dir(directory)? {
    let entry = entry?;
    if entry.path().extension().and_then(|value| value.to_str()) != Some("yaml") {
      continue;
    }
    let manifest: ReleaseManifest = serde_saphyr::from_str(&fs::read_to_string(entry.path())?)?;
    if entry.path().file_stem().and_then(|value| value.to_str()) != Some(&manifest.version) {
      bail!("release manifest filename must match version");
    }
    Version::parse(&manifest.version)?;
    validate_date(&manifest.date)?;
    if manifest.baseline == Some(true) && !manifest.changes.is_empty() {
      bail!("baseline release cannot assign changes");
    }
    if manifest.baseline != Some(true) && manifest.changes.is_empty() {
      bail!("non-baseline release must assign changes");
    }
    manifests.push(manifest);
  }
  manifests.sort_by(|left, right| {
    Version::parse(&left.version)
      .unwrap()
      .cmp(&Version::parse(&right.version).unwrap())
  });
  Ok(manifests)
}
fn render_manifest(manifest: &ReleaseManifest) -> Result<String> {
  Ok(serde_saphyr::to_string_with_options(
    manifest,
    serde_saphyr::ser_options! {
      compact_list_indent: false,
    },
  )?)
}

fn render_changelog(configuration: &Configuration, state: &State) -> String {
  let preamble = "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).";
  let mut lines = vec![
    preamble.to_owned(),
    String::new(),
    "## [Unreleased]".to_owned(),
    String::new(),
  ];
  if let Some(baseline) = state
    .manifests
    .iter()
    .find(|manifest| manifest.baseline == Some(true))
  {
    lines.push(format!(
      "<!-- Arcantry release baseline: {} ({}). Earlier history is not reconstructed. -->",
      baseline.version, baseline.date
    ));
    lines.push(String::new());
  }
  for manifest in state
    .manifests
    .iter()
    .rev()
    .filter(|manifest| manifest.baseline != Some(true))
  {
    let mut grouped: BTreeMap<&str, Vec<(&str, &Artifact)>> = BTreeMap::new();
    for id in &manifest.changes {
      let artifact = &state.archived[id];
      if artifact.visibility == "public" {
        grouped
          .entry(&artifact.category)
          .or_default()
          .push((id, artifact));
      }
    }
    if grouped.is_empty() {
      continue;
    }
    lines.push(format!("## [{}] - {}", manifest.version, manifest.date));
    lines.push(String::new());
    for (category, heading) in [
      ("added", "Added"),
      ("changed", "Changed"),
      ("deprecated", "Deprecated"),
      ("removed", "Removed"),
      ("fixed", "Fixed"),
      ("security", "Security"),
    ] {
      if let Some(entries) = grouped.get(category) {
        lines.push(format!("### {heading}"));
        lines.push(String::new());
        for (id, artifact) in entries {
          lines.push(format!("<!-- openspec: {id} -->"));
          lines.push(format!("#### {}", artifact.title));
          lines.push(String::new());
          lines.push(artifact.body.clone());
          lines.push(String::new());
        }
      }
    }
  }
  let mut result = format!("{}\n", lines.join("\n").trim_end());
  if let (Some(url), Some(latest)) = (&configuration.repository_url, state.manifests.last()) {
    result.push('\n');
    result.push_str(&format!(
      "[Unreleased]: {}/compare/{}{}...HEAD\n",
      url.trim_end_matches('/'),
      configuration.tag_prefix,
      latest.version
    ));
  }
  result
}

fn read_version(root: &Path, path: &str, adapter: &str) -> Result<String> {
  let content = fs::read_to_string(root.join(path))?;
  if adapter == "json-package@1" {
    let value: serde_json::Value = serde_json::from_str(&content)?;
    return value["version"]
      .as_str()
      .map(str::to_owned)
      .context(format!(
        "JSON version source has no full SemVer version: {path}"
      ));
  }
  if adapter == "cargo-workspace@1" {
    let document = content.parse::<toml_edit::DocumentMut>()?;
    return document["workspace"]["package"]["version"]
      .as_str()
      .map(str::to_owned)
      .context(format!(
        "Cargo workspace package version must be full SemVer: {path}"
      ));
  }
  bail!("Unsupported release version adapter: {adapter}")
}
fn update_version(root: &Path, path: &str, adapter: &str, version: &str) -> Result<String> {
  let content = fs::read_to_string(root.join(path))?;
  if adapter == "json-package@1" {
    let mut value: serde_json::Value = serde_json::from_str(&content)?;
    value["version"] = serde_json::Value::String(version.to_owned());
    return Ok(format!(
      "{}{}",
      serde_json::to_string_pretty(&value)?,
      if content.ends_with('\n') { "\n" } else { "" }
    ));
  }
  if adapter == "cargo-workspace@1" {
    let mut document = content.parse::<toml_edit::DocumentMut>()?;
    document["workspace"]["package"]["version"] = toml_edit::value(version);
    return Ok(document.to_string());
  }
  bail!("Unsupported release version adapter: {adapter}")
}
fn validate_versions(configuration: &Configuration, expected: &str) -> Result<()> {
  for (path, adapter) in &configuration.version_sources {
    let actual = read_version(&configuration.root, path, adapter)?;
    if actual != expected {
      bail!("Version source must match {expected}: {path} contains {actual}.");
    }
  }
  Ok(())
}
fn validate_date(date: &str) -> Result<()> {
  if date.len() != 10 || NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
    bail!("Invalid release date: {date}.");
  }
  Ok(())
}
fn highest_impact(values: &[&str]) -> &'static str {
  for impact in ["major", "minor", "patch"] {
    if values.contains(&impact) {
      return impact;
    }
  }
  "none"
}
fn bump(current: &str, impact: &str) -> Result<String> {
  let mut version = Version::parse(current)?;
  match impact {
    "major" => {
      version.major += 1;
      version.minor = 0;
      version.patch = 0;
    }
    "minor" => {
      version.minor += 1;
      version.patch = 0;
    }
    "patch" => version.patch += 1,
    _ => {}
  }
  Ok(version.to_string())
}
fn path_visibility(root: &Path, path: &str) -> Visibility {
  let path = Path::new(path);
  if path.is_absolute() && !path.starts_with(root) {
    return Visibility::Private;
  }
  let normalized = path.to_string_lossy().replace('\\', "/");
  if normalized == ".local" || normalized.starts_with(".local/") {
    Visibility::Private
  } else {
    Visibility::Shared
  }
}
