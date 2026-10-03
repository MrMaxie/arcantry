use crate::tooling;
use anyhow::{Context, Result, bail};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn configure() -> Result<()> {
  let github_path = env::var_os("GITHUB_PATH").context("GITHUB_PATH is required for CI setup.")?;
  let nub = tooling::mise_program(Path::new("."), "nub")?;
  let node_executable = nub_node_executable(&nub)?;
  append_node_directory(Path::new(&github_path), Path::new(&node_executable))?;
  println!("Added Nub's Node directory to GITHUB_PATH.");
  Ok(())
}

pub(crate) fn nub_node_executable(nub: &Path) -> Result<String> {
  let output = Command::new(nub)
    .args(["--node", "-p", "process.execPath"])
    .output()
    .context("CI setup could not run Nub's Node runtime")?;
  if !output.status.success() {
    bail!(
      "CI setup could not resolve Nub's Node runtime: {}",
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  let path = String::from_utf8(output.stdout)?;
  let path = path.trim();
  if path.is_empty() {
    bail!("CI setup received an empty Node executable path from Nub");
  }
  Ok(path.to_owned())
}

fn append_node_directory(github_path: &Path, node_executable: &Path) -> Result<()> {
  let node_directory = node_executable
    .parent()
    .context("Nub's Node executable does not have a parent directory")?;
  let mut output = OpenOptions::new()
    .create(true)
    .append(true)
    .open(github_path)
    .with_context(|| format!("could not open GITHUB_PATH at {}", github_path.display()))?;
  writeln!(output, "{}", node_directory.display())?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  fn ci_workflow() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    std::fs::read_to_string(root.join(".github/workflows/ci.yml"))
      .unwrap()
      .replace("\r\n", "\n")
  }

  fn release_workflow() -> String {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    std::fs::read_to_string(root.join(".github/workflows/release.yml"))
      .unwrap()
      .replace("\r\n", "\n")
  }

  #[test]
  fn appends_the_node_directory_to_github_path() {
    let root = tempfile::tempdir().unwrap();
    let github_path = root.path().join("github-path");
    let node_executable = root.path().join("nub/node/bin/node");
    append_node_directory(&github_path, &node_executable).unwrap();

    let content = std::fs::read_to_string(github_path).unwrap();
    assert_eq!(
      content.trim_end(),
      node_executable.parent().unwrap().display().to_string()
    );
  }

  #[test]
  fn ci_runs_host_and_linux_gates_in_parallel_before_the_required_check() {
    let workflow = ci_workflow();
    let host_job = workflow
      .split("\n  host:\n")
      .nth(1)
      .unwrap()
      .split("\n  linux-system-test:\n")
      .next()
      .unwrap();
    let linux_job = workflow
      .split("\n  linux-system-test:\n")
      .nth(1)
      .unwrap()
      .split("\n  check:\n")
      .next()
      .unwrap();
    let required_check = workflow.split("\n  check:\n").nth(1).unwrap();

    assert!(host_job.contains("timeout-minutes: 15"));
    assert!(
      host_job.contains("install_args: just nub pnpm aqua:oven-sh/bun rust cargo:cargo-deny")
    );
    assert!(host_job.contains("run: just openspec-validate check-host"));
    assert!(!host_job.contains("linux-system-test"));
    for unused in ["cargo:cargo-dist", "cargo:cargo-llvm-cov"] {
      assert!(
        !host_job.contains(unused),
        "host bootstrap installs {unused}"
      );
    }

    assert!(linux_job.contains("timeout-minutes: 15"));
    assert!(linux_job.contains("install_args: just rust"));
    assert!(linux_job.contains("run: just linux-system-test"));
    assert!(!linux_job.contains("ci-setup"));
    assert!(!linux_job.contains("check-host"));

    assert!(required_check.contains("name: check"));
    assert!(required_check.contains("needs: [host, linux-system-test]"));
    assert!(required_check.contains("if: ${{ always() }}"));
    assert!(required_check.contains("timeout-minutes: 5"));
    assert!(required_check.contains("needs.host.result"));
    assert!(required_check.contains("needs.linux-system-test.result"));
  }

  #[test]
  fn release_native_jobs_install_only_required_tools() {
    let workflow = release_workflow();
    let native_job = workflow
      .split("\n  native:\n")
      .nth(1)
      .unwrap()
      .split("\n  assemble:\n")
      .next()
      .unwrap();

    assert!(native_job.contains("install_args: just nub rust cargo:cargo-dist"));
    for unused in [
      "pnpm",
      "aqua:oven-sh/bun",
      "cargo:cargo-deny",
      "cargo:cargo-llvm-cov",
    ] {
      assert!(
        !native_job.contains(unused),
        "native release bootstrap installs {unused}"
      );
    }
  }
}
