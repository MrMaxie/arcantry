mod embedded;
mod help;
mod parse_error;
mod release_cmd;
mod repo_cmd;
mod skills_cmd;
mod todo_cmd;

use anyhow::Result;
pub(crate) use arcantry_cli::cli::{
  Cli, Command, ReleaseCommand, RepoCommand, RepoPlanArgs, SkillDoctorOptions, SkillLinkOptions,
  SkillUnlinkOptions, SkillsCommand, TodoCommand,
};
use clap::{Parser, error::ErrorKind};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
  let raw = std::env::args_os().collect::<Vec<_>>();
  let arguments = raw
    .iter()
    .skip(1)
    .map(|value| value.to_str().map(str::to_owned))
    .collect::<Option<Vec<_>>>();
  if raw.len() == 1 {
    eprint!("{}", help::root());
    return ExitCode::FAILURE;
  }
  if let Some(arguments) = &arguments {
    if let Some(error) = parse_error::render(arguments) {
      eprint!("{error}");
      return ExitCode::FAILURE;
    }
    if let Some(output) = help::render(arguments) {
      print!("{output}");
      return ExitCode::SUCCESS;
    }
  }
  if raw.len() == 2 && matches!(raw[1].to_str(), Some("--version" | "-V")) {
    println!("{}", arcantry_core::VERSION);
    return ExitCode::SUCCESS;
  }
  let cli = match Cli::try_parse_from(raw) {
    Ok(cli) => cli,
    Err(error) => {
      let kind = error.kind();
      let _ = error.print();
      return if matches!(kind, ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
        ExitCode::SUCCESS
      } else {
        ExitCode::FAILURE
      };
    }
  };
  match execute(cli) {
    Ok(0) => ExitCode::SUCCESS,
    Ok(code) => ExitCode::from(code.clamp(1, u8::MAX.into()) as u8),
    Err(error) => {
      eprintln!("Error: {error}");
      ExitCode::FAILURE
    }
  }
}

fn execute(cli: Cli) -> Result<i32> {
  let cwd_explicit = cli.cwd.is_some();
  let cwd = absolutize(cli.cwd.as_deref().unwrap_or(Path::new(".")))?;
  match cli.command {
    Command::Repo { command } => {
      repo_cmd::execute(command, &cwd, cli.config.as_deref(), cwd_explicit)
    }
    Command::Todo { command } => {
      todo_cmd::execute(command, &cwd, cli.config.as_deref(), cwd_explicit)
    }
    Command::Release { command } => {
      release_cmd::execute(command, &cwd, cli.config.as_deref(), cwd_explicit)
    }
    Command::Skills { command } => skills_cmd::execute(command, &cwd),
  }
}

fn absolutize(path: &Path) -> Result<PathBuf> {
  let path = if path.is_absolute() {
    path.to_path_buf()
  } else {
    std::env::current_dir()?.join(path)
  };
  Ok(dunce::canonicalize(&path).unwrap_or(path))
}
