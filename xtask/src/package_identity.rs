use crate::native_targets::TARGETS;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .unwrap()
    .to_path_buf()
}

fn text(relative: &str) -> String {
  fs::read_to_string(root().join(relative)).unwrap()
}

fn json(relative: &str) -> Value {
  serde_json::from_str(&text(relative)).unwrap()
}

#[test]
fn uses_distinct_public_and_workspace_package_names() {
  let package = json("packages/arcantry/package.json");
  let workspace = json("package.json");
  assert_eq!(package["name"], "arcantry");
  assert_eq!(workspace["name"], "arcantry-workspace");
  assert_eq!(workspace["private"], true);
  assert_ne!(workspace["name"], package["name"]);
}

#[test]
fn removes_previous_npm_scopes_from_public_surfaces() {
  for relative in [
    "apps/docs/src/content/docs/getting-started.mdx",
    "apps/docs/src/content/docs/lifecycle/releases.mdx",
    "apps/docs/src/components/ArcantryCommandPicker.astro",
    "apps/docs/src/components/ArcantryAgentPrompt.astro",
    "apps/docs/src/components/ArcantryHero.astro",
    "apps/docs/src/components/ArcantryCopyCommands.astro",
    "packages/arcantry/bin/arcantry.js",
  ] {
    let source = text(relative);
    assert!(!source.contains("@maxiedev/"), "{relative}");
    assert!(!source.contains("@arcantry/arcantry"), "{relative}");
  }
}

#[test]
fn keeps_authored_installation_and_launcher_examples_aligned() {
  let picker = text("apps/docs/src/components/ArcantryCommandPicker.astro");
  let getting_started = text("apps/docs/src/content/docs/getting-started.mdx");
  let prompt = text("apps/docs/src/components/ArcantryAgentPrompt.astro");
  for expected in [
    "const packageName = packageManifest.name",
    "value: `npx ${packageName} repo inspect`",
    "value: `npm install --global ${packageName}`",
    "arcantry-installer.ps1",
    "arcantry-installer.sh",
    "href={releasePage}>Download</a>",
  ] {
    assert!(picker.contains(expected), "{expected}");
  }
  for forbidden in [
    "id: 'pnpm'",
    "id: 'nub'",
    "value: `pnpm dlx ${packageName} repo inspect`",
    "value: `nubx ${packageName} repo inspect`",
    "value: `bunx ${packageName} repo inspect`",
    "Install native CLI",
    "Run once",
  ] {
    assert!(!picker.contains(forbidden), "{forbidden}");
  }
  let ordered_choices = [
    "id: 'npx'",
    "id: 'npm'",
    "id: 'powershell'",
    "id: 'shell'",
    "class=\"download-link\"",
  ];
  for pair in ordered_choices.windows(2) {
    assert!(
      picker.find(pair[0]).unwrap() < picker.find(pair[1]).unwrap(),
      "{} must precede {}",
      pair[0],
      pair[1]
    );
  }
  assert!(getting_started.contains("cargo install --locked --path crates/arcantry-cli"));
  assert!(getting_started.contains("supported public distribution channels for 1.0"));
  assert!(getting_started.contains("<ArcantryAgentPrompt variant=\"full\" />"));
  assert!(
    prompt.contains("Install Arcantry on this computer using the official getting-started guide.")
  );
  assert!(
    prompt
      .contains("Do not adopt Arcantry into a repository or change project files unless I ask.")
  );
  for forbidden in ["Homebrew", "Scoop", "Winget", "Chocolatey", "pnpm", "Nub"] {
    assert!(!getting_started.contains(forbidden), "{forbidden}");
    assert!(!prompt.contains(forbidden), "{forbidden}");
  }
}

#[test]
fn keeps_agent_manifests_aligned_with_the_package_identity() {
  let package = json("packages/arcantry/package.json");
  for relative in [".codex-plugin/plugin.json", ".claude-plugin/plugin.json"] {
    let manifest = json(relative);
    for field in [
      "name",
      "version",
      "description",
      "author",
      "homepage",
      "license",
    ] {
      assert_eq!(manifest[field], package[field], "{relative}: {field}");
    }
    assert_eq!(
      manifest["repository"],
      package["repository"]["url"]
        .as_str()
        .unwrap()
        .trim_end_matches(".git"),
      "{relative}: repository"
    );
  }
  let codex = json(".codex-plugin/plugin.json");
  let claude = json(".claude-plugin/plugin.json");
  for field in ["interface", "skills"] {
    assert!(codex.get(field).is_some(), "Codex requires {field}");
    assert!(
      claude.get(field).is_none(),
      "Claude must not receive Codex-only {field}"
    );
  }
}

#[test]
fn main_package_declares_both_plugin_manifests() {
  let package = json("packages/arcantry/package.json");
  let files = package["files"].as_array().unwrap();
  for host in [".codex-plugin", ".claude-plugin"] {
    assert!(files.iter().any(|entry| entry == host), "{host}");
  }
}

#[test]
fn keeps_native_target_metadata_aligned_with_platform_packages() {
  let main = json("packages/arcantry/package.json");
  for target in TARGETS {
    let manifest = json(&format!(
      "packages/{}/package.json",
      target.package_directory
    ));
    assert_eq!(manifest["name"], target.package_name, "{}", target.triple);
    assert_eq!(manifest["version"], main["version"], "{}", target.triple);
    assert_eq!(
      manifest["os"],
      serde_json::json!([target.os]),
      "{}",
      target.triple
    );
    assert_eq!(
      manifest["cpu"],
      serde_json::json!([target.cpu]),
      "{}",
      target.triple
    );
    assert!(manifest.get("libc").is_none(), "{}", target.triple);
    assert_eq!(
      main["optionalDependencies"][target.package_name], main["version"],
      "{}",
      target.triple
    );
  }
}
