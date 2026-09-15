use anyhow::{Result, bail};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeTarget {
  pub triple: &'static str,
  pub os: &'static str,
  pub cpu: &'static str,
  pub package_directory: &'static str,
  pub package_name: &'static str,
  pub executable: &'static str,
  pub archive: &'static str,
}

pub const TARGETS: [NativeTarget; 4] = [
  NativeTarget {
    triple: "x86_64-pc-windows-msvc",
    os: "win32",
    cpu: "x64",
    package_directory: "cli-win32-x64",
    package_name: "@arcantry/cli-win32-x64",
    executable: "arcantry.exe",
    archive: "arcantry-cli-x86_64-pc-windows-msvc.zip",
  },
  NativeTarget {
    triple: "x86_64-apple-darwin",
    os: "darwin",
    cpu: "x64",
    package_directory: "cli-darwin-x64",
    package_name: "@arcantry/cli-darwin-x64",
    executable: "arcantry",
    archive: "arcantry-cli-x86_64-apple-darwin.tar.xz",
  },
  NativeTarget {
    triple: "aarch64-apple-darwin",
    os: "darwin",
    cpu: "arm64",
    package_directory: "cli-darwin-arm64",
    package_name: "@arcantry/cli-darwin-arm64",
    executable: "arcantry",
    archive: "arcantry-cli-aarch64-apple-darwin.tar.xz",
  },
  NativeTarget {
    triple: "x86_64-unknown-linux-musl",
    os: "linux",
    cpu: "x64",
    package_directory: "cli-linux-x64",
    package_name: "@arcantry/cli-linux-x64",
    executable: "arcantry",
    archive: "arcantry-cli-x86_64-unknown-linux-musl.tar.xz",
  },
];

pub fn find(triple: &str) -> Result<&'static NativeTarget> {
  TARGETS
    .iter()
    .find(|target| target.triple == triple)
    .ok_or_else(|| anyhow::anyhow!("unsupported native target: {triple}"))
}

pub fn host() -> Result<&'static NativeTarget> {
  let os = match std::env::consts::OS {
    "windows" => "win32",
    "macos" => "darwin",
    "linux" => "linux",
    other => bail!("unsupported package host operating system: {other}"),
  };
  let cpu = match std::env::consts::ARCH {
    "x86_64" => "x64",
    "aarch64" => "arm64",
    other => bail!("unsupported package host architecture: {other}"),
  };
  TARGETS
    .iter()
    .find(|target| target.os == os && target.cpu == cpu)
    .ok_or_else(|| anyhow::anyhow!("unsupported package smoke host: {os}-{cpu}"))
}

pub fn downloaded_binary(root: &Path, target: &NativeTarget) -> PathBuf {
  root
    .join(format!("native-{}", target.triple))
    .join(target.triple)
    .join("dist")
    .join(target.executable)
}

pub fn built_binary(root: &Path, target: &NativeTarget) -> PathBuf {
  root
    .join(target.triple)
    .join("dist")
    .join(target.executable)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn target_metadata_is_unique_and_complete() {
    assert_eq!(TARGETS.len(), 4);
    assert_eq!(
      TARGETS
        .iter()
        .map(|target| target.triple)
        .collect::<HashSet<_>>()
        .len(),
      TARGETS.len()
    );
    assert!(
      TARGETS
        .iter()
        .all(|target| target.archive.contains(target.triple))
    );
  }

  #[test]
  fn derives_downloaded_and_built_binary_paths() {
    let target = find("x86_64-pc-windows-msvc").unwrap();
    assert_eq!(
      downloaded_binary(Path::new("artifacts"), target),
      Path::new("artifacts")
        .join("native-x86_64-pc-windows-msvc")
        .join("x86_64-pc-windows-msvc")
        .join("dist")
        .join("arcantry.exe")
    );
    assert_eq!(
      built_binary(Path::new("target"), target),
      Path::new("target")
        .join("x86_64-pc-windows-msvc")
        .join("dist")
        .join("arcantry.exe")
    );
  }
}
