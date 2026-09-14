use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use testcontainers::GenericBuildableImage;
use testcontainers::core::{CmdWaitFor, ExecCommand};
use testcontainers::runners::{SyncBuilder, SyncRunner};

const IMAGE_NAME: &str = "arcantry-rust-cli-system-test";
const IMAGE_TAG: &str = "1.0.0";

pub fn run() -> Result<()> {
  let root = workspace_root()?;
  let dockerfile = root.join("containers/rust-cli-test/Dockerfile");
  let mut image = GenericBuildableImage::new(IMAGE_NAME, IMAGE_TAG).with_dockerfile(&dockerfile);
  // Git's source inventory excludes private/local state and build output.
  let inventory = std::process::Command::new("git")
    .args([
      "ls-files",
      "--cached",
      "--others",
      "--exclude-standard",
      "-z",
    ])
    .current_dir(&root)
    .output()?;
  if !inventory.status.success() {
    bail!("Cannot inventory Linux test inputs.");
  }
  for relative in std::str::from_utf8(&inventory.stdout)?
    .split('\0')
    .filter(|p| !p.is_empty())
  {
    if relative
      .split('/')
      .any(|part| part.eq_ignore_ascii_case(".local"))
    {
      continue;
    }
    let source = root.join(relative);
    if source.is_file() {
      image = image.with_file(source, relative);
    }
  }

  let runnable = image
    .build_image()
    .context("Docker could not build the pinned Linux system-test image.")?;
  let container = runnable
    .start()
    .context("Docker could not start the Linux system-test container.")?;
  let command = [
    "cargo run -p xtask -- prepare-package",
    "cargo clippy --workspace --all-targets --locked -- -D warnings",
    "cargo test --workspace --locked",
    "cargo build -p arcantry-cli",
    "test \"$(target/debug/arcantry --version)\" = \"1.0.0\"",
    "target/debug/arcantry --help > /tmp/arcantry-help.txt",
    "grep -q \"Usage: arcantry\" /tmp/arcantry-help.txt",
  ]
  .join(" && ");
  let mut execution = container
    .exec(ExecCommand::new(["sh", "-c", &command]).with_cmd_ready_condition(CmdWaitFor::exit()))
    .context("The Linux system-test command could not be executed.")?;
  let stdout = String::from_utf8_lossy(&execution.stdout_to_vec()?).into_owned();
  let stderr = String::from_utf8_lossy(&execution.stderr_to_vec()?).into_owned();
  let exit_code = execution.exit_code()?;
  print!("{stdout}");
  eprint!("{stderr}");
  if exit_code != Some(0) {
    bail!("Linux system test failed with exit code {exit_code:?}.");
  }
  Ok(())
}

fn workspace_root() -> Result<PathBuf> {
  let root = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .context("xtask has no workspace parent directory.")?;
  std::fs::canonicalize(root).context("Could not resolve the workspace root.")
}
