use super::*;

fn release_artifact(impact: &str, visibility: &str, title: &str) -> String {
  format!(
    "---\ncategory: changed\nimpact: {impact}\nvisibility: {visibility}\ncomponents:\n  - repository-lifecycle\n---\n\n# {title}\n\nRelease notes come from delivered changes rather than commit messages.\n"
  )
}

fn legacy_configuration(root: &Path) -> Configuration {
  Configuration {
    root: root.to_path_buf(),
    releases: "releases".to_owned(),
    changelog: "CHANGELOG.md".to_owned(),
    changelog_visibility: Visibility::Shared,
    adapter: "openspec-release@1".to_owned(),
    topology: ReleaseTopology::Single,
    unit: None,
    openspec: vec![("openspec".to_owned(), "openspec".to_owned())],
    selectors: Vec::new(),
    dependencies: BTreeMap::new(),
    repository_url: None,
    tag_prefix: "v".to_owned(),
    version_sources: Vec::new(),
  }
}

fn archive_change(root: &Path, openspec: &str, dated_id: &str, artifact: &str) {
  let change = root.join(openspec).join("changes/archive").join(dated_id);
  fs::create_dir_all(&change).unwrap();
  fs::write(change.join("release.md"), artifact).unwrap();
}

fn write_manifest(root: &Path, version: &str, date: &str, changes: &[&str]) {
  fs::create_dir_all(root.join("releases")).unwrap();
  let changes = changes
    .iter()
    .map(|change| format!("  - {change}"))
    .collect::<Vec<_>>()
    .join("\n");
  fs::write(
    root.join("releases").join(format!("{version}.yaml")),
    format!("version: {version}\ndate: {date}\nchanges:\n{changes}\n"),
  )
  .unwrap();
}

fn write_json_version(root: &Path, path: &str, version: &str) {
  let path = root.join(path);
  fs::create_dir_all(path.parent().unwrap()).unwrap();
  fs::write(path, format!("{{\"version\":\"{version}\"}}\n")).unwrap();
}

#[test]
fn parses_and_validates_legacy_release_artifacts() {
  let parsed = parse_artifact(
    &release_artifact("minor", "public", "Better release history"),
    false,
  )
  .unwrap();
  assert_eq!(parsed.impact, "minor");
  assert_eq!(parsed.visibility, "public");
  assert_eq!(parsed.components, ["repository-lifecycle"]);
  assert_eq!(parsed.outcomes[0].category, "changed");
  assert_eq!(parsed.outcomes[0].title, "Better release history");
  assert_eq!(
    parsed.outcomes[0].body,
    "Release notes come from delivered changes rather than commit messages."
  );

  let invalid_impact = release_artifact("feature", "public", "Invalid impact");
  assert!(
    parse_artifact(&invalid_impact, false)
      .unwrap_err()
      .to_string()
      .contains("invalid release impact")
  );
  let invalid_component = release_artifact("minor", "public", "Invalid component")
    .replace("repository-lifecycle", "Repository Lifecycle");
  assert!(
    parse_artifact(&invalid_component, false)
      .unwrap_err()
      .to_string()
      .contains("visibility or components")
  );
  let missing_components = release_artifact("minor", "public", "Missing components")
    .replace("components:\n  - repository-lifecycle\n", "");
  assert!(parse_artifact(&missing_components, false).is_err());

  let whitespace_title = release_artifact("minor", "public", &" ".repeat(100_000));
  assert!(
    parse_artifact(&whitespace_title, false)
      .unwrap_err()
      .to_string()
      .contains("level-one title")
  );
}

