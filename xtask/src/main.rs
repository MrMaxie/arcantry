mod binary;
mod release;
mod smoke;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "xtask", version, about = "Arcantry native artifact checks")]
struct Arguments {
  #[command(subcommand)]
  command: Task,
}

#[derive(Debug, Subcommand)]
enum Task {
  /// Verify that a staged native executable reports the expected version.
  VerifyBinary {
    #[arg(long)]
    path: PathBuf,
    #[arg(long, default_value = "1.0.0")]
    expected_version: String,
  },
  /// Execute the release smoke suite against one native executable.
  SmokeBinary {
    #[arg(long)]
    path: PathBuf,
  },
  /// Execute the release smoke suite against one cargo-dist target.
  SmokeTarget {
    #[arg(long)]
    target: String,
    #[arg(long, default_value = "target")]
    root: PathBuf,
  },
  /// Collect the six cargo-dist archives downloaded from native jobs.
  CollectRelease {
    #[arg(long)]
    input: PathBuf,
    #[arg(long)]
    output: PathBuf,
  },
  /// Normalize cargo-dist output to the public Arcantry release contract.
  AssembleRelease {
    #[arg(long)]
    artifacts: PathBuf,
    #[arg(long)]
    installer_artifacts: Option<PathBuf>,
  },
}

fn main() -> Result<()> {
  match Arguments::parse().command {
    Task::VerifyBinary {
      path,
      expected_version,
    } => binary::verify_binary(path, &expected_version),
    Task::SmokeBinary { path } => smoke::smoke_binary(path),
    Task::SmokeTarget { target, root } => {
      smoke::smoke_binary(binary::target_binary(&root, &target)?)
    }
    Task::CollectRelease { input, output } => release::collect_release_artifacts(&input, &output),
    Task::AssembleRelease {
      artifacts,
      installer_artifacts,
    } => release::assemble_release(
      &artifacts,
      installer_artifacts.as_deref().unwrap_or(&artifacts),
    ),
  }
}
