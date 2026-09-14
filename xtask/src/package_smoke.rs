use crate::ci_setup;
use crate::native_targets;
use crate::package_native;
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::Builder;

#[derive(Debug, Deserialize)]
struct ManifestEntry {
  file: String,
}

pub fn smoke(
  root: &Path,
  binary: Option<&Path>,
  target: Option<&str>,
  build_root: Option<&Path>,
  retained_output: Option<&Path>,
) -> Result<()> {
  if binary.is_none() && target.is_none() {
    bail!("one of --binary or --target is required");
  }
  let root = absolute(root)?;
  let host = native_targets::host()?;
  let target = target
    .map(native_targets::find)
    .transpose()?
    .unwrap_or(host);
  if target.triple != host.triple {
    bail!(
      "package smoke target {} does not match host {}",
      target.triple,
      host.triple
    );
  }
  let binary = match binary {
    Some(path) => absolute(path)?,
    None => native_targets::built_binary(
      &absolute(build_root.unwrap_or_else(|| Path::new("target")))?,
      target,
    ),
  };
  let retained_output = retained_output.map(absolute).transpose()?;
  let temporary = Builder::new().prefix("arcantry-package-smoke-").tempdir()?;
  let archive_root = retained_output.unwrap_or_else(|| temporary.path().join("archives"));
  let install_root = temporary.path().join("install");
  fs::create_dir_all(&archive_root)?;
  fs::create_dir_all(&install_root)?;

  package_native::package(
    &root,
    &archive_root,
    true,
    None,
    &[format!("{}={}", target.triple, binary.display())],
  )?;

  let manifest: Vec<ManifestEntry> =
    serde_json::from_slice(&fs::read(archive_root.join("native-packages.json"))?)?;
  let package_manifest: Value =
    serde_json::from_slice(&fs::read(root.join("packages/arcantry/package.json"))?)?;
  let version = package_manifest["version"]
    .as_str()
    .context("main package manifest has no version")?;
  let main_prefix = format!("arcantry-{version}");
  let main_archive = manifest
    .iter()
    .find(|entry| entry.file.starts_with(&main_prefix))
    .map(|entry| entry.file.as_str())
    .context("native npm package set has no main package")?;
  let platform_archive = manifest
    .iter()
    .find(|entry| entry.file != main_archive)
    .map(|entry| entry.file.as_str())
    .context("native npm package set has no platform package")?;

  let mut dependencies = Map::new();
  dependencies.insert(
    "arcantry".to_owned(),
    Value::String(format!(
      "file:{}",
      archive_root.join(main_archive).display()
    )),
  );
  dependencies.insert(
    target.package_name.to_owned(),
    Value::String(format!(
      "file:{}",
      archive_root.join(platform_archive).display()
    )),
  );
  fs::write(
    install_root.join("package.json"),
    format!(
      "{}\n",
      serde_json::to_string_pretty(&json!({
        "name": "arcantry-native-package-smoke",
        "private": true,
        "dependencies": dependencies,
      }))?
    ),
  )?;
  run("nub", ["install", "--ignore-scripts"], &install_root)?;

  let package_root = install_root.join("node_modules/arcantry");
  let cli = package_root.join("bin/arcantry.js");
  let node = ci_setup::nub_node_executable()?;
  let version_output = run(
    &node,
    [cli.as_os_str(), OsStr::new("--version")],
    &install_root,
  )?;
  let actual_version = reported_version(&version_output)?;
  if actual_version != version {
    bail!("packed native CLI reported {actual_version}, expected {version}");
  }
  for (runner, arguments) in package_runners() {
    let output = run(runner, arguments, &install_root)?;
    let actual = reported_version(&output)?;
    if actual != version {
      bail!("{runner} reported {actual}, expected {version} from the packed native CLI");
    }
  }

  let subpaths = [
    "arcantry",
    "arcantry/catalog",
    "arcantry/repository",
    "arcantry/project",
    "arcantry/release",
  ];
  let import_expression = format!(
    "await Promise.all({}.map(async (specifier) => {{ let exposed = false; try {{ await import(specifier); exposed = true; }} catch {{}} if (exposed) throw new Error(`${{specifier}} unexpectedly exposes a JavaScript API`); }}));",
    serde_json::to_string(&subpaths)?
  );
  run(
    &node,
    [
      OsStr::new("--input-type=module"),
      OsStr::new("-e"),
      OsStr::new(&import_expression),
    ],
    &install_root,
  )?;
  run(
    &node,
    [
      cli.as_os_str(),
      OsStr::new("--cwd"),
      install_root.as_os_str(),
      OsStr::new("repo"),
      OsStr::new("inspect"),
      OsStr::new("--json"),
    ],
    &install_root,
  )?;
  fs::remove_dir_all(
    install_root.join("node_modules").join(
      target
        .package_name
        .replace('/', std::path::MAIN_SEPARATOR_STR),
    ),
  )?;
  let missing = run_failure(
    &node,
    [cli.as_os_str(), OsStr::new("--version")],
    &install_root,
  )?;
  let missing = String::from_utf8_lossy(&missing.stderr);
  if !missing.contains("is missing")
    || !missing.contains("Reinstall arcantry with optional dependencies enabled")
    || !missing.contains("GitHub Release archive")
  {
    bail!("missing platform package did not produce actionable launcher guidance: {missing}");
  }
  println!("Native npm package smoke passed for {}.", target.triple);
  Ok(())
}