#[test]
fn parses_canonical_consumer_outcomes_and_rejects_ambiguous_structures() {
  let frontmatter = "---\nimpact: minor\nvisibility: public\ncomponents:\n  - cli\n---";
  let source = format!(
    "{frontmatter}\n\n## Added\n\n### Add reusable exports\n\nProjects can export selected release data.\n\n### Add readable previews\n\nRelease plans show exact exported content.\n\n## Fixed\n\n### Preserve empty values\n\nExports retain intentionally empty fields.\n"
  );
  let artifact = parse_artifact(&source, false).unwrap();
  assert_eq!(artifact.impact, "minor");
  assert_eq!(artifact.visibility, "public");
  assert_eq!(artifact.components, ["cli"]);
  assert_eq!(artifact.outcomes.len(), 3);
  assert_eq!(artifact.outcomes[0].category, "added");
  assert_eq!(artifact.outcomes[0].title, "Add reusable exports");
  assert_eq!(artifact.outcomes[1].title, "Add readable previews");
  assert_eq!(artifact.outcomes[2].category, "fixed");
  assert_eq!(artifact.outcomes[2].title, "Preserve empty values");

  for (body, message) in [
    (
      "---\ncategory: changed\nimpact: patch\nvisibility: public\ncomponents:\n  - cli\n---\n\n## Added\n\n### Add exports\n\nProjects can export data.",
      "level-one title",
    ),
    (
      "---\nimpact: patch\nvisibility: public\ncomponents:\n  - cli\n---\n\n## Improved\n\n### Improve exports\n\nProjects can export data.",
      "invalid release category heading",
    ),
    (
      "---\nimpact: patch\nvisibility: public\ncomponents:\n  - cli\n---\n\n## Added\n\n### Add exports\n\nProjects can export data.\n\n## Added\n\n### Add previews\n\nProjects can preview data.",
      "duplicate release category heading",
    ),
    (
      "---\nimpact: patch\nvisibility: public\ncomponents:\n  - cli\n---\n\nRelease overview.\n\n## Added\n\n### Add exports\n\nProjects can export data.",
      "must use level-two categories",
    ),
    (
      "---\nimpact: patch\nvisibility: public\ncomponents:\n  - cli\n---\n\n## Added",
      "release category must contain an outcome",
    ),
    (
      "---\nimpact: patch\nvisibility: public\ncomponents:\n  - cli\n---\n\n## Added\n\n### Add exports",
      "release outcome must describe",
    ),
  ] {
    assert!(
      parse_artifact(body, false)
        .unwrap_err()
        .to_string()
        .contains(message),
      "expected parser error containing {message}"
    );
  }
}

#[test]
fn renders_every_consumer_outcome_with_shared_change_provenance() {
  let directory = tempfile::tempdir().unwrap();
  let root = directory.path();
  archive_change(
    root,
    "openspec",
    "2026-09-01-export-release-data",
    "---\nimpact: minor\nvisibility: public\ncomponents:\n  - cli\n---\n\n## Added\n\n### Add reusable exports\n\nProjects can export selected release data.\n\n### Add readable previews\n\nRelease plans show exact exported content.\n\n## Fixed\n\n### Preserve empty values\n\nExports retain intentionally empty fields.\n",
  );
  write_manifest(root, "1.1.0", "2026-09-01", &["export-release-data"]);
  let configuration = legacy_configuration(root);
  let changelog = render_changelog(&configuration, &state(&configuration).unwrap());
  assert!(
    changelog
      .contains("### Added\n\n<!-- openspec: export-release-data -->\n#### Add reusable exports")
  );
  assert!(changelog.contains("#### Add readable previews"));
  assert!(
    changelog
      .contains("### Fixed\n\n<!-- openspec: export-release-data -->\n#### Preserve empty values")
  );
  assert_eq!(
    changelog
      .matches("<!-- openspec: export-release-data -->")
      .count(),
    3
  );
}

#[test]
fn plans_the_highest_semver_impact() {
  assert_eq!(highest_impact(&["patch", "major", "minor"]), "major");
  assert_eq!(bump("1.4.7", "patch").unwrap(), "1.4.8");
  assert_eq!(bump("1.4.7", "minor").unwrap(), "1.5.0");
  assert_eq!(bump("1.4.7", "major").unwrap(), "2.0.0");
}

#[test]
fn plans_only_unassigned_archived_changes() {
  let directory = tempfile::tempdir().unwrap();
  let root = directory.path();
  archive_change(
    root,
    "openspec",
    "2026-08-16-release-history",
    &release_artifact("minor", "public", "Better release history"),
  );
  let configuration = legacy_configuration(root);

  let plan = inspect_configuration(&configuration).unwrap();
  assert_eq!(plan.current, "0.0.0");
  assert_eq!(plan.next, "0.1.0");
  assert_eq!(plan.impact, "minor");
  assert_eq!(plan.changes, ["release-history"]);

  write_manifest(root, "0.1.0", "2026-08-16", &["release-history"]);
  let assigned = inspect_configuration(&configuration).unwrap();
  assert_eq!(assigned.current, "0.1.0");
  assert_eq!(assigned.next, "0.1.0");
  assert_eq!(assigned.impact, "none");
  assert!(assigned.changes.is_empty());

  fs::remove_dir_all(root.join("releases")).unwrap();
  archive_change(
    root,
    "openspec",
    "2026-08-16-release-history",
    &release_artifact("none", "public", "Better release history"),
  );
  assert!(
    inspect_configuration(&configuration)
      .unwrap_err()
      .to_string()
      .contains("completed changes must declare a SemVer impact")
  );
}

