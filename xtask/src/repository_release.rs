use anyhow::{Context, Result, bail};
use arcantry_core::config::{
  Management, PROJECT_CONFIG_VERSION, ProjectConfig, RawSourceConfig, ReleaseConfig,
  ReleaseTopology, ReleaseVersionSource, ResolvedProject, SourceKind, Visibility,
};
use arcantry_core::path_security::ReadAuthority;
use arcantry_core::project_plan::{
  ApplyAuthority, ProjectPlan, create_write_operation, hash_content,
};
use chrono::Local;
use clap::Subcommand;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Subcommand)]
pub enum Command {
  /// Show unassigned archived changes and the resulting SemVer bump.
  Plan {
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Create the next release manifest, versions, and changelog.
  Cut {
    #[arg(long, default_value_t = Local::now().format("%Y-%m-%d").to_string())]
    date: String,
    #[arg(long)]
    apply: bool,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Regenerate CHANGELOG.md from release manifests and archived changes.
  Render {
    #[arg(long)]
    apply: bool,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Check the repository release state without publishing.
  Check {
    #[arg(long)]
    sealed: bool,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
}

pub fn run(command: Command) -> Result<()> {
  match command {
    Command::Plan { root } => {
      let project = project(&root)?;
      println!(
        "{}",
        serde_json::to_string_pretty(&arcantry_core::release::inspect(&project, None)?)?
      );
      Ok(())
    }
    Command::Cut { date, apply, root } => {
      let project = project(&root)?;
      let release = arcantry_core::release::inspect(&project, None)?;
      let mut plan = arcantry_core::release::cut(&project, &date, None)?;
      augment_repository_versions(&root, &release.current, &release.next, &mut plan)?;
      if !apply {
        return preview(&plan);
      }
      apply_plan(&plan)?;
      println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
          "version": release.next,
          "date": date,
          "changes": release.changes,
        }))?
      );
      Ok(())
    }
    Command::Render { apply, root } => {
      let project = project(&root)?;
      let plan = arcantry_core::release::render(&project, None)?;
      if apply {
        apply_plan(&plan)
      } else {
        preview(&plan)
      }
    }
    Command::Check { sealed, root } => {
      let project = project(&root)?;
      arcantry_core::release::check_with_pull_request_head(
        &project,
        sealed,
        None,
        arcantry_core::release::github_pull_request_head().as_deref(),
      )
    }
  }
}

pub(crate) fn project(root: &Path) -> Result<ResolvedProject> {
  let root = std::fs::canonicalize(root)
    .with_context(|| format!("repository release root does not exist: {}", root.display()))?;
  let mut sources = BTreeMap::new();
  sources.insert(
    "openspec".to_owned(),
    RawSourceConfig {
      kind: SourceKind::Openspec,
      path: "openspec".to_owned(),
      management: Management::Observe,
      adapter: "openspec@1".to_owned(),
      from: Vec::new(),
      managed_from: None,
      visibility: Some(Visibility::Shared),
      scope: "root".to_owned(),
    },
  );
  sources.insert(
    "changelog".to_owned(),
    RawSourceConfig {
      kind: SourceKind::Changelog,
      path: "CHANGELOG.md".to_owned(),
      management: Management::Manage,
      adapter: "keep-a-changelog@2".to_owned(),
      from: vec!["openspec".to_owned()],
      managed_from: None,
      visibility: Some(Visibility::Shared),
      scope: "root".to_owned(),
    },
  );
  let version_sources = std::iter::once(ReleaseVersionSource {
    path: "Cargo.toml".to_owned(),
    adapter: "cargo-workspace@1".to_owned(),
  })
  .chain(
    [
      "packages/arcantry/package.json",
      "packages/cli-win32-x64/package.json",
      "packages/cli-darwin-x64/package.json",
      "packages/cli-darwin-arm64/package.json",
      "packages/cli-linux-x64/package.json",
      ".codex-plugin/plugin.json",
      ".claude-plugin/plugin.json",
    ]
    .map(|path| ReleaseVersionSource {
      path: path.to_owned(),
      adapter: "json-package@1".to_owned(),
    }),
  )
  .collect();
  Ok(ResolvedProject {
    root,
    config_path: None,
    config: Some(ProjectConfig {
      config_version: PROJECT_CONFIG_VERSION,
      workflow: None,
      context: None,
      schema_reference: None,
      tool: None,
      project: None,
      sources,
      release: Some(ReleaseConfig {
        version_strategy: Default::default(),
        changelog_template: None,
        adapter: "openspec-release@1".to_owned(),
        topology: ReleaseTopology::Single,
        manifests_path: Some("releases".to_owned()),
        changelog_source: Some("changelog".to_owned()),
        tag_prefix: Some("v".to_owned()),
        repository_url: None,
        version_sources,
        units: BTreeMap::new(),
      }),
      extra: BTreeMap::new(),
    }),
    mode: "repository-tooling",
    scope: Some("shared"),
    shadowed_config_paths: Vec::new(),
    allow_external_paths: false,
  })
}

