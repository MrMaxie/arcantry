use crate::native_targets::{self, NativeTarget, TARGETS};
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::Builder;

#[derive(Debug, Deserialize)]
struct PackFile {
  path: String,
}

#[derive(Debug, Deserialize)]
struct PackResult {
  filename: Option<String>,
  #[serde(default)]
  files: Vec<PackFile>,
}

#[derive(Debug, Serialize)]
struct ManifestEntry {
  file: String,
  sha256: String,
}

pub fn package(
  root: &Path,
  output: &Path,
  include_main: bool,
  artifacts: Option<&Path>,
  binaries: &[String],
) -> Result<()> {
  if artifacts.is_some() && !binaries.is_empty() {
    bail!("--artifacts conflicts with --binary");
  }
  if artifacts.is_none() && binaries.is_empty() && !include_main {
    bail!("at least one --binary or --main input is required");
  }

  let root = absolute(root)?;
  let output = absolute(output)?;
  let artifacts = artifacts.map(absolute).transpose()?;
  let binaries = if let Some(artifact_root) = artifacts.as_deref() {
    TARGETS
      .iter()
      .map(|target| {
        (
          target,
          native_targets::downloaded_binary(artifact_root, target),
        )
      })
      .collect::<Vec<_>>()
  } else {
    binaries
      .iter()
      .map(|value| parse_binary(value))
      .collect::<Result<Vec<_>>>()?
  };

  fs::create_dir_all(&output)
    .with_context(|| format!("failed to create package output {}", output.display()))?;
  let temporary = Builder::new().prefix("arcantry-native-pack-").tempdir()?;
  let mut archives = Vec::new();

  for (target, binary) in binaries {
    let source = root.join("packages").join(target.package_directory);
    let stage = temporary.path().join(target.package_directory);
    fs::create_dir_all(stage.join("bin"))?;
    copy(&source.join("package.json"), &stage.join("package.json"))?;
    copy(&source.join("README.md"), &stage.join("README.md"))?;
    copy(&root.join("LICENSE"), &stage.join("LICENSE"))?;
    copy(&binary, &stage.join("bin").join(target.executable))?;
    assert_platform_package(&stage, target.executable)?;
    archives.push(pack(&stage, &output)?);
  }

  if include_main {
    let main = root.join("packages/arcantry");
    assert_main_package(&main)?;
    archives.push(pack(&main, &output)?);
  }

  archives.sort();
  let manifest = archives
    .iter()
    .map(|archive| {
      let bytes = fs::read(output.join(archive))?;
      Ok(ManifestEntry {
        file: archive.clone(),
        sha256: Sha256::digest(bytes)
          .iter()
          .map(|byte| format!("{byte:02x}"))
          .collect(),
      })
    })
    .collect::<Result<Vec<_>>>()?;
  fs::write(
    output.join("native-packages.json"),
    format!("{}\n", serde_json::to_string_pretty(&manifest)?),
  )?;
  println!(
    "Packed {} npm archive(s) into {}.",
    archives.len(),
    output.display()
  );
  Ok(())
}

fn parse_binary(value: &str) -> Result<(&'static NativeTarget, PathBuf)> {
  let Some((triple, path)) = value.split_once('=') else {
    bail!("invalid --binary value {value}. Expected <target>=<path>.");
  };
  if triple.is_empty() || path.is_empty() {
    bail!("invalid --binary value {value}. Expected <target>=<path>.");
  }
  Ok((native_targets::find(triple)?, absolute(Path::new(path))?))
}

fn absolute(path: &Path) -> Result<PathBuf> {
  if path.is_absolute() {
    Ok(path.to_path_buf())
  } else {
    Ok(std::env::current_dir()?.join(path))
  }
}

fn copy(source: &Path, destination: &Path) -> Result<()> {
  fs::copy(source, destination).with_context(|| {
    format!(
      "failed to copy {} to {}",
      source.display(),
      destination.display()
    )
  })?;
  Ok(())
}

fn pack(package_root: &Path, output: &Path) -> Result<String> {
  let result = run_nub(
    package_root,
    [
      OsString::from("pack"),
      OsString::from("--json"),
      OsString::from("--ignore-scripts"),
      OsString::from("--pack-destination"),
      output.as_os_str().to_owned(),
    ],
  )?;
  parse_pack(&result.stdout)?
    .first()
    .and_then(|entry| entry.filename.as_deref())
    .and_then(|filename| Path::new(filename).file_name())
    .map(|filename| filename.to_string_lossy().into_owned())
    .with_context(|| {
      format!(
        "nub pack returned no archive for {}",
        package_root.display()
      )
    })
}

fn dry_run_files(package_root: &Path) -> Result<Vec<String>> {
  let result = run_nub(
    package_root,
    ["pack", "--dry-run", "--json", "--ignore-scripts"].map(OsString::from),
  )?;
  Ok(
    parse_pack(&result.stdout)?
      .first()
      .map(|entry| entry.files.iter().map(|file| file.path.clone()).collect())
      .unwrap_or_default(),
  )
}

