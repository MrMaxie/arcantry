use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

const TARGETS: [&str; 6] = [
  "x86_64-pc-windows-msvc",
  "aarch64-pc-windows-msvc",
  "x86_64-apple-darwin",
  "aarch64-apple-darwin",
  "x86_64-unknown-linux-musl",
  "aarch64-unknown-linux-musl",
];

pub fn target_binary(root: &Path, target: &str) -> Result<PathBuf> {
  if !TARGETS.contains(&target) {
    bail!("unsupported native target: {target}");
  }
  let executable = if target.ends_with("windows-msvc") {
    "arcantry.exe"
  } else {
    "arcantry"
  };
  Ok(root.join(target).join("dist").join(executable))
}

pub fn verify_binary(path: PathBuf, expected_version: &str) -> Result<()> {
  if !path.is_file() {
    bail!("native executable does not exist: {}", path.display());
  }

  let output = Command::new(&path)
    .arg("--version")
    .output()
    .with_context(|| format!("failed to execute {}", path.display()))?;
  if !output.status.success() {
    bail!(
      "{} --version exited with {}: {}",
      path.display(),
      output.status,
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }

  let actual = String::from_utf8(output.stdout)
    .context("native executable returned non-UTF-8 version output")?;
  if actual.trim() != expected_version {
    bail!(
      "{} reported version {:?}, expected {:?}",
      path.display(),
      actual.trim(),
      expected_version
    );
  }

  println!(
    "Verified {} reports version {}.",
    path.display(),
    expected_version
  );
  Ok(())
}
