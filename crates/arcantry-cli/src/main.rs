mod cli;
mod embedded;
mod help;
mod parse_error;
mod release_cmd;
mod repo_cmd;
mod skills_cmd;
mod todo_cmd;

use anyhow::Result;
use clap::{Parser, error::ErrorKind};
pub(crate) use cli::{
  Cli, Command, ReleaseCommand, RepoCommand, RepoPlanArgs, SkillDoctorOptions, SkillLinkOptions,
  SkillUnlinkOptions, SkillsCommand, TodoCommand,
};
use std::path::{Path, PathBuf};

fn main() {
  let raw = std::env::args_os().collect::<Vec<_>>();
  let arguments = raw
    .iter()
    .skip(1)
    .filter_map(|value| value.to_str().map(str::to_owned))
    .collect::<Vec<_>>();
  if raw.len() == 1 {
    eprint!("{}", help::root());
    std::process::exit(1);
  }
  if let Some(error) = parse_error::render(&arguments) {
    eprint!("{error}");
    std::process::exit(1);
  }
  if let Some(output) = help::render(&arguments) {
    print!("{output}");
    return;
  }
  if raw.len() == 2 && matches!(raw[1].to_str(), Some("--version" | "-V")) {
    println!("{}", arcantry_core::VERSION);
    return;
  }
  let cli = match Cli::try_parse_from(raw) {
    Ok(cli) => cli,
    Err(error) => {
      let kind = error.kind();
      let _ = error.print();
      std::process::exit(
        if matches!(kind, ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
          0
        } else {
          1
        },
      );
    }
  };
  match execute(cli) {
    Ok(code) => std::process::exit(code),
    Err(error) => {
      eprintln!("Error: {error}");
      std::process::exit(1);
    }
  }
}

fn execute(cli: Cli) -> Result<i32> {
  let cwd = absolutize(cli.cwd.as_deref().unwrap_or(Path::new(".")))?;
  match cli.command {
    Command::Repo { command } => repo_cmd::execute(command, &cwd, cli.config.as_deref()),
    Command::Todo { command } => todo_cmd::execute(command, &cwd, cli.config.as_deref()),
    Command::Release { command } => release_cmd::execute(command, &cwd, cli.config.as_deref()),
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
