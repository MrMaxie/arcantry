use crate::ReleaseCommand;
use crate::repo_cmd::handle_plan;
use anyhow::Result;
use arcantry_core::config::resolve_project;
use std::path::Path;

pub fn execute(
  command: ReleaseCommand,
  cwd: &Path,
  config: Option<&Path>,
  cwd_explicit: bool,
) -> Result<i32> {
  let project = resolve_project(cwd, config, cwd_explicit, Some(arcantry_core::VERSION))?;
  match command {
    ReleaseCommand::Baseline {
      version,
      date,
      unit,
      apply,
      json,
    } => handle_plan(
      arcantry_core::release::baseline(&project, &version, &date, unit.as_deref())?,
      apply,
      json,
    ),
    ReleaseCommand::Plan { unit, json } => {
      let plan = arcantry_core::release::inspect(&project, unit.as_deref())?;
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
        if let Some(unit) = &plan.unit {
          println!("Unit: {unit}");
          println!("Topology: {}", plan.topology.as_deref().unwrap_or("single"));
          println!(
            "Ready: {}",
            if plan.ready == Some(true) {
              "yes"
            } else {
              "no"
            }
          );
          let pending = plan
            .pending_dependencies
            .as_ref()
            .map(|values| {
              values
                .iter()
                .map(|(id, version)| format!("{id}@{version}"))
                .collect::<Vec<_>>()
            })
            .unwrap_or_default();
          println!(
            "{}",
            if pending.is_empty() {
              "Pending dependencies: none".to_owned()
            } else {
              format!("Pending dependencies: {}", pending.join(", "))
            }
          );
        }
      }
      Ok(0)
    }
    ReleaseCommand::Cut {
      date,
      unit,
      apply,
      json,
    } => handle_plan(
      arcantry_core::release::cut(&project, &date, unit.as_deref())?,
      apply,
      json,
    ),
    ReleaseCommand::Render { unit, apply, json } => handle_plan(
      arcantry_core::release::render(&project, unit.as_deref())?,
      apply,
      json,
    ),
    ReleaseCommand::Check { unit, sealed } => {
      arcantry_core::release::check(&project, sealed, unit.as_deref())?;
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
