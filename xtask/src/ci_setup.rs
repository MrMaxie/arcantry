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
}
