mod config;
mod files;

use super::add_private_exclude_operation;
use crate::RepoPlanArgs;
use anyhow::{Context, Result, bail};
use arcantry_core::config::{
  Management, ProjectSourcePatch, SourceKind, Visibility, parse_project_config,
  patch_project_source,
};
use arcantry_core::knowledge::{KnowledgeInspection, adapter_status};
use arcantry_core::project_plan::{ProjectPlan, create_write_operation};
use config::{SourceUpdate, add_configured_source};
use files::*;
use std::fs;
use std::path::Path;

pub(super) fn plan_transition(
  inspection: &KnowledgeInspection,
  args: RepoPlanArgs,
) -> Result<ProjectPlan> {
  let RepoPlanArgs {
    source: source_id,
    transition,
    to_path,
    to_adapter,
    from,
    managed_from,
    delete_source,
    json: _,
  } = args;
  if !matches!(
    transition.as_str(),
    "preserve" | "adopt" | "rebind" | "cutover" | "migrate" | "relocate"
  ) {
    bail!("Invalid transition: {transition}");
  }
  let source = inspection
    .sources
    .iter()
    .find(|source| source.id == source_id)
    .cloned()
    .or_else(|| {
      if transition == "adopt" {
        standard_missing_source(&inspection.root, &source_id)
      } else {
        None
      }
    })
    .with_context(|| format!("Unknown source: {source_id}"))?;
  let mut plan = ProjectPlan::new(
    inspection.root.clone(),
    &source.id,
    &transition,
    &source.adapter,
  );
  let mut desired_path = source.path.clone();
  let mut desired_adapter = to_adapter.unwrap_or_else(|| source.adapter.clone());
  let mut desired_management = source.management.clone();
  let mut desired_managed_from = source.managed_from.clone();
  let mut desired_from = source.from.clone();

  match transition.as_str() {
    "preserve" => plan
      .notes
      .push("The source remains unchanged and keeps its current management policy.".to_owned()),
    "adopt" => {
      desired_management = Management::Manage;
      if !from.is_empty() {
        desired_from = from;
      }
      if adapter_status(&source.kind, &source.adapter) != "supported" {
        plan.conflicts.push(format!(
          "Adapter {} is not available for adoption.",
          source.adapter
        ));
      }
      if !source.exists && plan.conflicts.is_empty() {
        plan_source_initialization(inspection, &source, &mut plan.operations)?;
        plan.notes.push(format!(
          "Missing {} source will be initialized without project-language scaffolding.",
          source.kind.name()
        ));
      }
    }
    "rebind" => {
      desired_management = Management::Manage;
      if let Some(target) = to_path {
        desired_path = target;
        if !source_path_exists(&inspection.root, &desired_path, &source.kind) {
          plan.conflicts.push(format!(
            "Rebind target does not contain a {} source: {}.",
            source.kind.name(),
            desired_path
          ));
        }
      } else {
        plan.conflicts.push("Rebind requires --to-path.".to_owned());
      }
    }
    "cutover" => {
      desired_management = Management::Manage;
      desired_adapter = desired_adapter_for(&source, "keep-a-changelog@2", desired_adapter);
      desired_managed_from = managed_from.clone();
      if source.kind != SourceKind::Changelog {
        plan
          .conflicts
          .push("Cutover is currently defined only for changelog sources.".to_owned());
      }
      if !source.exists {
        plan
          .conflicts
          .push(format!("Source {} does not exist.", source.id));
      }
      if managed_from.is_none() {
        plan
          .conflicts
          .push("Changelog cutover requires --managed-from <version>.".to_owned());
      }
      if plan.conflicts.is_empty() {
        let current = fs::read_to_string(&source.absolute_path)?;
        match arcantry_core::changelog::cutover(&current, managed_from.as_deref().unwrap()) {
          Ok(desired) if desired != current => plan.operations.push(create_write_operation(
            &inspection.root,
            &source.path,
            desired,
            source.visibility,
          )?),
          Ok(_) => {}
          Err(error) => plan.conflicts.push(error.to_string()),
        }
      }
    }
    "migrate" => {
      desired_management = Management::Manage;
      desired_adapter = desired_adapter_for(&source, "keep-a-changelog@2", desired_adapter);
      desired_managed_from = None;
      if source.kind != SourceKind::Changelog {
        plan
          .conflicts
          .push("No semantic migration is available for this source kind.".to_owned());
      }
      if !source.exists {
        plan
          .conflicts
          .push(format!("Source {} does not exist.", source.id));
      }
      if plan.conflicts.is_empty() {
        let current = fs::read_to_string(&source.absolute_path)?;
        match arcantry_core::changelog::migrate_to_v2(&current) {
          Ok(desired) if desired != current => plan.operations.push(create_write_operation(
            &inspection.root,
            &source.path,
            desired,
            source.visibility,
          )?),
          Ok(_) => {}
          Err(error) => plan.conflicts.push(error.to_string()),
        }
      }
    }
    "relocate" => {
      if let Some(target_path) = to_path {
        let target = resolve_source_path(&inspection.root, &target_path);
        if same_path(&target, &source.absolute_path) {
          plan
            .conflicts
            .push("Relocate target must differ from the source path.".to_owned());
        } else if !source.exists {
          plan
            .conflicts
            .push(format!("Source {} does not exist.", source.id));
        } else {
          desired_path = target_path;
          if source.kind == SourceKind::Openspec {
            if delete_source && is_within(&source.absolute_path, &target) {
              plan.conflicts.push(
                "A relocated OpenSpec target cannot be inside a source tree that will be deleted."
                  .to_owned(),
              );
            } else {
              plan_directory_relocation(
                inspection,
                &source,
                &target,
                delete_source,
                &mut plan.operations,
                &mut plan.conflicts,
              )?;
            }
          } else {
            plan_file_relocation(
              inspection,
              &source,
              &target,
              delete_source,
              &mut plan.operations,
              &mut plan.conflicts,
            )?;
          }
        }
      } else {
        plan
          .conflicts
          .push("Relocate requires --to-path.".to_owned());
      }
    }
    _ => unreachable!(),
  }

  if matches!(
    transition.as_str(),
    "adopt" | "rebind" | "cutover" | "migrate"
  ) && adapter_status(&source.kind, &desired_adapter) != "supported"
  {
    plan.conflicts.push(format!(
      "Target adapter {desired_adapter} is not supported for {transition}."
    ));
  }

  if let Some(config_path) = &inspection.config_path {
    if plan.conflicts.is_empty() && transition != "preserve" {
      if Path::new(&desired_path).is_absolute() && is_within(&inspection.root, config_path) {
        plan.conflicts.push(
          "Embedded configuration cannot persist an absolute source path. Use an external --config file."
            .to_owned(),
        );
      } else {
        let allow_absolute_paths = inspection.config_scope == Some("external");
        let current = fs::read_to_string(config_path)?;
        let updated = if source.origin == "configured" {
          patch_project_source(
            &current,
            ProjectSourcePatch {
              id: &source.id,
              path: &desired_path,
              management: &desired_management,
              adapter: &desired_adapter,
              from: &desired_from,
              managed_from: desired_managed_from.as_deref(),
              allow_absolute_paths,
            },
          )?
        } else {
          add_configured_source(
            &current,
            &source,
            SourceUpdate {
              path: desired_path.clone(),
              management: desired_management.clone(),
              adapter: desired_adapter.clone(),
              from: desired_from.clone(),
              managed_from: desired_managed_from.clone(),
            },
            allow_absolute_paths,
          )?
        };
        if let Err(error) =
          parse_project_config(&updated, Some(arcantry_core::VERSION), allow_absolute_paths)
        {
          plan.conflicts.push(error.to_string());
        } else if updated != current {
          plan.operations.push(create_write_operation(
            &inspection.root,
            &plan_path(&inspection.root, config_path),
            updated,
            config_visibility(&inspection.root, config_path),
          )?);
        }
      }
    }
  } else if transition != "preserve" {
    plan.notes.push(
      "No configuration will be created; this transition leaves no Arcantry-specific project metadata."
        .to_owned(),
    );
  }

  if plan.conflicts.is_empty()
    && plan
      .operations
      .iter()
      .any(|operation| is_local_plan_path(&inspection.root, &operation.path))
  {
    add_private_exclude_operation(&inspection.root, Visibility::Private, &mut plan)?;
  }
  plan.target_adapter = Some(desired_adapter);
  if !plan.conflicts.is_empty() {
    plan.operations.clear();
  }
  Ok(plan)
}
