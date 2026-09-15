mod binary;
mod catalog;
mod ci_setup;
mod docs_output;
mod generate;
mod installer_smoke;
mod linux_system;
mod native_targets;
#[cfg(test)]
mod package_identity;
mod package_native;
mod package_projection;
mod package_smoke;
mod publication;
mod registry_smoke;
mod release;
mod repository_release;
mod smoke;
mod tooling;
mod typescript_boundary;

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
  /// Validate the public skill catalog and every canonical skill package.
  CatalogValidate {
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Expose Nub's pinned Node runtime to later GitHub Actions steps.
  CiSetup,
  /// Verify the built documentation origin and generated asset layout.
  DocsOutput {
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Generate plugin manifests and documentation projections.
  Generate {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[arg(long)]
    check: bool,
    #[arg(long)]
    docs_only: bool,
  },
  /// Refresh the npm package projections from canonical repository sources.
  PreparePackage {
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Build the reviewed Arcantry npm package set.
  PackageNative {
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    main: bool,
    #[arg(long, conflicts_with = "binary")]
    artifacts: Option<PathBuf>,
    #[arg(long, conflicts_with = "artifacts")]
    binary: Vec<String>,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Smoke-test the npm launcher and one native platform package.
  PackageSmoke {
    #[arg(long, conflicts_with = "target")]
    binary: Option<PathBuf>,
    #[arg(long, conflicts_with = "binary")]
    target: Option<String>,
    #[arg(long, requires = "target")]
    build_root: Option<PathBuf>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Validate and execute the reviewed Arcantry publication sequence.
  Publish {
    #[command(subcommand)]
    command: publication::Command,
  },
  /// Smoke-test all npm packages through a local registry.
  RegistrySmoke {
    #[arg(long)]
    archives: PathBuf,
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
  /// Smoke-test the shell and PowerShell release installers.
  InstallerSmoke {
    #[arg(long)]
    artifacts: PathBuf,
  },
  /// Manage this repository's conventional local release story.
  RepositoryRelease {
    #[command(subcommand)]
    command: repository_release::Command,
  },
  /// Run the complete Rust CLI system gate in a disposable pinned Linux container.
  LinuxSystemTest,
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
  /// Reject TypeScript outside the reviewed shrinking migration inventory.
  TypescriptBoundary {
    #[arg(long, default_value = ".")]
    root: PathBuf,
  },
}

fn main() -> Result<()> {
  match Arguments::parse().command {
    Task::CatalogValidate { root } => catalog::validate(&root),
    Task::CiSetup => ci_setup::configure(),
    Task::DocsOutput { root } => docs_output::verify(&root),
    Task::Generate {
      root,
      check,
      docs_only,
    } => generate::run(&root, check, docs_only),
    Task::PreparePackage { root } => package_projection::prepare(&root),
    Task::PackageNative {
      output,
      main,
      artifacts,
      binary,
      root,
    } => package_native::package(&root, &output, main, artifacts.as_deref(), &binary),
    Task::PackageSmoke {
      binary,
      target,
      build_root,
      output,
      root,
    } => package_smoke::smoke(
      &root,
      binary.as_deref(),
      target.as_deref(),
      build_root.as_deref(),
      output.as_deref(),
    ),
    Task::Publish { command } => publication::run(command),
    Task::RegistrySmoke { archives, root } => registry_smoke::smoke(&root, &archives),
    Task::InstallerSmoke { artifacts } => installer_smoke::smoke(&artifacts),
    Task::RepositoryRelease { command } => repository_release::run(command),
    Task::LinuxSystemTest => linux_system::run(),
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
    Task::TypescriptBoundary { root } => typescript_boundary::check(&root),
  }
}