fn run_failure<I, S>(
  command: impl AsRef<OsStr>,
  args: I,
  cwd: &Path,
) -> Result<std::process::Output>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let clean_environment = std::env::vars_os().filter(|(key, _)| key != "NODE_OPTIONS");
  let output = Command::new(command)
    .args(args)
    .current_dir(cwd)
    .env_clear()
    .envs(clean_environment)
    .output()
    .with_context(|| {
      format!(
        "failed to execute package smoke command in {}",
        cwd.display()
      )
    })?;
  if output.status.success() {
    bail!("package smoke command unexpectedly succeeded");
  }
  Ok(output)
}

fn run<I, S>(command: impl AsRef<OsStr>, args: I, cwd: &Path) -> Result<Vec<u8>>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let clean_environment = std::env::vars_os().filter(|(key, _)| key != "NODE_OPTIONS");
  let program = platform_program(command.as_ref());
  let output = Command::new(&program)
    .args(args)
    .current_dir(cwd)
    .env_clear()
    .envs(clean_environment)
    .output()
    .with_context(|| {
      format!(
        "failed to execute package smoke command in {}",
        cwd.display()
      )
    })?;
  if !output.status.success() {
    bail!(
      "package smoke command exited with {}: {}",
      output.status,
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  Ok(output.stdout)
}

fn package_runners() -> Vec<(&'static str, Vec<&'static OsStr>)> {
  vec![
    (
      "npm",
      vec![
        OsStr::new("exec"),
        OsStr::new("--offline"),
        OsStr::new("--"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "npx",
      vec![
        OsStr::new("--offline"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "pnpm",
      vec![
        OsStr::new("exec"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "bun",
      vec![
        OsStr::new("run"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
    (
      "nub",
      vec![
        OsStr::new("exec"),
        OsStr::new("arcantry"),
        OsStr::new("--version"),
      ],
    ),
  ]
}

fn reported_version(output: &[u8]) -> Result<String> {
  String::from_utf8(output.to_vec())?
    .lines()
    .rev()
    .map(str::trim)
    .find(|line| !line.is_empty())
    .map(str::to_owned)
    .context("package runner produced no version output")
}

fn platform_program(command: &OsStr) -> OsString {
  #[cfg(windows)]
  if ["npm", "npx", "pnpm"]
    .iter()
    .any(|candidate| command == OsStr::new(candidate))
  {
    let mut program = command.to_os_string();
    program.push(".cmd");
    return program;
  }
  command.to_os_string()
}

fn absolute(path: &Path) -> Result<PathBuf> {
  if path.is_absolute() {
    Ok(path.to_path_buf())
  } else {
    Ok(std::env::current_dir()?.join(path))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn reads_the_version_after_package_manager_progress() {
    assert_eq!(
      reported_version(b"Recreating node_modules\nProgress: resolved 2\n\n1.0.0\n").unwrap(),
      "1.0.0"
    );
  }

  #[test]
  fn rejects_a_target_that_does_not_match_the_host() {
    let other = native_targets::TARGETS
      .iter()
      .find(|target| target.triple != native_targets::host().unwrap().triple)
      .unwrap();
    let error = smoke(Path::new("."), None, Some(other.triple), None, None).unwrap_err();
    assert!(error.to_string().contains("does not match host"));
  }

  #[test]
  fn requires_a_binary_or_target() {
    let error = smoke(Path::new("."), None, None, None, None).unwrap_err();
    assert_eq!(error.to_string(), "one of --binary or --target is required");
  }
}
