mod transition;

use crate::RepoCommand;
use anyhow::{Result, bail};
use arcantry_core::config::{Management, SourceKind, Visibility, resolve_project};
use arcantry_core::knowledge::{KnowledgeInspection, inspect as inspect_knowledge};
use arcantry_core::project_plan::{
  Action, ProjectPlan, apply as apply_plan, create_write_operation, parse as parse_plan,
  render as render_plan, serialize as serialize_plan,
};
use arcantry_core::repository;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use transition::plan_transition;

pub fn execute(
  command: RepoCommand,
  cwd: &Path,
  config: Option<&Path>,
  cwd_explicit: bool,
) -> Result<i32> {
  match command {
    RepoCommand::Inspect { json } => {
      let inspection = project_inspection(cwd, config, cwd_explicit)?;
      if json {
        println!("{}", serde_json::to_string_pretty(&inspection)?);
      } else {
        render_inspection(&inspection);
      }
      Ok(0)
    }
    RepoCommand::Plan(args) => {
      let json = args.json;
      let plan = plan_transition(&project_inspection(cwd, config, cwd_explicit)?, args)?;
      let code = i32::from(!plan.conflicts.is_empty());
      if json {
        print!("{}", serialize_plan(&plan)?);
      } else {
        print!("{}", render_plan(&plan));
      }
      Ok(code)
    }
    RepoCommand::Apply { plan } => {
      let content = if plan == "-" {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        content
      } else {
        fs::read_to_string(cwd.join(plan))?
      };
      let applied = apply_plan(&parse_plan(&content)?)?;
      if applied.is_empty() {
        println!("No file changes.");
      } else {
        for operation in applied {
          println!("{}: {}", action_name(&operation.action), operation.path);
        }
      }
      Ok(0)
    }
    RepoCommand::Init(args) => {
      let (scope, compatibility) = repository_args(args.scope, args.compat)?;
      render_repository_changes(repository::init(cwd, scope, compatibility)?);
      Ok(0)
    }
    RepoCommand::Update(args) => {
      let (scope, compatibility) = repository_args(args.scope, args.compat)?;
      render_repository_changes(repository::update(cwd, scope, compatibility)?);
      Ok(0)
    }
    RepoCommand::Remove { scope } => {
      render_repository_changes(repository::remove(cwd, repository::Scope::parse(&scope)?)?);
      Ok(0)
    }
    RepoCommand::Doctor => validate_repository_and_knowledge(cwd, config, cwd_explicit, true),
    RepoCommand::Validate => validate_repository_and_knowledge(cwd, config, cwd_explicit, false),
  }
}

pub fn project_inspection(
  cwd: &Path,
  config: Option<&Path>,
  cwd_explicit: bool,
) -> Result<KnowledgeInspection> {
  inspect_knowledge(&resolve_project(
    cwd,
    config,
    cwd_explicit,
    Some(arcantry_core::VERSION),
  )?)
}

pub fn handle_plan(plan: ProjectPlan, apply: bool, json: bool) -> Result<i32> {
  if !apply {
    if json {
      print!("{}", serialize_plan(&plan)?);
    } else {
      print!("{}", render_plan(&plan));
      println!("Run the same command with --apply to write these changes.");
    }
    return Ok(i32::from(!plan.conflicts.is_empty()));
  }
  let applied = apply_plan(&plan)?;
  if json {
    println!(
      "{}",
      serde_json::to_string_pretty(&serde_json::json!({ "applied": applied }))?
    );
  } else {
    for operation in &applied {
      println!("{}: {}", action_name(&operation.action), operation.path);
    }
    if applied.is_empty() {
      println!("No file changes.");
    }
  }
  Ok(0)
}

pub(super) fn add_private_exclude_operation(
  root: &Path,
  visibility: Visibility,
  plan: &mut ProjectPlan,
) -> Result<()> {
  if visibility != Visibility::Private {
    return Ok(());
  }
  let Ok(git_path) = duct::cmd("git", ["rev-parse", "--git-path", "info/exclude"])
    .dir(root)
    .stderr_null()
    .read()
  else {
    return Ok(());
  };
  let candidate = PathBuf::from(git_path.trim());
  let absolute = if candidate.is_absolute() {
    candidate
  } else {
    root.join(candidate)
  };
  let current = fs::read_to_string(&absolute).unwrap_or_default();
  if current.lines().any(|line| line == ".local/") {
    return Ok(());
  }
  let separator = if !current.is_empty() && !current.ends_with('\n') {
    "\n"
  } else {
    ""
  };
  let path = absolute.strip_prefix(root).map_or_else(
    |_| absolute.display().to_string(),
    |value| value.to_string_lossy().replace('\\', "/"),
  );
  plan.operations.push(create_write_operation(
    root,
    &path,
    format!("{current}{separator}.local/\n"),
    Visibility::Private,
  )?);
  Ok(())
}

