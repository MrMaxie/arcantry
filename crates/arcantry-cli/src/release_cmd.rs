use crate::ReleaseCommand;
use crate::repo_cmd::handle_plan;
use anyhow::Result;
use arcantry_core::config::resolve_project;
use std::path::Path;

pub fn execute(command: ReleaseCommand, cwd: &Path, config: Option<&Path>) -> Result<i32> {
  let project = resolve_project(cwd, config, true, Some(arcantry_core::VERSION))?;
  match command {
    ReleaseCommand::Baseline {
      version,
      date,
      apply,
      json,
    } => handle_plan(
      arcantry_core::release::baseline(&project, &version, &date)?,
      apply,
      json,
    ),
    ReleaseCommand::Plan { json } => {
      let plan = arcantry_core::release::inspect(&project)?;
      if json {
        println!("{}", serde_json::to_string_pretty(&plan)?);
      } else {
        println!(
          "Current: {}\nNext: {}\nImpact: {}",
          plan.current, plan.next, plan.impact
        );
        println!(
          "{}",
          if plan.changes.is_empty() {
            "Changes: none".to_owned()
          } else {
            format!("Changes: {}", plan.changes.join(", "))
          }
        );
      }
      Ok(0)
    }
    ReleaseCommand::Cut { date, apply, json } => {
      handle_plan(arcantry_core::release::cut(&project, &date)?, apply, json)
    }
    ReleaseCommand::Render { apply, json } => {
      handle_plan(arcantry_core::release::render(&project)?, apply, json)
    }
    ReleaseCommand::Check { sealed } => {
      arcantry_core::release::check(&project, sealed)?;
      println!(
        "{}",
        if sealed {
          "Release state is sealed."
        } else {
          "Release state is consistent."
        }
      );
      Ok(0)
    }
  }
}