fn augment_repository_versions(
  root: &Path,
  current: &str,
  next: &str,
  plan: &mut ProjectPlan,
) -> Result<()> {
  let authority = ReadAuthority::new(root)?;
  let package_path = "packages/arcantry/package.json";
  let package = plan
    .operations
    .iter_mut()
    .find(|operation| operation.path == package_path)
    .and_then(|operation| operation.content.as_mut())
    .context("repository release plan is missing the main npm package")?;
  for name in [
    "@arcantry/cli-win32-x64",
    "@arcantry/cli-darwin-x64",
    "@arcantry/cli-darwin-arm64",
    "@arcantry/cli-linux-x64",
  ] {
    replace_once(
      package,
      &format!("\"{name}\": \"{current}\""),
      &format!("\"{name}\": \"{next}\""),
      package_path,
    )?;
  }
  if let Some(operation) = plan
    .operations
    .iter_mut()
    .find(|operation| operation.path == package_path)
  {
    operation.content_hash = operation.content.as_deref().map(hash_content);
  }

  for path in ["crates/arcantry-cli/Cargo.toml", "xtask/Cargo.toml"] {
    let mut content = authority.read_to_string(&root.join(path))?;
    replace_once(
      &mut content,
      &format!("arcantry-core = {{ version = \"={current}\""),
      &format!("arcantry-core = {{ version = \"={next}\""),
      path,
    )?;
    plan.operations.push(create_write_operation(
      root,
      path,
      content,
      Visibility::Shared,
    )?);
  }

  let lock_path = "Cargo.lock";
  let mut lock = authority.read_to_string(&root.join(lock_path))?;
  for package in ["arcantry-cli", "arcantry-core", "xtask"] {
    update_lock_package(&mut lock, package, current, next)?;
  }
  plan.operations.push(create_write_operation(
    root,
    lock_path,
    lock,
    Visibility::Shared,
  )?);
  Ok(())
}

fn replace_once(content: &mut String, old: &str, new: &str, path: &str) -> Result<()> {
  let matches = content.matches(old).count();
  if matches != 1 {
    bail!("Expected one release version pin in {path}, found {matches}.");
  }
  *content = content.replacen(old, new, 1);
  Ok(())
}

fn update_lock_package(
  content: &mut String,
  package: &str,
  current: &str,
  next: &str,
) -> Result<()> {
  let marker = format!("name = \"{package}\"\nversion = \"{current}\"");
  let replacement = format!("name = \"{package}\"\nversion = \"{next}\"");
  replace_once(content, &marker, &replacement, "Cargo.lock")
}

fn preview(plan: &ProjectPlan) -> Result<()> {
  if !plan.conflicts.is_empty() {
    bail!(plan.conflicts.join("; "));
  }
  print!("{}", arcantry_core::project_plan::serialize(plan)?);
  Ok(())
}

