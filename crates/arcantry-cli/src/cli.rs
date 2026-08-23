use chrono::Local;
use clap::{Args, CommandFactory, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
  name = "arcantry",
  version = arcantry_core::VERSION,
  about = "Manage Arcantry project sources, local releases, and skill links.",
  arg_required_else_help = true
)]
pub struct Cli {
  #[arg(
    long,
    global = true,
    value_name = "path",
    help = "Run against another repository or catalog location."
  )]
  pub cwd: Option<PathBuf>,
  #[arg(
    long,
    global = true,
    value_name = "path",
    help = "Use one explicit arcantry.toml file without config merging."
  )]
  pub config: Option<PathBuf>,
  #[command(subcommand)]
  pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
  #[command(about = "Manage repository adoption.")]
  Repo {
    #[command(subcommand)]
    command: RepoCommand,
  },
  #[command(about = "Use root or private todo.txt queues without imposing a workflow taxonomy.")]
  Todo {
    #[command(subcommand)]
    command: TodoCommand,
  },
  #[command(about = "Manage an OpenSpec-backed local release story without publishing.")]
  Release {
    #[command(subcommand)]
    command: ReleaseCommand,
  },
  #[command(about = "Inspect the catalog and manage skill links.")]
  Skills {
    #[command(subcommand)]
    command: SkillsCommand,
  },
}

#[derive(Subcommand)]
pub enum RepoCommand {
  #[command(about = "Discover project knowledge sources without changing them.")]
  Inspect {
    #[arg(long, help = "Write the complete machine-readable inspection.")]
    json: bool,
  },
  #[command(about = "Plan one explicit source transition without changing the project.")]
  Plan(RepoPlanArgs),
  #[command(about = "Apply an unchanged serialized transition plan.")]
  Apply {
    #[arg(
      long,
      value_name = "path",
      required = true,
      help = "Plan JSON file, or - for standard input."
    )]
    plan: String,
  },
  #[command(about = "Initialize minimal shared or private Arcantry repository state.")]
  Init(RepoInitArgs),
  #[command(about = "Refresh owned artifacts while preserving configuration.")]
  Update(RepoUpdateArgs),
  #[command(about = "Diagnose repository adoption without changing it.")]
  Doctor,
  #[command(about = "Validate repository adoption without changing it.")]
  Validate,
  #[command(about = "Remove only verified Arcantry-owned artifacts and sections.")]
  Remove {
    #[arg(
      long,
      required = true,
      value_name = "scope",
      help = "Configuration scope: shared or private."
    )]
    scope: String,
  },
}

#[derive(Args)]
pub struct RepoInitArgs {
  #[arg(
    long,
    required = true,
    value_name = "scope",
    help = "Configuration scope: shared or private."
  )]
  pub scope: String,
  #[arg(
    long,
    value_name = "compatibility",
    help = "Add an explicit compatibility adapter: claude."
  )]
  pub compat: Option<String>,
}

#[derive(Args)]
pub struct RepoUpdateArgs {
  #[arg(
    long,
    required = true,
    value_name = "scope",
    help = "Configuration scope: shared or private."
  )]
  pub scope: String,
  #[arg(
    long,
    value_name = "compatibility",
    help = "Add or refresh an explicit compatibility adapter: claude."
  )]
  pub compat: Option<String>,
}

#[derive(Args)]
pub struct RepoPlanArgs {
  #[arg(
    long,
    required = true,
    value_name = "id",
    help = "Source id reported by repo inspect."
  )]
  pub source: String,
  #[arg(
    long,
    required = true,
    value_name = "transition",
    help = "Transition: preserve, adopt, rebind, cutover, migrate, or relocate."
  )]
  pub transition: String,
  #[arg(
    long,
    value_name = "path",
    help = "Target source path for rebind or relocate."
  )]
  pub to_path: Option<String>,
  #[arg(long, value_name = "adapter", help = "Target versioned adapter.")]
  pub to_adapter: Option<String>,
  #[arg(
    long,
    value_name = "source",
    num_args = 1..,
    help = "Explicit source dependencies for adoption."
  )]
  pub from: Vec<String>,
  #[arg(
    long,
    value_name = "version",
    help = "First managed SemVer version for changelog cutover."
  )]
  pub managed_from: Option<String>,
  #[arg(long, help = "Delete the verified source after relocation.")]
  pub delete_source: bool,
  #[arg(long, help = "Write the serializable plan required by repo apply.")]
  pub json: bool,
}