fn render_inspection(inspection: &KnowledgeInspection) {
  println!("Mode: {}", inspection.mode);
  println!(
    "Config: {}",
    inspection.config_path.as_ref().map_or_else(
      || "none".to_owned(),
      |path| format!(
        "{} ({})",
        inspection.config_scope.unwrap_or("external"),
        path.display()
      )
    )
  );
  for path in &inspection.shadowed_config_paths {
    println!("Shadowed config: {}", path.display());
  }
  if inspection.sources.is_empty() {
    println!("No knowledge sources detected.");
  }
  for source in &inspection.sources {
    println!(
      "{}\t{}\t{}\t{}\t{}\t{}\t{}",
      source.id,
      source.kind.name(),
      source.management.name(),
      source.adapter,
      source.confidence,
      if source.exists { "present" } else { "missing" },
      source.path
    );
  }
  for diagnostic in &inspection.diagnostics {
    println!("WARNING: {diagnostic}");
  }
}

fn validate_repository_and_knowledge(
  cwd: &Path,
  config: Option<&Path>,
  cwd_explicit: bool,
  doctor: bool,
) -> Result<i32> {
  let report = repository::validate(cwd, config, doctor)?;
  let mut valid = report.valid;
  for diagnostic in report.diagnostics {
    let line = format!(
      "{}: {}: {}",
      diagnostic.severity.to_uppercase(),
      diagnostic.path,
      diagnostic.message
    );
    if diagnostic.severity == "error" {
      eprintln!("{line}");
    } else {
      println!("{line}");
    }
    if let Some(repair) = diagnostic.repair {
      if diagnostic.severity == "error" {
        eprintln!("Repair: {repair}");
      } else {
        println!("Repair: {repair}");
      }
    }
  }
  if report.valid {
    println!("Repository adoption is valid.");
  }
  let inspection = project_inspection(cwd, config, cwd_explicit)?;
  for source in &inspection.sources {
    if source.management == Management::Ignore {
      continue;
    }
    if source.adapter_status != "supported" {
      let severity = if source.management == Management::Observe {
        "WARNING"
      } else {
        valid = false;
        "ERROR"
      };
      let message = if source.adapter_status == "wrong-kind" {
        format!(
          "Adapter {} does not support {}.",
          source.adapter,
          source.kind.name()
        )
      } else {
        format!(
          "Adapter {} is not supported by this Arcantry version.",
          source.adapter
        )
      };
      if severity == "ERROR" {
        eprintln!("{severity}: {}: {message}", source.id);
      } else {
        println!("{severity}: {}: {message}", source.id);
      }
    } else if !source.exists
      && matches!(source.management, Management::Validate | Management::Manage)
    {
      valid = false;
      eprintln!(
        "ERROR: {}: Configured source is missing at {}.",
        source.id, source.path
      );
    } else if source.kind == SourceKind::Openspec
      && source.exists
      && matches!(source.management, Management::Validate | Management::Manage)
      && !source.absolute_path.join("config.yaml").is_file()
    {
      valid = false;
      eprintln!("ERROR: {}: OpenSpec config.yaml is missing.", source.id);
    }
  }
  if valid {
    println!("Knowledge stack is valid.");
    Ok(0)
  } else {
    Ok(1)
  }
}

fn repository_args(scope: String, compat: Option<String>) -> Result<(repository::Scope, bool)> {
  let compatibility = match compat.as_deref() {
    None => false,
    Some("claude") => true,
    Some(_) => bail!("Invalid compatibility: only claude is supported."),
  };
  Ok((repository::Scope::parse(&scope)?, compatibility))
}

fn render_repository_changes(changes: Vec<repository::RepositoryChange>) {
  if changes.is_empty() {
    println!("No changes required.");
  } else {
    for change in changes {
      println!("{}: {}", change.action, change.path);
    }
  }
}

fn action_name(action: &Action) -> &'static str {
  match action {
    Action::Write => "write",
    Action::Delete => "delete",
    Action::DeleteTree => "delete-tree",
  }
}