#[test]
fn rejects_unknown_and_duplicate_release_assignments() {
  let directory = tempfile::tempdir().unwrap();
  let root = directory.path();
  archive_change(
    root,
    "openspec",
    "2026-08-16-release-history",
    &release_artifact("minor", "public", "Better release history"),
  );
  let configuration = legacy_configuration(root);

  write_manifest(root, "0.1.0", "2026-08-16", &["missing-change"]);
  assert!(
    state(&configuration)
      .unwrap_err()
      .to_string()
      .contains("references unknown archived change")
  );

  write_manifest(root, "0.1.0", "2026-08-16", &["release-history"]);
  write_manifest(root, "0.1.1", "2026-08-17", &["release-history"]);
  assert!(
    state(&configuration)
      .unwrap_err()
      .to_string()
      .contains("assigned more than once")
  );
}

#[test]
fn renders_public_changes_from_every_openspec_source() {
  let directory = tempfile::tempdir().unwrap();
  let root = directory.path();
  archive_change(
    root,
    "openspec",
    "2026-08-16-release-history",
    &release_artifact("minor", "public", "Better release history"),
  );
  archive_change(
    root,
    "openspec",
    "2026-08-17-internal-cleanup",
    &release_artifact("patch", "internal", "Internal cleanup"),
  );
  archive_change(
    root,
    "components/api/openspec",
    "2026-08-17-api-change",
    &release_artifact("patch", "public", "API change"),
  );
  write_manifest(
    root,
    "0.1.0",
    "2026-08-18",
    &["release-history", "internal-cleanup", "api-change"],
  );
  let mut configuration = legacy_configuration(root);
  configuration
    .openspec
    .push(("api".to_owned(), "components/api/openspec".to_owned()));
  let changelog = render_changelog(&configuration, &state(&configuration).unwrap());

  assert!(changelog.contains("https://keepachangelog.com/en/2.0.0/"));
  assert!(changelog.contains("## [Unreleased]"));
  assert!(changelog.contains("## [0.1.0] - 2026-08-18"));
  assert!(changelog.contains("<!-- openspec: release-history -->"));
  assert!(changelog.contains("#### Better release history"));
  assert!(changelog.contains("#### API change"));
  assert!(!changelog.contains("Internal cleanup"));
}

#[test]
fn validates_distribution_versions_and_the_rendered_changelog() {
  let directory = tempfile::tempdir().unwrap();
  let root = directory.path();
  archive_change(
    root,
    "openspec",
    "2026-08-16-release-history",
    &release_artifact("minor", "public", "Better release history"),
  );
  write_manifest(root, "0.1.0", "2026-08-16", &["release-history"]);
  let mut configuration = legacy_configuration(root);
  configuration.version_sources = [
    "packages/arcantry/package.json",
    ".codex-plugin/plugin.json",
    ".claude-plugin/plugin.json",
  ]
  .map(|path| (path.to_owned(), "json-package@1".to_owned()))
  .to_vec();
  for (path, _) in &configuration.version_sources {
    write_json_version(root, path, "0.1.0");
  }
  write_json_version(root, ".claude-plugin/plugin.json", "0.2.0");
  assert!(
    check_configuration(&configuration, false, None)
      .unwrap_err()
      .to_string()
      .contains("Version source must match 0.1.0")
  );

  write_json_version(root, ".claude-plugin/plugin.json", "0.1.0");
  fs::write(root.join("CHANGELOG.md"), "# Changelog\n").unwrap();
  assert!(
    check_configuration(&configuration, false, None)
      .unwrap_err()
      .to_string()
      .contains("CHANGELOG.md is stale")
  );
  fs::write(
    root.join("CHANGELOG.md"),
    render_changelog(&configuration, &state(&configuration).unwrap()),
  )
  .unwrap();
  check_configuration(&configuration, false, None).unwrap();
}

#[test]
fn sealed_checks_reject_active_and_unassigned_changes_before_git() {
  let directory = tempfile::tempdir().unwrap();
  let root = directory.path();
  archive_change(
    root,
    "openspec",
    "2026-08-16-release-history",
    &release_artifact("minor", "public", "Better release history"),
  );
  let configuration = legacy_configuration(root);
  fs::write(
    root.join("CHANGELOG.md"),
    render_changelog(&configuration, &state(&configuration).unwrap()),
  )
  .unwrap();
  assert!(
    check_configuration(&configuration, true, None)
      .unwrap_err()
      .to_string()
      .contains("archived OpenSpec changes are not assigned")
  );

  write_manifest(root, "0.1.0", "2026-08-16", &["release-history"]);
  fs::write(
    root.join("CHANGELOG.md"),
    render_changelog(&configuration, &state(&configuration).unwrap()),
  )
  .unwrap();
  fs::create_dir_all(root.join("openspec/changes/in-progress")).unwrap();
  assert!(
    check_configuration(&configuration, true, None)
      .unwrap_err()
      .to_string()
      .contains("active OpenSpec changes are not release-complete")
  );
}