#[derive(Subcommand)]
pub enum TodoCommand {
  #[command(about = "List todo.txt tasks from one or all detected queues.")]
  List {
    #[arg(long, value_name = "id", help = "Todo source id, root, or local.")]
    source: Option<String>,
  },
  #[command(about = "Preview or apply one todo.txt task addition.")]
  Add {
    #[arg(value_name = "task")]
    task: String,
    #[arg(long, value_name = "id", help = "Todo source id, root, or local.")]
    source: Option<String>,
    #[arg(long, help = "Apply the previewed file change.")]
    apply: bool,
  },
  #[command(about = "Preview or apply completion of one todo.txt line.")]
  Complete {
    #[arg(value_name = "line")]
    line: String,
    #[arg(long, value_name = "id", help = "Todo source id, root, or local.")]
    source: Option<String>,
    #[arg(long, value_name = "date", help = "Completion date in YYYY-MM-DD.")]
    date: Option<String>,
    #[arg(long, help = "Apply the previewed file change.")]
    apply: bool,
  },
  #[command(about = "Preview or apply an explicit move between todo.txt queues.")]
  Move {
    #[arg(value_name = "line")]
    line: String,
    #[arg(
      long,
      required = true,
      value_name = "id",
      help = "Source todo id, root, or local."
    )]
    from: String,
    #[arg(
      long,
      required = true,
      value_name = "id",
      help = "Target todo id, root, or local."
    )]
    to: String,
    #[arg(long, help = "Apply the previewed file changes.")]
    apply: bool,
  },
}

#[derive(Subcommand)]
pub enum ReleaseCommand {
  #[command(about = "Preview or apply a baseline for an existing project release.")]
  Baseline {
    #[arg(value_name = "version")]
    version: String,
    #[arg(
      long,
      required = true,
      value_name = "date",
      help = "Existing release date in YYYY-MM-DD."
    )]
    date: String,
    #[arg(
      long,
      value_name = "id",
      help = "Release unit for a multi-unit project."
    )]
    unit: Option<String>,
    #[arg(long, help = "Apply the previewed release plan.")]
    apply: bool,
    #[arg(long, help = "Write the plan or applied operations as JSON.")]
    json: bool,
  },
  #[command(about = "Inspect the next local release without changing files.")]
  Plan {
    #[arg(
      long,
      value_name = "id",
      help = "Release unit for a multi-unit project."
    )]
    unit: Option<String>,
    #[arg(long, help = "Write the release plan as JSON.")]
    json: bool,
  },
  #[command(about = "Preview or apply the next local release manifest, versions and changelog.")]
  Cut {
    #[arg(
      long,
      value_name = "date",
      default_value_t = local_date(),
      help = "Release date in YYYY-MM-DD."
    )]
    date: String,
    #[arg(
      long,
      value_name = "id",
      help = "Release unit for a multi-unit project."
    )]
    unit: Option<String>,
    #[arg(long, help = "Apply the previewed release plan.")]
    apply: bool,
    #[arg(long, help = "Write the plan or applied operations as JSON.")]
    json: bool,
  },
  #[command(about = "Preview or apply the deterministic managed changelog.")]
  Render {
    #[arg(
      long,
      value_name = "id",
      help = "Release unit for a multi-unit project."
    )]
    unit: Option<String>,
    #[arg(long, help = "Apply the previewed changelog plan.")]
    apply: bool,
    #[arg(long, help = "Write the plan or applied operations as JSON.")]
    json: bool,
  },
  #[command(about = "Check local release consistency without changing files.")]
  Check {
    #[arg(
      long,
      value_name = "id",
      help = "Release unit, required for sealed multi-unit checking."
    )]
    unit: Option<String>,
    #[arg(
      long,
      help = "Also require final assignment, clean Git state and a release seal."
    )]
    sealed: bool,
  },
}

