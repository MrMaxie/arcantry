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
    "value: `npm install --global ${packageName}`",
    "value: `npx ${packageName} repo inspect`",
    "value: `pnpm dlx ${packageName} repo inspect`",
    "value: `nubx ${packageName} repo inspect`",
    "arcantry-installer.ps1",
    "arcantry-installer.sh",
  ] {
    assert!(picker.contains(expected), "{expected}");
  }
  for forbidden in [
    "value: `bunx ${packageName} repo inspect`",
    "Install native CLI",
    "Run once",
  ] {
    assert!(!picker.contains(forbidden), "{forbidden}");
  }
  assert!(getting_started.contains("cargo install --locked --path crates/arcantry-cli"));
  assert!(getting_started.contains("public 1.0 package or GitHub Release"));
  assert!(getting_started.contains("<ArcantryAgentPrompt variant=\"full\" />"));
  assert!(
    prompt.contains("Install Arcantry on this computer using the official getting-started guide.")
  );
  assert!(
    prompt
      .contains("Do not adopt Arcantry into a repository or change project files unless I ask.")
  );
}

#[test]
fn keeps_agent_manifests_aligned_with_the_package_identity() {
  let package = json("packages/arcantry/package.json");
  for relative in [".codex-plugin/plugin.json", ".claude-plugin/plugin.json"] {
    let manifest = json(relative);
    assert_eq!(manifest["name"], package["name"], "{relative}");
    assert_eq!(manifest["version"], package["version"], "{relative}");
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