fn apply_plan(plan: &ProjectPlan) -> Result<()> {
  arcantry_core::project_plan::apply(plan, &ApplyAuthority::new(&plan.root)?)?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn write_release_fixture(root: &Path) {
    let change = root.join("openspec/changes/archive/2026-08-16-release-history");
    std::fs::create_dir_all(&change).unwrap();
    std::fs::write(
      change.join("release.md"),
      "---\ncategory: changed\nimpact: minor\nvisibility: public\ncomponents:\n  - repository-lifecycle\n---\n\n# Better release history\n\nRelease notes come from delivered changes rather than commit messages.\n",
    )
    .unwrap();
    for path in [
      "packages/arcantry/package.json",
      "packages/cli-win32-x64/package.json",
      "packages/cli-darwin-x64/package.json",
      "packages/cli-darwin-arm64/package.json",
      "packages/cli-linux-x64/package.json",
      ".codex-plugin/plugin.json",
      ".claude-plugin/plugin.json",
    ] {
      let path = root.join(path);
      std::fs::create_dir_all(path.parent().unwrap()).unwrap();
      std::fs::write(path, "{\"version\":\"0.0.0\"}\n").unwrap();
    }
    std::fs::write(
      root.join("packages/arcantry/package.json"),
      "{\n  \"version\": \"0.0.0\",\n  \"optionalDependencies\": {\n    \"@arcantry/cli-win32-x64\": \"0.0.0\",\n    \"@arcantry/cli-darwin-x64\": \"0.0.0\",\n    \"@arcantry/cli-darwin-arm64\": \"0.0.0\",\n    \"@arcantry/cli-linux-x64\": \"0.0.0\"\n  }\n}\n",
    )
    .unwrap();
    std::fs::write(
      root.join("Cargo.toml"),
      "[workspace]\n\n[workspace.package]\nversion = \"0.0.0\"\n",
    )
    .unwrap();
    for path in ["crates/arcantry-cli/Cargo.toml", "xtask/Cargo.toml"] {
      let path = root.join(path);
      std::fs::create_dir_all(path.parent().unwrap()).unwrap();
      std::fs::write(
        path,
        "[dependencies]\narcantry-core = { version = \"=0.0.0\", path = \"../core\" }\n",
      )
      .unwrap();
    }
    std::fs::write(
      root.join("Cargo.lock"),
      "[[package]]\nname = \"arcantry-cli\"\nversion = \"0.0.0\"\n\n[[package]]\nname = \"arcantry-core\"\nversion = \"0.0.0\"\n\n[[package]]\nname = \"xtask\"\nversion = \"0.0.0\"\n",
    )
    .unwrap();
  }

  #[test]
  fn repository_adapter_preserves_the_legacy_release_paths() {
    let directory = tempfile::tempdir().unwrap();
    let project = project(directory.path()).unwrap();
    let config = project.config.unwrap();
    let release = config.release.unwrap();
    assert_eq!(release.adapter, "openspec-release@1");
    assert_eq!(release.manifests_path.as_deref(), Some("releases"));
    assert_eq!(release.changelog_source.as_deref(), Some("changelog"));
    assert_eq!(release.version_sources.len(), 8);
    assert_eq!(config.sources["openspec"].path, "openspec");
    assert_eq!(config.sources["changelog"].path, "CHANGELOG.md");
  }

  #[test]
  fn repository_commands_use_the_rust_release_owner() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().to_path_buf();
    write_release_fixture(&root);

    run(Command::Render {
      apply: true,
      root: root.clone(),
    })
    .unwrap();
    run(Command::Check {
      sealed: false,
      root: root.clone(),
    })
    .unwrap();
    run(Command::Plan { root: root.clone() }).unwrap();
    run(Command::Cut {
      date: "2026-08-16".to_owned(),
      apply: false,
      root: root.clone(),
    })
    .unwrap();

    assert!(root.join("CHANGELOG.md").is_file());
    assert!(!root.join("releases/0.1.0.yaml").exists());
  }

  #[test]
  fn repository_release_plan_aligns_every_product_version() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().to_path_buf();
    write_release_fixture(&root);
    let project = project(&root).unwrap();
    let release = arcantry_core::release::inspect(&project, None).unwrap();
    let mut plan = arcantry_core::release::cut(&project, "2026-10-03", None).unwrap();

    augment_repository_versions(&root, &release.current, &release.next, &mut plan).unwrap();

    let paths = plan
      .operations
      .iter()
      .map(|operation| operation.path.as_str())
      .collect::<std::collections::BTreeSet<_>>();
    for path in [
      "Cargo.toml",
      "Cargo.lock",
      "crates/arcantry-cli/Cargo.toml",
      "xtask/Cargo.toml",
      "packages/arcantry/package.json",
      "packages/cli-win32-x64/package.json",
      "packages/cli-darwin-x64/package.json",
      "packages/cli-darwin-arm64/package.json",
      "packages/cli-linux-x64/package.json",
      ".codex-plugin/plugin.json",
      ".claude-plugin/plugin.json",
    ] {
      assert!(paths.contains(path), "missing version operation for {path}");
    }
    let main = plan
      .operations
      .iter()
      .find(|operation| operation.path == "packages/arcantry/package.json")
      .and_then(|operation| operation.content.as_deref())
      .unwrap();
    assert!(main.contains("\"@arcantry/cli-linux-x64\": \"0.1.0\""));
    for path in [
      "packages/arcantry/package.json",
      "packages/cli-win32-x64/package.json",
      "packages/cli-darwin-x64/package.json",
      "packages/cli-darwin-arm64/package.json",
      "packages/cli-linux-x64/package.json",
      ".codex-plugin/plugin.json",
      ".claude-plugin/plugin.json",
    ] {
      let content = plan
        .operations
        .iter()
        .find(|operation| operation.path == path)
        .and_then(|operation| operation.content.as_deref())
        .unwrap();
      let value: serde_json::Value = serde_json::from_str(content).unwrap();
      assert_eq!(value["version"], "0.1.0", "{path}");
    }
    for path in ["crates/arcantry-cli/Cargo.toml", "xtask/Cargo.toml"] {
      let content = plan
        .operations
        .iter()
        .find(|operation| operation.path == path)
        .and_then(|operation| operation.content.as_deref())
        .unwrap();
      assert!(
        content.contains("arcantry-core = { version = \"=0.1.0\""),
        "{path}"
      );
    }
    let lock = plan
      .operations
      .iter()
      .find(|operation| operation.path == "Cargo.lock")
      .and_then(|operation| operation.content.as_deref())
      .unwrap();
    assert_eq!(lock.matches("version = \"0.1.0\"").count(), 3);
  }
}
