mod transition;

use crate::RepoCommand;
use anyhow::{Context, Result, bail};
use arcantry_core::config::{Management, SourceKind, Visibility, resolve_project};
use arcantry_core::knowledge::{KnowledgeInspection, inspect as inspect_knowledge};
use arcantry_core::project_plan::{
  Action, ApplyAuthority, ApplyOutcome, ProjectPlan, apply as apply_plan, create_delete_operation,
  create_write_operation, parse as parse_plan, render as render_plan, serialize as serialize_plan,
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
  output: Option<&Path>,
) -> Result<i32> {
  match command {
    RepoCommand::Recover {
      acknowledge,
      allow_outside,
    } => {
      let mut authority = ApplyAuthority::new(cwd)?;
      for path in allow_outside {
        authority = authority.allow_exact(&cwd.join(path))?;
      }
      println!(
        "{}",
        serde_json::to_string_pretty(&arcantry_core::project_plan::recover_with_authority(
          &authority,
          acknowledge
        )?)?
      );
      Ok(0)
    }
    RepoCommand::Inspect { json, detailed } => {
      let inspection = project_inspection(cwd, config, cwd_explicit)?;
      if json {
        println!("{}", serde_json::to_string_pretty(&inspection)?);
      } else {
        render_inspection(&inspection, detailed);
      }
      Ok(0)
    }
    RepoCommand::Plan(args) => {
      let json = args.json;
      let plan = plan_transition(&project_inspection(cwd, config, cwd_explicit)?, args)?;
      let code = i32::from(!plan.conflicts.is_empty());
      if let Some(path) = output {
        arcantry_core::project_plan::save(&plan, &cwd.join(path))?;
      } else if json {
        print!("{}", serialize_plan(&plan)?);
      } else {
        print!("{}", render_plan(&plan));
      }
      Ok(code)
    }
    RepoCommand::Detach(args) => {
      let plan = plan_detachment(
        &project_inspection(cwd, config, cwd_explicit)?,
        &args.capability,
      )?;
      handle_plan(plan, args.apply, args.json, output)
    }
    RepoCommand::Apply {
      plan,
      allow_outside,
    } => {
      let content = if plan == "-" {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        content
      } else {
        fs::read_to_string(cwd.join(plan))?
      };
      let parsed = parse_plan(&content)?;
      let inspection = project_inspection(cwd, config, cwd_explicit)?;
      let mut authority = ApplyAuthority::new(&inspection.root)?;
      for path in allow_outside {
        authority = authority.allow_exact(&if path.is_absolute() {
          path
        } else {
          cwd.join(path)
        })?;
      }
      let outcome = apply_plan(&parsed, &authority)?;
      render_apply_warnings(&outcome);
      if outcome.operations.is_empty() {
        println!("No file changes.");
      } else {
        for operation in outcome.operations {
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

fn plan_detachment(inspection: &KnowledgeInspection, requested: &[String]) -> Result<ProjectPlan> {
  let config_path = inspection
    .config_path
    .as_ref()
    .context("Detachment requires explicit Arcantry repository configuration.")?;
  let content = fs::read_to_string(config_path)?;
  let config = arcantry_core::config::parse_project_config(
    &content,
    Some(arcantry_core::VERSION),
    inspection.config_scope == Some("external"),
  )?;
  let mut available = config
    .sources
    .keys()
    .map(|id| format!("source:{id}"))
    .collect::<Vec<_>>();
  if config.release.is_some() {
    available.push("release-workflow".to_owned());
  }
  for (scope, path) in [("shared", "AGENTS.md"), ("private", ".local/AGENTS.md")] {
    if fs::read_to_string(inspection.root.join(path))
      .ok()
      .is_some_and(|text| arcantry_core::managed_content::contains_managed_section(&text))
    {
      available.push(format!("guidance:{scope}"));
    }
  }
  available.sort();
  let selected = if requested.is_empty() {
    available.clone()
  } else {
    let unique = requested.iter().collect::<std::collections::BTreeSet<_>>();
    if unique.len() != requested.len() {
      bail!("Detachment capability ids must be unique.");
    }
    for id in requested {
      if !available.contains(id) {
        bail!(
          "Unknown detachment capability: {id}. Available: {}",
          available.join(", ")
        );
      }
    }
    requested.to_vec()
  };
  let full = requested.is_empty();
  let selected_sources = selected
    .iter()
    .filter_map(|value| value.strip_prefix("source:"))
    .collect::<std::collections::BTreeSet<_>>();
  for (id, source) in &config.sources {
    if selected_sources.contains(id.as_str()) {
      continue;
    }
    if let Some(dependency) = source
      .from
      .iter()
      .find(|dependency| selected_sources.contains(dependency.as_str()))
    {
      bail!(
        "Capability source:{dependency} is still required by source:{id}; detach the dependent capability in the same plan."
      );
    }
  }
  if !selected.contains(&"release-workflow".to_owned())
    && config.release.as_ref().is_some_and(|release| {
      release
        .changelog_source
        .as_deref()
        .is_some_and(|id| selected_sources.contains(id))
        || release.units.values().any(|unit| {
          selected_sources.contains(unit.changelog_source.as_str())
            || unit
              .selectors
              .iter()
              .any(|selector| selected_sources.contains(selector.source.as_str()))
        })
    })
  {
    bail!("Selected sources are still required by release-workflow; detach it in the same plan.");
  }

  let mut plan = ProjectPlan::new(
    inspection.root.clone(),
    "repository",
    "detach",
    "arcantry-detachment@1",
  );
  plan.watch(&config_path.to_string_lossy())?;
  for source_id in &selected_sources {
    if let Some(source) = inspection
      .sources
      .iter()
      .find(|source| source.id == **source_id && source.origin == "configured" && source.exists)
    {
      plan.watch(&source.absolute_path.to_string_lossy())?;
    }
  }
  let config_plan_path = config_path.strip_prefix(&inspection.root).map_or_else(
    |_| config_path.to_string_lossy().into_owned(),
    |path| path.to_string_lossy().replace('\\', "/"),
  );
  let config_visibility = if config_plan_path == ".local/arcantry.toml" {
    Visibility::Private
  } else {
    Visibility::Shared
  };
  if full {
    plan.operations.push(create_delete_operation(
      &inspection.root,
      &config_plan_path,
      config_visibility,
    )?);
  } else {
    let desired = arcantry_core::config::detach_project_capabilities(
      &content,
      &selected,
      inspection.config_scope == Some("external"),
    )?;
    plan.operations.push(create_write_operation(
      &inspection.root,
      &config_plan_path,
      desired,
      config_visibility,
    )?);
  }
  for (capability, path, visibility) in [
    ("guidance:shared", "AGENTS.md", Visibility::Shared),
    ("guidance:private", ".local/AGENTS.md", Visibility::Private),
  ] {
    if !selected.iter().any(|value| value == capability) {
      continue;
    }
    let current = fs::read_to_string(inspection.root.join(path))?;
    match arcantry_core::managed_content::remove_managed_section(&current) {
      arcantry_core::managed_content::ManagedSectionResult::Changed(desired) => {
        plan.operations.push(create_write_operation(
          &inspection.root,
          path,
          desired,
          visibility,
        )?);
      }
      arcantry_core::managed_content::ManagedSectionResult::Unchanged(_) => {}
      arcantry_core::managed_content::ManagedSectionResult::Conflict { reason, .. } => {
        plan.conflicts.push(reason);
      }
    }
  }

  let shared_sources = selected_sources
    .iter()
    .filter_map(|id| {
      config
        .sources
        .get(*id)
        .filter(|source| arcantry_core::config::effective_visibility(source) == Visibility::Shared)
        .map(|source| format!("- source:{id}: `{}`", source.path))
    })
    .collect::<Vec<_>>();
  if !shared_sources.is_empty()
    || selected
      .iter()
      .any(|value| value == "release-workflow" || value == "guidance:shared")
    || (full && config_visibility == Visibility::Shared)
  {
    let record = detachment_record(&selected, &shared_sources, full);
    plan.operations.push(create_write_operation(
      &inspection.root,
      "PROJECT_CAPABILITIES.md",
      record,
      Visibility::Shared,
    )?);
  }
  let private_sources = selected_sources
    .iter()
    .filter_map(|id| {
      config
        .sources
        .get(*id)
        .filter(|source| arcantry_core::config::effective_visibility(source) == Visibility::Private)
        .map(|source| format!("- source:{id}: `{}`", source.path))
    })
    .collect::<Vec<_>>();
  if !private_sources.is_empty()
    || selected.iter().any(|value| value == "guidance:private")
    || (full && config_visibility == Visibility::Private)
  {
    let record = detachment_record(&selected, &private_sources, full);
    plan.operations.push(create_write_operation(
      &inspection.root,
      ".local/PROJECT_CAPABILITIES.md",
      record,
      Visibility::Private,
    )?);
    add_private_exclude_operation(&inspection.root, Visibility::Private, &mut plan)?;
  }
  plan.notes.push("Detached capabilities remain project-owned and receive no synchronization, support, or updates from Arcantry. Re-adoption is a new reviewed transition.".to_owned());
  plan.notes.push("Fresh-checkout verification must run without an Arcantry executable, network access, user-scoped skills, or .local state for shared capabilities.".to_owned());
  if !plan.conflicts.is_empty() {
    plan.operations.clear();
  }
  Ok(plan)
}

fn detachment_record(selected: &[String], sources: &[String], full: bool) -> String {
  let source_lines = if sources.is_empty() {
    "- No source path is disclosed in this scope.".to_owned()
  } else {
    sources.join("\n")
  };
  format!(
    "# Project-owned capabilities\n\nOwnership transfer: {}.\n\n## Capability budget\n\n- Selected capabilities: {}\n- Final owner: project maintainers\n- License: preserve the license and attribution of every retained or copied file\n\n## Retained sources\n\n{}\n\n## Negative dependency contract\n\nThese capabilities must work from a fresh checkout without an Arcantry executable or package, network access, user-scoped skills, private `.local` state, or an automatic update channel. Re-adoption requires a new reviewed transition.\n",
    if full { "full" } else { "partial" },
    selected.len(),
    source_lines
  )
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

pub fn handle_plan(
  plan: ProjectPlan,
  apply: bool,
  json: bool,
  output: Option<&Path>,
) -> Result<i32> {
  if let Some(path) = output {
    if apply {
      bail!(
        "--output cannot be combined with --apply; save the preview and use repo apply --plan."
      );
    }
    arcantry_core::project_plan::save(&plan, path)?;
    return Ok(i32::from(!plan.conflicts.is_empty()));
  }
  if !apply {
    if json {
      print!("{}", serialize_plan(&plan)?);
    } else {
      print!("{}", render_plan(&plan));
      println!("Run the same command with --apply to write these changes.");
    }
    return Ok(i32::from(!plan.conflicts.is_empty()));
  }
  let authority = authority_for_generated_plan(&plan)?;
  let outcome = apply_plan(&plan, &authority)?;
  render_apply_warnings(&outcome);
  if json {
    println!(
      "{}",
      serde_json::to_string_pretty(&serde_json::json!({ "applied": outcome.operations }))?
    );
  } else {
    for operation in &outcome.operations {
      println!("{}: {}", action_name(&operation.action), operation.path);
    }
    if outcome.operations.is_empty() {
      println!("No file changes.");
    }
  }
  Ok(0)
}

fn authority_for_generated_plan(plan: &ProjectPlan) -> Result<ApplyAuthority> {
  let mut authority = ApplyAuthority::new(&plan.root)?;
  for operation in &plan.operations {
    let path = Path::new(&operation.path);
    if path.is_absolute() {
      authority = authority.allow_exact(path)?;
    }
  }
  for path in plan
    .inputs
    .keys()
    .filter(|path| Path::new(path).is_absolute())
  {
    authority = authority.allow_exact(Path::new(path))?;
  }
  Ok(authority)
}

fn render_apply_warnings(outcome: &ApplyOutcome) {
  for warning in &outcome.warnings {
    eprintln!("WARNING: {warning}");
  }
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

fn render_inspection(inspection: &KnowledgeInspection, detailed: bool) {
  println!("Project: {}", inspection.root.display());
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
  let present = inspection
    .sources
    .iter()
    .filter(|source| source.exists)
    .count();
  println!(
    "Sources: {present} present, {} absent",
    inspection.sources.len() - present
  );
  println!(
    "Present: {}",
    joined_ids(
      inspection
        .sources
        .iter()
        .filter(|source| source.exists)
        .map(|source| source.id.as_str())
    )
  );
  println!(
    "Absent: {}",
    joined_ids(
      inspection
        .sources
        .iter()
        .filter(|source| !source.exists)
        .map(|source| source.id.as_str())
    )
  );
  println!(
    "Methodologies: {}",
    joined_ids(
      inspection
        .methodologies
        .iter()
        .filter(|item| item.active)
        .map(|item| item.id)
    )
  );
  println!(".local: {}", inspection.local_boundary.status);
  if detailed {
    for source in &inspection.sources {
      println!(
        "Source {}: kind={}, scope={}, visibility={}, management={}, adapter={}, status={}, state={}, origin={}, path={}, from={}",
        source.id,
        source.kind.name(),
        source.scope,
        source.visibility.name(),
        source.management.name(),
        source.adapter,
        source.adapter_status,
        if source.exists { "present" } else { "absent" },
        source.origin,
        source.path,
        if source.from.is_empty() {
          "none".to_owned()
        } else {
          source.from.join(",")
        }
      );
    }
    for methodology in &inspection.methodologies {
      println!(
        "Methodology {}: state={}, evidence={}",
        methodology.id,
        if methodology.active {
          "active"
        } else {
          "absent"
        },
        if methodology.evidence.is_empty() {
          "none".to_owned()
        } else {
          methodology.evidence.join(",")
        }
      );
    }
    println!(
      ".local details: git={}, exists={}, ignored={}, tracked={}, remote={}",
      inspection.local_boundary.git_repository,
      inspection.local_boundary.exists,
      inspection
        .local_boundary
        .ignored
        .map_or("n/a".to_owned(), |value| value.to_string()),
      inspection.local_boundary.tracked,
      inspection
        .local_boundary
        .remote_reference
        .as_deref()
        .unwrap_or("none")
    );
  }
  for diagnostic in &inspection.diagnostics {
    println!("WARNING: {diagnostic}");
  }
}

fn joined_ids<'a>(values: impl Iterator<Item = &'a str>) -> String {
  let values = values.collect::<Vec<_>>();
  if values.is_empty() {
    "none".to_owned()
  } else {
    values.join(", ")
  }
}

fn validate_repository_and_knowledge(
  cwd: &Path,
  config: Option<&Path>,
  cwd_explicit: bool,
  doctor: bool,
) -> Result<i32> {
  let report = repository::validate(cwd, config, cwd_explicit, doctor)?;
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
