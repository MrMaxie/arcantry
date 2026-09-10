mod embedded;
mod mcp;
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
    Command::Mcp => {
      mcp::run(&cwd, cli.config.as_deref())?;
      Ok(0)
    }
    command @ (Command::Context { .. } | Command::Next { .. } | Command::Explain { .. }) => {
      let project = arcantry_core::config::resolve_project(
        &cwd,
        cli.config.as_deref(),
        cwd_explicit,
        Some(arcantry_core::VERSION),
      )?;
      let (value, json, detailed) = match command {
        Command::Context { json, detailed } => {
          (arcantry_core::guidance::context(&project)?, json, detailed)
        }
        Command::Next { change, json } => (
          arcantry_core::guidance::next(&project, change.as_deref())?,
          json,
          false,
        ),
        Command::Explain { topic, json } => (
          arcantry_core::guidance::explain(&project, &topic)?,
          json,
          true,
        ),
        _ => unreachable!(),
      };
      if json || detailed {
        println!("{}", serde_json::to_string_pretty(&value)?);
      } else if let Some(action) = value["action"].as_str() {
        println!(
          "{action}\nNext: {}",
          value["command"].as_str().unwrap_or_default()
        );
        for blocker in value["blockers"].as_array().into_iter().flatten() {
          println!("Blocked: {}", blocker.as_str().unwrap_or_default());
        }
      } else {
        println!(
          "Project: {}\nMode: {}",
          project.root.display(),
          project.mode
        );
        for source in value["sources"]
          .as_array()
          .into_iter()
          .flatten()
          .filter(|s| s["exists"] == true)
        {
          println!(
            "Source: {} ({})",
            source["path"].as_str().unwrap_or_default(),
            source["visibility"].as_str().unwrap_or_default()
          );
        }
        println!(
          "Active changes: {}\nNext: arcantry next\nRules and examples: arcantry explain rules",
          value["changes"].as_array().map_or(0, Vec::len)
        );
      }
      Ok(0)
    }

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
