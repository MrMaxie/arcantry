use crate::binary::verify_binary;
use anyhow::{Context, Result};
use assert_cmd::Command;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

struct SmokeScenario {
  name: &'static str,
  arguments: Vec<OsString>,
}

impl SmokeScenario {
  fn new(name: &'static str, arguments: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
    Self {
      name,
      arguments: arguments.into_iter().map(Into::into).collect(),
    }
  }
}

pub fn smoke_binary(path: PathBuf) -> Result<()> {
  let path = path
    .canonicalize()
    .with_context(|| format!("native executable does not exist: {}", path.display()))?;
  verify_binary(path.clone(), "1.0.0")?;

  let temporary = tempfile::tempdir()?;
  let repository = temporary.path().join("repository");
  let links = temporary.path().join("links");
  let shims = temporary.path().join("shims");
  fs::create_dir_all(&repository)?;
  fs::create_dir_all(&links)?;
  fs::create_dir_all(&shims)?;

  successful_command("git", ["init", "--quiet"], &repository, None)
    .context("failed to initialize native smoke repository")?;
  write_runtime_shims(&shims)?;
  let original_path = std::env::var_os("PATH").unwrap_or_default();
  let mut smoke_paths = vec![shims];
  smoke_paths.extend(std::env::split_paths(&original_path));
  let smoke_path = std::env::join_paths(smoke_paths)?;

  for scenario in smoke_scenarios(&repository, &links) {
    successful_command(&path, scenario.arguments, &repository, Some(&smoke_path))
      .with_context(|| format!("native smoke scenario failed: {}", scenario.name))?;
  }

  println!("Native release smoke passed for {}.", path.display());
  Ok(())
}

fn successful_command(
  executable: impl AsRef<OsStr>,
  arguments: impl IntoIterator<Item = impl AsRef<OsStr>>,
  directory: &Path,
  path: Option<&OsStr>,
) -> Result<()> {
  let mut command = Command::new(executable);
  command
    .args(arguments)
    .current_dir(directory)
    .timeout(COMMAND_TIMEOUT);
  if let Some(path) = path {
    command.env("PATH", path);
  }
  command.ok().map(|_| ()).map_err(Into::into)
}

fn smoke_scenarios(repository: &Path, links: &Path) -> Vec<SmokeScenario> {
  vec![
    SmokeScenario::new("help", [OsString::from("--help")]),
    SmokeScenario::new(
      "repository inspection",
      [
        OsString::from("--cwd"),
        repository.as_os_str().to_owned(),
        OsString::from("repo"),
        OsString::from("inspect"),
        OsString::from("--json"),
      ],
    ),
    SmokeScenario::new(
      "todo write",
      [
        OsString::from("--cwd"),
        repository.as_os_str().to_owned(),
        OsString::from("todo"),
        OsString::from("add"),
        OsString::from("Native release smoke +Arcantry"),
        OsString::from("--source"),
        OsString::from("root"),
        OsString::from("--apply"),
      ],
    ),
    SmokeScenario::new(
      "private repository initialization",
      [
        OsString::from("--cwd"),
        repository.as_os_str().to_owned(),
        OsString::from("repo"),
        OsString::from("init"),
        OsString::from("--scope"),
        OsString::from("private"),
      ],
    ),
    SmokeScenario::new(
      "private repository removal",
      [
        OsString::from("--cwd"),
        repository.as_os_str().to_owned(),
        OsString::from("repo"),
        OsString::from("remove"),
        OsString::from("--scope"),
        OsString::from("private"),
      ],
    ),
    SmokeScenario::new(
      "skill link",
      [
        OsString::from("skills"),
        OsString::from("link"),
        OsString::from("adopt-arcantry"),
        OsString::from("--target"),
        links.as_os_str().to_owned(),
      ],
    ),
    SmokeScenario::new(
      "skill unlink",
      [
        OsString::from("skills"),
        OsString::from("unlink"),
        OsString::from("adopt-arcantry"),
        OsString::from("--target"),
        links.as_os_str().to_owned(),
      ],
    ),
  ]
}

fn write_runtime_shims(directory: &Path) -> Result<()> {
  for name in ["node", "bun", "python"] {
    #[cfg(windows)]
    fs::write(directory.join(format!("{name}.cmd")), "@exit /b 99\r\n")?;

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;
      let path = directory.join(name);
      fs::write(&path, "#!/bin/sh\nexit 99\n")?;
      fs::set_permissions(path, fs::Permissions::from_mode(0o755))?;
    }
  }
  Ok(())
}
