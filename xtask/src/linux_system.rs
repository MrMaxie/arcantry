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
  let mut image = GenericBuildableImage::new(IMAGE_NAME, IMAGE_TAG)
    .with_dockerfile(&dockerfile)
    .with_file(root.join("Cargo.toml"), "Cargo.toml")
    .with_file(root.join("Cargo.lock"), "Cargo.lock")
    .with_file(root.join("catalog.json"), "catalog.json")
    .with_file(root.join("crates"), "crates")
    .with_file(root.join("xtask"), "xtask")
    .with_file(root.join("contracts"), "contracts")
    .with_file(
      root.join("apps/docs/src/content/docs"),
      "apps/docs/src/content/docs",
    )
    .with_file(root.join("skills"), "skills")
    .with_file(root.join("schemas"), "schemas")
    .with_file(
      root.join("openspec/schemas/arcantry"),
      "openspec/schemas/arcantry",
    );
  for optional in ["deny.toml", "LICENSE"] {
    let source = root.join(optional);
    if source.is_file() {
      image = image.with_file(source, optional);
    }
  }

  let runnable = image
    .build_image()
    .context("Docker could not build the pinned Linux system-test image.")?;
  let container = runnable
    .start()
    .context("Docker could not start the Linux system-test container.")?;
  let command = [
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo test -p arcantry-cli --test cli_contract",
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
