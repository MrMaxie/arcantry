pub const MANAGED_SECTION_START: &str = "<!-- arcantry:start -->";
pub const MANAGED_SECTION_END: &str = "<!-- arcantry:end -->";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManagedSectionResult {
  Unchanged(String),
  Changed(String),
  Conflict { content: String, reason: String },
}

pub fn render_managed_section(body: &str) -> String {
  format!(
    "{MANAGED_SECTION_START}\n{}\n{MANAGED_SECTION_END}",
    body.trim()
  )
}

pub fn upsert_managed_section(existing: &str, body: &str) -> ManagedSectionResult {
  let desired = render_managed_section(body);
  let starts: Vec<_> = existing
    .match_indices(MANAGED_SECTION_START)
    .map(|(index, _)| index)
    .collect();
  let ends: Vec<_> = existing
    .match_indices(MANAGED_SECTION_END)
    .map(|(index, _)| index)
    .collect();
  if starts.is_empty() && ends.is_empty() {
    let prefix = if existing.is_empty() {
      ""
    } else if existing.ends_with('\n') {
      "\n"
    } else {
      "\n\n"
    };
    return ManagedSectionResult::Changed(format!("{existing}{prefix}{desired}\n"));
  }
  if starts.len() != 1 || ends.len() != 1 || starts[0] > ends[0] {
    return ManagedSectionResult::Conflict {
      content: existing.to_owned(),
      reason: "Arcantry section markers are incomplete or duplicated.".to_owned(),
    };
  }
  let section_end = ends[0] + MANAGED_SECTION_END.len();
  if existing[starts[0]..section_end] == *desired {
    return ManagedSectionResult::Unchanged(existing.to_owned());
  }
  ManagedSectionResult::Changed(format!(
    "{}{}{}",
    &existing[..starts[0]],
    desired,
    &existing[section_end..]
  ))
}

pub fn remove_managed_section(existing: &str) -> ManagedSectionResult {
  let starts: Vec<_> = existing
    .match_indices(MANAGED_SECTION_START)
    .map(|(index, _)| index)
    .collect();
  let ends: Vec<_> = existing
    .match_indices(MANAGED_SECTION_END)
    .map(|(index, _)| index)
    .collect();
  if starts.is_empty() && ends.is_empty() {
    return ManagedSectionResult::Unchanged(existing.to_owned());
  }
  if starts.len() != 1 || ends.len() != 1 || starts[0] > ends[0] {
    return ManagedSectionResult::Conflict {
      content: existing.to_owned(),
      reason: "Arcantry section markers are incomplete or duplicated.".to_owned(),
    };
  }
  let section_end = ends[0] + MANAGED_SECTION_END.len();
  let before = existing[..starts[0]].trim_end_matches('\n');
  let after = existing[section_end..].trim_start_matches('\n');
  let body = [before, after]
    .into_iter()
    .filter(|part| !part.is_empty())
    .collect::<Vec<_>>()
    .join("\n\n");
  ManagedSectionResult::Changed(if body.is_empty() {
    String::new()
  } else {
    format!("{body}\n")
  })
}

pub fn contains_managed_section(content: &str) -> bool {
  content.contains(MANAGED_SECTION_START) && content.contains(MANAGED_SECTION_END)
}