#[derive(Subcommand)]
pub enum SkillsCommand {
  #[command(about = "List public catalog skills or private repository skills.")]
  List {
    #[arg(
      long,
      value_name = "path",
      help = "Directory containing catalog.json and skills/."
    )]
    catalog_root: Option<PathBuf>,
    #[arg(
      long,
      value_name = "scope",
      default_value = "public",
      help = "Inventory scope: public or private."
    )]
    scope: String,
  },
  #[command(about = "Show metadata for one public or private skill.")]
  Inspect {
    #[arg(value_name = "name")]
    name: String,
    #[arg(
      long,
      value_name = "path",
      help = "Directory containing catalog.json and skills/."
    )]
    catalog_root: Option<PathBuf>,
    #[arg(
      long,
      value_name = "scope",
      default_value = "public",
      help = "Inventory scope: public or private."
    )]
    scope: String,
  },
  #[command(about = "Link one canonical skill into the universal Agent Skills directory.")]
  Link {
    #[arg(value_name = "name")]
    name: String,
    #[command(flatten)]
    options: SkillLinkOptions,
    #[arg(
      long,
      help = "Back up an ordinary target or replace a different link before linking."
    )]
    replace: bool,
  },
  #[command(about = "Remove only an exact link to one catalog skill.")]
  Unlink {
    #[arg(value_name = "name")]
    name: String,
    #[command(flatten)]
    options: SkillUnlinkOptions,
  },
  #[command(
    about = "Validate skill packages and optionally inspect universal and compatibility links."
  )]
  Doctor {
    #[command(flatten)]
    options: SkillDoctorOptions,
  },
}

#[derive(Args)]
pub struct SkillLinkOptions {
  #[arg(
    long,
    value_name = "path",
    help = "Directory containing catalog.json and skills/."
  )]
  pub catalog_root: Option<PathBuf>,
  #[arg(
    long,
    value_name = "scope",
    help = "Skill scope: user, repo, or private."
  )]
  pub scope: Option<String>,
  #[arg(
    long,
    value_name = "compatibility",
    help = "Also add a compatibility alias: claude."
  )]
  pub compat: Option<String>,
  #[arg(
    long,
    value_name = "path",
    help = "Advanced explicit Agent Skills directory."
  )]
  pub target: Option<PathBuf>,
}

#[derive(Args)]
pub struct SkillUnlinkOptions {
  #[arg(
    long,
    value_name = "path",
    help = "Directory containing catalog.json and skills/."
  )]
  pub catalog_root: Option<PathBuf>,
  #[arg(
    long,
    value_name = "scope",
    help = "Skill scope: user, repo, or private."
  )]
  pub scope: Option<String>,
  #[arg(
    long,
    value_name = "compatibility",
    help = "Also remove the compatibility alias: claude."
  )]
  pub compat: Option<String>,
  #[arg(
    long,
    value_name = "path",
    help = "Advanced explicit Agent Skills directory."
  )]
  pub target: Option<PathBuf>,
}

#[derive(Args)]
pub struct SkillDoctorOptions {
  #[arg(
    long,
    value_name = "path",
    help = "Directory containing catalog.json and skills/."
  )]
  pub catalog_root: Option<PathBuf>,
  #[arg(
    long,
    value_name = "scope",
    help = "Skill scope to inspect: user, repo, or private."
  )]
  pub scope: Option<String>,
  #[arg(
    long,
    value_name = "compatibility",
    help = "Also inspect a compatibility alias: claude."
  )]
  pub compat: Option<String>,
  #[arg(
    long,
    value_name = "path",
    help = "Advanced explicit Agent Skills directory to inspect."
  )]
  pub target: Option<PathBuf>,
}

fn local_date() -> String {
  Local::now().format("%Y-%m-%d").to_string()
}

pub fn command() -> clap::Command {
  Cli::command()
}
