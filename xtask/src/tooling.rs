use anyhow::{Context, Result, bail};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn mise_program(root: &Path, name: &str) -> Result<PathBuf> {
  let output = Command::new("mise")
    .args(["which", name, "-C"])
    .arg(root)
    .output()
    .with_context(|| format!("failed to resolve {name} through Mise"))?;
  if !output.status.success() {
    bail!(
      "Mise could not resolve {name}: {}",
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  let reported = String::from_utf8(output.stdout)?;
  let reported = reported.trim();
  if reported.is_empty() {
    bail!("Mise returned an empty path for {name}");
  }
  let program = PathBuf::from(platform_program(OsStr::new(reported)));
  if !program.is_file() {
    bail!(
      "Mise resolved {name} to missing executable {}",
      program.display()
    );
  }
  Ok(program)
}

pub(crate) fn platform_program(command: &OsStr) -> OsString {
  #[cfg(windows)]
  if ["npm", "npx", "pnpm", "nub"]
    .iter()
    .any(|candidate| Path::new(command).file_name() == Some(OsStr::new(candidate)))
  {
    let mut program = command.to_os_string();
    program.push(".cmd");
    return program;
  }
  command.to_os_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keeps_the_resolved_parent_when_adding_a_windows_command_suffix() {
    let command = Path::new("tool-root").join("nub");
    let expected = if cfg!(windows) {
      Path::new("tool-root").join("nub.cmd").into_os_string()
    } else {
      command.clone().into_os_string()
    };
    assert_eq!(platform_program(command.as_os_str()), expected);
  }
}
