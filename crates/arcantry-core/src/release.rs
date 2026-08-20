use crate::config::{ResolvedProject, Visibility, effective_visibility, is_private_project_path};
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
    parse_stable_version(version)?;
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
    let unassigned = unassigned_changes(&state);
    if !unassigned.is_empty() {
      bail!(
        "archived OpenSpec changes are not assigned to a release: {}",
        unassigned.join(", ")
      );
    }
    validate_git_seal(&configuration, &state)?;
  }
  Ok(())
}

fn unassigned_changes(state: &State) -> Vec<String> {
  state
    .archived
    .keys()
    .filter(|id| !state.assigned.contains(*id))
    .cloned()
    .collect()
}

fn validate_git_seal(configuration: &Configuration, state: &State) -> Result<()> {
  let latest = state
    .manifests
    .last()
    .context("release sealing requires at least one release manifest")?;
  let status = duct::cmd("git", ["status", "--porcelain=v1", "--untracked-files=all"])
    .dir(&configuration.root)
    .read()?;
  if !status.trim().is_empty() {
    bail!("repository contains unsealed working tree changes");
  }
  let manifest_path = format!(
    "{}/{}.yaml",
    configuration.releases.trim_end_matches(['/', '\\']),
    latest.version
  );
  let manifest_commit = duct::cmd(
    "git",
    [
      "log",
      "--diff-filter=A",
      "-1",
      "--format=%H",
      "--",
      &manifest_path,
    ],
  )
  .dir(&configuration.root)
  .read()?;
  if manifest_commit.trim().is_empty() {
    bail!("latest release manifest is not committed: {manifest_path}");
  }
  let head = duct::cmd("git", ["rev-parse", "HEAD"])
    .dir(&configuration.root)
    .read()?;
  if head.trim() != manifest_commit.trim() {
    bail!(
      "repository HEAD is not sealed by release {}",
      latest.version
    );
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
    let version = parse_stable_version(&manifest.version)?;
    validate_date(&manifest.date)?;
    if manifest.baseline == Some(true) && !manifest.changes.is_empty() {
      bail!("baseline release cannot assign changes");
    }
    if manifest.baseline != Some(true) && manifest.changes.is_empty() {
      bail!("non-baseline release must assign changes");
    }
    manifests.push((version, manifest));
  }
  manifests.sort_by(|(left, _), (right, _)| left.cmp(right));
  let manifests = manifests
    .into_iter()
    .map(|(_, manifest)| manifest)
    .collect::<Vec<_>>();
  let baselines = manifests
    .iter()
    .enumerate()
    .filter(|(_, manifest)| manifest.baseline == Some(true))
    .map(|(index, _)| index)
    .collect::<Vec<_>>();
  if baselines.len() > 1 {
    bail!("release history may contain only one baseline manifest");
  }
  if baselines.first().is_some_and(|index| *index != 0) {
    bail!("release baseline must be the oldest manifest");
  }
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
    let url = url.trim_end_matches('/');
    result.push('\n');
    result.push_str(&format!(
      "[Unreleased]: {}/compare/{}{}...HEAD\n",
      url, configuration.tag_prefix, latest.version
    ));
    for (index, manifest) in state.manifests.iter().enumerate() {
      if manifest.baseline == Some(true) {
        continue;
      }
      if let Some(previous) = index
        .checked_sub(1)
        .and_then(|value| state.manifests.get(value))
      {
        result.push_str(&format!(
          "[{}]: {}/compare/{}{}...{}{}\n",
          manifest.version,
          url,
          configuration.tag_prefix,
          previous.version,
          configuration.tag_prefix,
          manifest.version
        ));
      } else {
        result.push_str(&format!(
          "[{}]: {}/releases/tag/{}{}\n",
          manifest.version, url, configuration.tag_prefix, manifest.version
        ));
      }
    }
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
    let indentation = json_indentation(&content);
    let formatter = serde_json::ser::PrettyFormatter::with_indent(indentation.as_bytes());
    let mut bytes = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    value.serialize(&mut serializer)?;
    let mut rendered = String::from_utf8(bytes)?;
    let newline = if content.contains("\r\n") {
      "\r\n"
    } else {
      "\n"
    };
    if newline == "\r\n" {
      rendered = rendered.replace('\n', newline);
    }
    if content.ends_with('\n') {
      rendered.push_str(newline);
    }
    return Ok(rendered);
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

fn parse_stable_version(value: &str) -> Result<Version> {
  let version = Version::parse(value)
    .with_context(|| format!("Release version must be full stable SemVer: {value}."))?;
  if !version.pre.is_empty() || !version.build.is_empty() {
    bail!("Release version must be full stable SemVer: {value}.");
  }
  Ok(version)
}

fn json_indentation(content: &str) -> String {
  content
    .lines()
    .skip(1)
    .find_map(|line| {
      let trimmed = line.trim_start_matches([' ', '\t']);
      trimmed
        .starts_with('"')
        .then(|| line[..line.len() - trimmed.len()].to_owned())
    })
    .filter(|value| !value.is_empty())
    .unwrap_or_else(|| "  ".to_owned())
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
  if is_private_project_path(&path.to_string_lossy()) {
    Visibility::Private
  } else {
    Visibility::Shared
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn git(root: &Path, arguments: &[&str]) {
    duct::cmd("git", arguments).dir(root).run().unwrap();
  }

  fn release_configuration(root: &Path) -> Configuration {
    Configuration {
      root: root.to_path_buf(),
      releases: "releases".to_owned(),
      changelog: "CHANGELOG.md".to_owned(),
      changelog_visibility: Visibility::Shared,
      openspec: Vec::new(),
      repository_url: None,
      tag_prefix: "v".to_owned(),
      version_sources: Vec::new(),
    }
  }

  #[test]
  fn finds_unassigned_archived_changes() {
    let state = State {
      archived: BTreeMap::from([(
        "pending".to_owned(),
        Artifact {
          category: "fixed".to_owned(),
          impact: "patch".to_owned(),
          visibility: "public".to_owned(),
          title: "Pending".to_owned(),
          body: "Pending change.".to_owned(),
        },
      )]),
      manifests: Vec::new(),
      assigned: BTreeSet::new(),
    };

    assert_eq!(unassigned_changes(&state), ["pending"]);
  }

  #[test]
  fn rejects_non_stable_release_versions() {
    for version in ["1.0.0-alpha", "1.0.0+build"] {
      assert!(
        parse_stable_version(version)
          .unwrap_err()
          .to_string()
          .contains("full stable SemVer")
      );
    }
  }

  #[test]
  fn requires_one_oldest_release_baseline() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir(root.join("releases")).unwrap();
    fs::write(
      root.join("releases/1.0.0.yaml"),
      "version: 1.0.0\ndate: 2026-08-20\nchanges: []\nbaseline: true\n",
    )
    .unwrap();
    fs::write(
      root.join("releases/1.1.0.yaml"),
      "version: 1.1.0\ndate: 2026-08-21\nchanges: []\nbaseline: true\n",
    )
    .unwrap();
    let configuration = release_configuration(root);
    assert!(
      read_manifests(&configuration)
        .unwrap_err()
        .to_string()
        .contains("only one baseline")
    );

    fs::write(
      root.join("releases/1.0.0.yaml"),
      "version: 1.0.0\ndate: 2026-08-20\nchanges:\n  - first\n",
    )
    .unwrap();
    assert!(
      read_manifests(&configuration)
        .unwrap_err()
        .to_string()
        .contains("baseline must be the oldest")
    );
  }

  #[test]
  fn renders_version_comparison_links() {
    let mut configuration = release_configuration(Path::new("."));
    configuration.repository_url = Some("https://github.com/example/project/".to_owned());
    let state = State {
      archived: BTreeMap::from([(
        "new-cli".to_owned(),
        Artifact {
          category: "added".to_owned(),
          impact: "minor".to_owned(),
          visibility: "public".to_owned(),
          title: "Native CLI".to_owned(),
          body: "Run the native CLI.".to_owned(),
        },
      )]),
      manifests: vec![
        ReleaseManifest {
          version: "1.0.0".to_owned(),
          date: "2026-08-20".to_owned(),
          changes: Vec::new(),
          baseline: Some(true),
        },
        ReleaseManifest {
          version: "1.1.0".to_owned(),
          date: "2026-08-21".to_owned(),
          changes: vec!["new-cli".to_owned()],
          baseline: None,
        },
      ],
      assigned: BTreeSet::from(["new-cli".to_owned()]),
    };

    let changelog = render_changelog(&configuration, &state);
    assert!(
      changelog.contains("[Unreleased]: https://github.com/example/project/compare/v1.1.0...HEAD")
    );
    assert!(
      changelog.contains("[1.1.0]: https://github.com/example/project/compare/v1.0.0...v1.1.0")
    );
  }

  #[test]
  fn preserves_json_version_source_formatting() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let source =
      "{\r\n    \"z\": true,\r\n    \"version\": \"1.0.0\",\r\n    \"name\": \"fixture\"\r\n}\r\n";
    fs::write(root.join("package.json"), source).unwrap();

    assert_eq!(
      update_version(root, "package.json", "json-package@1", "1.1.0").unwrap(),
      source.replace("\"1.0.0\"", "\"1.1.0\"")
    );
  }

  #[test]
  fn requires_head_to_be_the_manifest_commit() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    fs::create_dir(root.join("releases")).unwrap();
    fs::write(root.join("releases/1.0.0.yaml"), "version: 1.0.0\n").unwrap();
    git(root, &["init", "--quiet"]);
    git(root, &["config", "user.name", "Arcantry Tests"]);
    git(root, &["config", "user.email", "tests@arcantry.invalid"]);
    git(root, &["add", "releases/1.0.0.yaml"]);
    git(root, &["commit", "--quiet", "-m", "release: add manifest"]);

    let state = State {
      archived: BTreeMap::new(),
      manifests: vec![ReleaseManifest {
        version: "1.0.0".to_owned(),
        date: "2026-08-20".to_owned(),
        changes: Vec::new(),
        baseline: Some(true),
      }],
      assigned: BTreeSet::new(),
    };
    let configuration = release_configuration(root);
    validate_git_seal(&configuration, &state).unwrap();

    fs::write(root.join("later.txt"), "later\n").unwrap();
    git(root, &["add", "later.txt"]);
    git(root, &["commit", "--quiet", "-m", "feat: add later work"]);

    assert!(
      validate_git_seal(&configuration, &state)
        .unwrap_err()
        .to_string()
        .contains("repository HEAD is not sealed")
    );
  }
}
