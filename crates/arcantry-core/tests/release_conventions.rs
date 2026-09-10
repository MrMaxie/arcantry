use arcantry_core::{
  config::resolve_project,
  project_plan::{ApplyAuthority, apply},
  release,
};
use std::fs;

#[test]
fn numbered_and_calendar_releases_preserve_history_and_render_project_templates() {
  for (strategy, current, next) in [
    ("integer", "9", "10"),
    ("calendar", "2026.09.01.1", "2026.09.02.1"),
  ] {
    let root = tempfile::tempdir().unwrap();
    fs::write(
      root.path().join("arcantry.toml"),
      format!(
        r#"config_version = 1
[sources.openspec]
kind = "openspec"
path = "openspec"
adapter = "openspec@1"
[sources.changelog]
kind = "changelog"
path = "CHANGELOG.md"
adapter = "keep-a-changelog@2"
management = "manage"
from = ["openspec"]
[release]
adapter = "openspec-release@1"
version_strategy = "{strategy}"
changelog_template = "changelog.jinja"
manifests_path = "releases"
changelog_source = "changelog"
[[release.version_sources]]
path = "VERSION"
adapter = "text-version@1"
"#
      ),
    )
    .unwrap();
    fs::write(root.path().join("VERSION"), format!("{current}\r\n")).unwrap();
    let history = "# Previous project notes\r\n\r\nKeep these exact bytes.\r\n";
    fs::write(root.path().join("CHANGELOG.md"), history).unwrap();
    fs::write(root.path().join("changelog.jinja"), "# Releases\n{% for release in releases %}## {{ release.version }}\n{% for change in release.changes %}{% for outcome in change.outcomes %}- {{ outcome.title }}\n{% endfor %}{% endfor %}{% endfor %}").unwrap();
    let config_path = root.path().join("arcantry.toml");
    let configuration = fs::read_to_string(&config_path).unwrap();
    fs::write(
      &config_path,
      configuration.replace("text-version@1", "cargo-workspace@1"),
    )
    .unwrap();
    assert!(resolve_project(root.path(), None, true, None).is_err());
    fs::write(
      &config_path,
      configuration.replace("changelog.jinja", ".local/private.jinja"),
    )
    .unwrap();
    let private_template_project = resolve_project(root.path(), None, true, None).unwrap();
    assert!(
      !release::baseline(&private_template_project, current, "2026-09-01", None)
        .unwrap()
        .conflicts
        .is_empty()
    );
    fs::write(&config_path, configuration).unwrap();
    let project = resolve_project(root.path(), None, true, None).unwrap();
    let authority = ApplyAuthority::new(root.path()).unwrap();
    let baseline = release::baseline(&project, current, "2026-09-01", None).unwrap();
    assert!(baseline.conflicts.is_empty(), "{:?}", baseline.conflicts);
    apply(&baseline, &authority).unwrap();
    let archive = root
      .path()
      .join("openspec/changes/archive/2026-09-02-useful-guidance");
    fs::create_dir_all(&archive).unwrap();
    fs::write(archive.join("release.md"), "---\ncategory: added\nvisibility: public\ncomponents: [cli]\n---\n# Useful guidance\n\nFind the next action.\n").unwrap();
    let cut = release::cut(&project, "2026-09-02", None).unwrap();
    assert!(cut.conflicts.is_empty(), "{:?}", cut.conflicts);
    fs::write(root.path().join("changelog.jinja"), "changed input").unwrap();
    assert!(apply(&cut, &authority).is_err());
    assert_eq!(
      fs::read_to_string(root.path().join("VERSION")).unwrap(),
      format!("{current}\r\n")
    );
    fs::write(root.path().join("changelog.jinja"), "# Releases\n{% for release in releases %}## {{ release.version }}\n{% for change in release.changes %}{% for outcome in change.outcomes %}- {{ outcome.title }}\n{% endfor %}{% endfor %}{% endfor %}").unwrap();
    apply(&cut, &authority).unwrap();
    assert_eq!(
      fs::read_to_string(root.path().join("VERSION")).unwrap(),
      format!("{next}\r\n")
    );
    let changelog = fs::read_to_string(root.path().join("CHANGELOG.md")).unwrap();
    assert!(changelog.contains("Useful guidance"));
    assert!(changelog.ends_with(history));
    assert!(
      release::render(&project, None)
        .unwrap()
        .operations
        .is_empty()
    );
    release::check(&project, false, None).unwrap();
  }
}
