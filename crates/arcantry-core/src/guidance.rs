//! Read-only, bounded project answers shared by the CLI and MCP.
use crate::config::{ResolvedProject, SourceKind};
use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::{fs, io::Read, path::Path};

const LIMIT: u64 = 32 * 1024;

fn read(path: &Path) -> Result<Option<String>> {
  if !path.exists() {
    return Ok(None);
  }
  let mut text = String::new();
  fs::File::open(path)?
    .take(LIMIT + 1)
    .read_to_string(&mut text)?;
  if text.len() as u64 > LIMIT {
    bail!(
      "{} exceeds the contextual read limit of {LIMIT} bytes.",
      path.display()
    );
  }
  Ok(Some(text))
}

pub fn context(project: &ResolvedProject) -> Result<Value> {
  let inspection = crate::knowledge::inspect(project)?;
  let profile = project.config.as_ref().and_then(|c| c.context.as_ref());
  let mut changes = Vec::new();
  let mut archived = Vec::new();
  let mut rules = Vec::new();
  for relative in ["AGENTS.md", ".local/AGENTS.md"] {
    if let Some(content) = read(&project.root.join(relative))? {
      rules.push(json!({"source":relative,"content":content,"authority":"project guidance; conversation overrides are unknown"}));
    }
  }
  for source in &inspection.sources {
    if source.kind != SourceKind::Openspec
      || !source.exists
      || source.management == crate::config::Management::Ignore
    {
      continue;
    }
    let directory = source.absolute_path.join("changes");
    if let Ok(entries) = fs::read_dir(directory.join("archive")) {
      for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
          let name = entry.file_name().to_string_lossy().into_owned();
          if let Some(id) = name
            .get(11..)
            .filter(|_| name.as_bytes().get(10) == Some(&b'-'))
          {
            archived.push(id.to_owned());
          }
        }
      }
    }
    if !directory.is_dir() {
      continue;
    }
    let mut entries = fs::read_dir(directory)?.collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
      if entry.file_name() == "archive" || !entry.file_type()?.is_dir() {
        continue;
      }
      if changes.len() == 100 {
        bail!("More than 100 active changes; narrow the configured OpenSpec sources.");
      }
      let id = entry.file_name().to_string_lossy().into_owned();
      let relative = format!("{}/changes/{id}", source.path);
      if profile.is_some_and(|p| {
        p.exclude
          .iter()
          .any(|excluded| relative == *excluded || relative.starts_with(&format!("{excluded}/")))
      }) {
        continue;
      }
      let tasks = read(&entry.path().join("tasks.md"))?.unwrap_or_default();
      let pending: Vec<_> = tasks
        .lines()
        .filter_map(|line| line.trim().strip_prefix("- [ ] "))
        .collect();
      let done = tasks
        .lines()
        .filter(|line| line.trim().starts_with("- [x] "))
        .count();
      changes.push(json!({"id":id,"source":source.id,"path":format!("{}/changes/{id}",source.path),"pending":pending,"completedTasks":done,"status":if pending.is_empty() {"needs-closeout-review"} else {"pending"},"approval":"unknown"}));
    }
  }
  let configured = project.config.as_ref();
  Ok(
    json!({"schemaVersion":1,"root":project.root,"mode":project.mode,
    "sources":inspection.sources,"changes":changes,"archived":archived,"rules":rules,
    "workflow":configured.and_then(|c| c.workflow.as_ref()),
    "profile":configured.and_then(|c| c.context.as_ref()),
    "tools": (["git","cargo","just","openspec","varlock"].map(|name| json!({"name":name,"available":on_path(name)}))),
    "diagnostics":inspection.diagnostics,"authority":"Conversational approval is unknown. Project configuration does not grant permission.",
    "nextCommand":"arcantry next"}),
  )
}

