use crate::TodoCommand;
use crate::repo_cmd::{add_private_exclude_operation, handle_plan, project_inspection};
use anyhow::{Context, Result, bail};
use arcantry_core::config::{Management, SourceKind, Visibility};
use arcantry_core::knowledge::{KnowledgeInspection, ProjectSource};
use arcantry_core::project_plan::{ProjectPlan, create_write_operation};
use arcantry_core::todo;
use chrono::Local;
use std::fs;
use std::path::Path;

pub fn execute(command: TodoCommand, cwd: &Path, config: Option<&Path>) -> Result<i32> {
  let inspection = project_inspection(cwd, config)?;
  match command {
    TodoCommand::List { source } => {
      let sources: Vec<_> = if let Some(source) = source {
        vec![resolve_source(&inspection, &source, false)?]
      } else {
        inspection
          .sources
          .iter()
          .filter(|source| source.kind == SourceKind::TodoTxt && source.exists)
          .cloned()
          .collect()
      };
      if sources.is_empty() {
        println!("No todo.txt tasks.");
      }
      for source in sources {
        println!("[{}]", source.id);
        for task in todo::inspect_tasks(&fs::read_to_string(source.absolute_path)?) {
          println!("{}\t{}", task.line, task.raw);
        }
      }
      Ok(0)
    }
    TodoCommand::Add {
      task,
      source,
      apply,
    } => {
      let source = select_source(&inspection, source.as_deref(), true)?;
      let current = if source.exists {
        fs::read_to_string(&source.absolute_path)?
      } else {
        String::new()
      };
      let mut plan = ProjectPlan::new(inspection.root.clone(), &source.id, "adopt", "todo-txt@1");
      plan.operations.push(create_write_operation(
        &inspection.root,
        &source.path,
        todo::add_task(&current, &task)?,
        source.visibility,
      )?);
      add_private_exclude_operation(&inspection.root, source.visibility, &mut plan)?;
      handle_plan(plan, apply, false)
    }
    TodoCommand::Complete {
      line,
      source,
      date,
      apply,
    } => {
      let source = select_source(&inspection, source.as_deref(), false)?;
      let current = fs::read_to_string(&source.absolute_path)?;
      let date = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
      let desired = todo::complete_task(&current, parse_line(&line)?, &date)?;
      let mut plan = ProjectPlan::new(inspection.root.clone(), &source.id, "adopt", "todo-txt@1");
      if desired != current {
        plan.operations.push(create_write_operation(
          &inspection.root,
          &source.path,
          desired,
          source.visibility,
        )?);
      }
      handle_plan(plan, apply, false)
    }
    TodoCommand::Move {
      line,
      from,
      to,
      apply,
    } => {
      let source = resolve_source(&inspection, &from, false)?;
      let target = resolve_source(&inspection, &to, true)?;
      if source.id == target.id {
        bail!("Todo move source and target must differ.");
      }
      let source_content = fs::read_to_string(&source.absolute_path)?;
      let target_content = if target.exists {
        fs::read_to_string(&target.absolute_path)?
      } else {
        String::new()
      };
      let (source_desired, target_desired) =
        todo::move_task(&source_content, &target_content, parse_line(&line)?)?;
      let mut plan = ProjectPlan::new(
        inspection.root.clone(),
        &source.id,
        "relocate",
        "todo-txt@1",
      );
      plan.operations.push(create_write_operation(
        &inspection.root,
        &source.path,
        source_desired,
        source.visibility,
      )?);
      plan.operations.push(create_write_operation(
        &inspection.root,
        &target.path,
        target_desired,
        target.visibility,
      )?);
      add_private_exclude_operation(&inspection.root, target.visibility, &mut plan)?;
      handle_plan(plan, apply, false)
    }
  }
}

fn resolve_source(
  inspection: &KnowledgeInspection,
  requested: &str,
  allow_missing: bool,
) -> Result<ProjectSource> {
  let id = match requested {
    "root" => "todo-root",
    "local" => "todo-local",
    value => value,
  };
  if let Some(source) = inspection.sources.iter().find(|source| {
    source.id == id && source.kind == SourceKind::TodoTxt && (allow_missing || source.exists)
  }) {
    return Ok(source.clone());
  }
  if allow_missing && matches!(id, "todo-root" | "todo-local") {
    let private = id == "todo-local";
    let path = if private {
      ".local/todo.txt"
    } else {
      "todo.txt"
    };
    return Ok(ProjectSource {
      id: id.to_owned(),
      kind: SourceKind::TodoTxt,
      path: path.to_owned(),
      management: Management::Observe,
      adapter: "todo-txt@1".to_owned(),
      from: Vec::new(),
      managed_from: None,
      visibility: if private {
        Visibility::Private
      } else {
        Visibility::Shared
      },
      scope: ".".to_owned(),
      absolute_path: inspection.root.join(path),
      exists: false,
      origin: "discovered",
      confidence: "high",
      adapter_status: "supported",
    });
  }
  bail!("Todo source is missing: {id}.")
}

fn select_source(
  inspection: &KnowledgeInspection,
  requested: Option<&str>,
  allow_missing: bool,
) -> Result<ProjectSource> {
  if let Some(requested) = requested {
    return resolve_source(inspection, requested, allow_missing);
  }
  let sources: Vec<_> = inspection
    .sources
    .iter()
    .filter(|source| source.kind == SourceKind::TodoTxt && (allow_missing || source.exists))
    .cloned()
    .collect();
  match sources.as_slice() {
    [source] => Ok(source.clone()),
    [] => bail!("No todo.txt source exists; choose --source root or --source local."),
    _ => bail!("More than one todo.txt source exists; choose --source explicitly."),
  }
}

fn parse_line(value: &str) -> Result<usize> {
  let line: usize = value
    .parse()
    .context("Todo line must be a positive integer.")?;
  if line == 0 || line.to_string() != value {
    bail!("Todo line must be a positive integer.");
  }
  Ok(line)
}