fn run_nub<const N: usize>(package_root: &Path, args: [OsString; N]) -> Result<Output> {
  let clean_environment = std::env::vars_os().filter(|(key, _)| key != "NODE_OPTIONS");
  let output = Command::new("nub")
    .args(args)
    .current_dir(package_root)
    .env_clear()
    .envs(clean_environment)
    .output()
    .with_context(|| format!("failed to run nub in {}", package_root.display()))?;
  if !output.status.success() {
    bail!(
      "nub pack failed in {} with {}: {}",
      package_root.display(),
      output.status,
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  Ok(output)
}

fn parse_pack(stdout: &[u8]) -> Result<Vec<PackResult>> {
  serde_json::from_slice(stdout).context("nub pack returned invalid JSON")
}

fn assert_platform_package(package_root: &Path, executable: &str) -> Result<()> {
  let actual = dry_run_files(package_root)?
    .into_iter()
    .collect::<BTreeSet<_>>();
  let expected = [
    "LICENSE".to_owned(),
    "README.md".to_owned(),
    format!("bin/{executable}"),
    "package.json".to_owned(),
  ]
  .into_iter()
  .collect::<BTreeSet<_>>();
  if actual != expected {
    bail!(
      "platform package allowlist mismatch. Expected {}, received {}.",
      expected.into_iter().collect::<Vec<_>>().join(", "),
      actual.into_iter().collect::<Vec<_>>().join(", ")
    );
  }
  Ok(())
}

fn assert_main_package(package_root: &Path) -> Result<()> {
  let manifest: Value = serde_json::from_slice(&fs::read(package_root.join("package.json"))?)?;
  if manifest.pointer("/bin/arcantry").and_then(Value::as_str) != Some("bin/arcantry.js") {
    bail!("main package must expose bin/arcantry.js as the arcantry launcher");
  }
  for field in ["exports", "types", "main", "module", "dependencies"] {
    if manifest.get(field).is_some() {
      bail!("launcher-only main package must not declare {field}");
    }
  }
  let files = dry_run_files(package_root)?;
  let allowed_files = ["package.json", "catalog.json", "README.md", "LICENSE"];
  let allowed_prefixes = [
    ".claude-plugin/",
    ".codex-plugin/",
    "bin/",
    "skills/",
    "assets/",
    "contracts/",
    "schemas/",
  ];
  let required_files = [
    ".claude-plugin/plugin.json",
    ".codex-plugin/plugin.json",
    "bin/arcantry.js",
  ];
  let unexpected = files
    .iter()
    .filter(|path| {
      !allowed_files.contains(&path.as_str())
        && !allowed_prefixes
          .iter()
          .any(|prefix| path.starts_with(prefix))
    })
    .map(|path| format!("Unexpected main package file: {path}"));
  let missing = required_files
    .iter()
    .filter(|path| !files.iter().any(|file| file == **path))
    .map(|path| format!("Missing required main package file: {path}"));
  let forbidden = files
    .iter()
    .filter(|path| forbidden_main_path(path))
    .map(|path| format!("Forbidden main package file: {path}"));
  let errors = (files.is_empty())
    .then_some("Main package dry run returned no files.".to_owned())
    .into_iter()
    .chain(unexpected)
    .chain(missing)
    .chain(forbidden)
    .collect::<Vec<_>>();
  if !errors.is_empty() {
    bail!(errors.join("\n"));
  }
  Ok(())
}

fn forbidden_main_path(path: &str) -> bool {
  let lower = path.to_ascii_lowercase();
  let segments = lower.split('/').collect::<Vec<_>>();
  matches!(
    segments.first().copied(),
    Some(".local" | "openspec" | "node_modules" | "src" | "scripts")
  ) || segments.contains(&"__pycache__")
    || lower.ends_with(".pyc")
    || lower.ends_with(".log")
    || segments
      .last()
      .is_some_and(|name| *name == ".env" || name.starts_with(".env."))
    || lower.contains("backup")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_repeated_binary_input() {
    let (target, path) = parse_binary("x86_64-pc-windows-msvc=target/debug/arcantry.exe").unwrap();
    assert_eq!(target.package_name, "@arcantry/cli-win32-x64");
    assert!(path.ends_with("target/debug/arcantry.exe"));
  }

  #[test]
  fn rejects_invalid_binary_input() {
    assert!(parse_binary("x86_64-pc-windows-msvc").is_err());
    assert!(parse_binary("=binary").is_err());
    assert!(parse_binary("x86_64-pc-windows-msvc=").is_err());
    assert!(parse_binary("unknown=binary").is_err());
  }

  #[test]
  fn classifies_forbidden_main_package_paths() {
    for path in [
      ".local/config.toml",
      "src/index.ts",
      "nested/__pycache__/file",
      "debug.log",
      ".env.production",
      "backup.json",
    ] {
      assert!(forbidden_main_path(path), "{path}");
    }
    assert!(!forbidden_main_path("bin/arcantry.js"));
  }
}