pub fn next(project: &ResolvedProject, selected: Option<&str>) -> Result<Value> {
  let state = context(project)?;
  let changes = state["changes"]
    .as_array()
    .context("Missing change inventory")?;
  let workflow = project.config.as_ref().and_then(|c| c.workflow.as_ref());
  let completed = state["archived"]
    .as_array()
    .context("Missing archive inventory")?;
  let mut candidates: Vec<_> = changes
    .iter()
    .filter(|change| !change["pending"].as_array().is_none_or(Vec::is_empty))
    .collect();
  if let Some(profile) = project.config.as_ref().and_then(|c| c.context.as_ref()) {
    candidates.sort_by_key(|change| {
      !profile.focus.iter().any(|focus| {
        change
          .to_string()
          .to_lowercase()
          .contains(&focus.to_lowercase())
      })
    });
  }
  if let Some(workflow) = workflow {
    candidates.sort_by_key(|change| {
      workflow
        .order
        .iter()
        .position(|id| Some(id.as_str()) == change["id"].as_str())
        .unwrap_or(usize::MAX)
    });
  }
  let chosen = if let Some(selected) = selected {
    let matches: Vec<_> = changes.iter().filter(|c| c["id"] == selected).collect();
    if matches.len() != 1 {
      bail!("Change '{selected}' must identify exactly one discovered change.");
    }
    matches.first().copied()
  } else {
    candidates
      .iter()
      .copied()
      .find(|change| blockers(change, changes, completed, workflow).is_empty())
      .or_else(|| candidates.first().copied())
  };
  if let Some(change) = chosen {
    let blocked = blockers(change, changes, completed, workflow);
    let action = if !blocked.is_empty() {
      "Resolve the listed dependencies before implementation.".to_owned()
    } else if let Some(task) = change["pending"]
      .as_array()
      .and_then(|v| v.first())
      .and_then(Value::as_str)
    {
      format!("Confirm existing authorization, then: {task}")
    } else {
      "Review evidence and archive the completed change; a release is a separate decision."
        .to_owned()
    };
    Ok(
      json!({"schemaVersion":1,"change":change,"action":action,"blockers":blocked,"reason":"Project workflow order followed by stable source order; approval is not inferred.","command":"arcantry explain tasks","authority":"unknown"}),
    )
  } else {
    Ok(
      json!({"schemaVersion":1,"action":"Choose a concrete outcome from the todo queue or current request, then describe its acceptance criteria.","reason":"No pending OpenSpec change was discovered.","command":"arcantry explain proposal","blockers":[],"authority":"unknown"}),
    )
  }
}

fn blockers(
  change: &Value,
  changes: &[Value],
  completed: &[Value],
  workflow: Option<&crate::config::WorkflowConfig>,
) -> Vec<String> {
  workflow
    .and_then(|w| {
      w.dependencies
        .get(change["id"].as_str().unwrap_or_default())
    })
    .into_iter()
    .flatten()
    .filter(|id| {
      !completed
        .iter()
        .any(|value| value.as_str() == Some(id.as_str()))
        && !changes.iter().any(|c| {
          c["id"].as_str() == Some(id.as_str())
            && c["pending"].as_array().is_some_and(Vec::is_empty)
        })
    })
    .map(|id| {
      format!("Dependency {id} is missing or has pending tasks; review its completion evidence.")
    })
    .collect()
}

pub fn explain(project: &ResolvedProject, topic: &str) -> Result<Value> {
  let (format, example) = match topic {
    "todo" => (
      "One task per line. Preserve the selected queue's existing vocabulary; optional tags are not mandatory.",
      "2026-09-10 Improve the setup guide",
    ),
    "versions" | "version" => (
      "The configured release strategy controls identifiers. SemVer is the default. Package formats retain their own restrictions.",
      "semver: 1.2.3; integer: 42; calendar: 2026.09.10.1",
    ),
    "changelog" => (
      "Use the configured preset or project template. Preserve unmanaged history. Preview before applying.",
      "## [1.2.3] - 2026-09-10\n\n### Added\n\n- Explain the user-visible outcome.",
    ),
    "rules" | "workflow" => (
      "Project files describe conventions, not conversational authorization. Read the supplied source text; resolve conflicts against the current user request.",
      "Implementation, commit, push, deployment and release are separate actions.",
    ),
    "proposal" => (
      "Describe the outcome, why it matters, acceptance criteria and exclusions.",
      "# Why\nAgents need a next step.\n\n# What changes\nShow the next actionable task.\n\n# Out of scope\nAutomatic implementation.",
    ),
    "tasks" => (
      "Track executable work and verification using checkboxes; check an item only after its outcome is verified.",
      "- [ ] Implement the observable behavior.\n- [ ] Verify the acceptance scenario.",
    ),
    "specs" => (
      "Use the project's selected OpenSpec schema. Requirements describe observable behavior.",
      "## ADDED Requirements\n\n### Requirement: Useful context\nThe CLI MUST report the next step.\n\n#### Scenario: Resuming work\n- **WHEN** context is requested\n- **THEN** the next step is shown",
    ),
    "design" => (
      "Record decisions that would otherwise need to be rediscovered.",
      "# Approach\nReuse project discovery.\n\n# Trade-offs\nDo not infer permission from configuration.",
    ),
    "release" => (
      "Release notes describe consumer outcomes. A release is optional for ordinary work; use the configured project schema when preparing one.",
      "category: changed\nimpact: patch\nvisibility: public\ncomponents: [cli]\ntitle: Clearer project guidance",
    ),
    _ => bail!(
      "Unknown topic '{topic}'. Topics: proposal, tasks, specs, design, release, todo, versions, changelog, rules, workflow."
    ),
  };
  let state = context(project)?;
  let mut sources = Vec::new();
  for source in state["sources"]
    .as_array()
    .into_iter()
    .flatten()
    .filter(|s| s["kind"] == "openspec" && s["exists"] == true && s["management"] != "ignore")
  {
    let base = Path::new(source["absolutePath"].as_str().unwrap_or_default());
    if let Some(content) = read(&base.join("config.yaml"))? {
      let config: Value = serde_saphyr::from_str(&content)?;
      sources.push(json!({"source":format!("{}/config.yaml",source["path"].as_str().unwrap_or_default()),"content":content}));
      if let Some(schema) = config["schema"]
        .as_str()
        .filter(|s| !s.contains(['/', '\\']) && *s != "..")
      {
        let template = base
          .join("schemas")
          .join(schema)
          .join("templates")
          .join(format!("{topic}.md"));
        if let Some(content) = read(&template)? {
          sources.push(json!({"source":template,"content":content}));
        }
      }
    }
  }
  Ok(
    json!({"schemaVersion":1,"topic":topic,"format":format,"example":example,"defaultSource":"Arcantry built-in guidance; project schema and instructions take precedence","projectSources":sources,"rules":state["rules"],"configuration":project.config,"authority":"unknown"}),
  )
}

fn on_path(name: &str) -> bool {
  std::env::var_os("PATH").is_some_and(|path| {
    std::env::split_paths(&path).any(|directory| {
      if cfg!(windows) {
        [".exe", ".cmd", ".bat"]
          .iter()
          .any(|extension| directory.join(format!("{name}{extension}")).is_file())
      } else {
        directory.join(name).is_file()
      }
    })
  })
}

pub fn diagnostics(cwd: &Path, config: Option<&Path>, explicit: bool) -> Value {
  let project = crate::config::resolve_project(cwd, config, explicit, Some(crate::VERSION));
  let inspection = project
    .as_ref()
    .ok()
    .and_then(|p| crate::knowledge::inspect(p).ok());
  json!({"schemaVersion":1,"version":crate::VERSION,"platform":std::env::consts::OS,
    "configurationReadable":project.is_ok(),"inspectionAvailable":inspection.is_some(),
    "sources":inspection.as_ref().map(|i| i.sources.iter().map(|s| json!({"kind":s.kind,"management":s.management,"visibility":s.visibility,"exists":s.exists,"adapterStatus":s.adapter_status})).collect::<Vec<_>>()).unwrap_or_default(),
    "tools":(["git","cargo","just","openspec","varlock"].map(|name| json!({"name":name,"available":on_path(name)}))),
    "privacy":"Allowlisted metadata only. No paths, source contents, environment values or automatic upload."})
}
