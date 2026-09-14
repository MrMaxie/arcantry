use anyhow::{Result, bail};
use chrono::NaiveDate;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use todo_txt::task::Simple;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoDocument {
  bom: bool,
  newline: &'static str,
  trailing_newline: bool,
  entries: Vec<TodoEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TodoEntry {
  Task { raw: String, parsed: Simple },
  Empty(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TodoTask {
  pub line: usize,
  pub raw: String,
  pub completed: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub priority: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub creation_date: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub completion_date: Option<String>,
  pub projects: Vec<String>,
  pub contexts: Vec<String>,
  pub metadata: BTreeMap<String, String>,
  pub digest: String,
}

impl TodoDocument {
  pub fn parse(content: &str) -> Self {
    let (bom, plain) = content
      .strip_prefix('\u{feff}')
      .map_or((false, content), |plain| (true, plain));
    let newline = if plain.contains("\r\n") { "\r\n" } else { "\n" };
    let trailing_newline = plain.ends_with('\n');
    let mut lines = if plain.is_empty() {
      Vec::new()
    } else {
      plain
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line))
        .collect::<Vec<_>>()
    };
    if trailing_newline {
      lines.pop();
    }

    let entries = lines
      .into_iter()
      .map(|line| {
        if line.trim().is_empty() {
          TodoEntry::Empty(line.to_owned())
        } else {
          TodoEntry::Task {
            raw: line.to_owned(),
            parsed: parse_task(line),
          }
        }
      })
      .collect();

    Self {
      bom,
      newline,
      trailing_newline,
      entries,
    }
  }

  pub fn render(&self) -> String {
    let mut body = self
      .entries
      .iter()
      .map(|entry| match entry {
        TodoEntry::Task { raw, .. } => raw.clone(),
        TodoEntry::Empty(raw) => raw.clone(),
      })
      .collect::<Vec<_>>()
      .join(self.newline);
    if self.trailing_newline && !self.entries.is_empty() {
      body.push_str(self.newline);
    }
    if self.bom {
      body.insert(0, '\u{feff}');
    }
    body
  }

  pub fn tasks(&self) -> Vec<TodoTask> {
    self
      .entries
      .iter()
      .enumerate()
      .filter_map(|(index, entry)| match entry {
        TodoEntry::Task { raw, parsed } => Some(TodoTask::from_task(index + 1, raw, parsed)),
        TodoEntry::Empty(_) => None,
      })
      .collect()
  }

  pub fn add(&mut self, input: &str) -> Result<()> {
    let normalized = input.trim();
    if normalized.is_empty() || normalized.contains(['\r', '\n']) {
      bail!("A todo.txt task must be one non-empty line.");
    }
    self.entries.push(TodoEntry::Task {
      raw: normalized.to_owned(),
      parsed: parse_task(normalized),
    });
    if self.entries.len() == 1 {
      self.trailing_newline = true;
    }
    Ok(())
  }

  pub fn complete(&mut self, line: usize, completion_date: &str) -> Result<()> {
    let date = NaiveDate::parse_from_str(completion_date, "%Y-%m-%d")
      .ok()
      .filter(|_| completion_date.len() == 10)
      .ok_or_else(|| anyhow::anyhow!("Completion date must use YYYY-MM-DD."))?;
    let Some(entry) = self.entries.get_mut(line.saturating_sub(1)) else {
      bail!("todo.txt line {line} does not exist.");
    };
    let TodoEntry::Task { raw, parsed } = entry else {
      bail!("todo.txt line {line} does not contain a task.");
    };
    if parsed.finished {
      return Ok(());
    }

    if !parsed.priority.is_lowest() {
      parsed.tags.insert(
        "pri".to_owned(),
        char::from(parsed.priority.clone()).to_string(),
      );
      parsed.priority = todo_txt::Priority::lowest();
    }
    parsed.finished = true;
    parsed.finish_date = Some(date);
    *raw = parsed.to_string();
    Ok(())
  }

  pub fn defer(&mut self, line: usize, until: Option<&str>, wait: Option<&str>) -> Result<()> {
    if until.is_none() && wait.is_none() {
      bail!("Todo deferral requires --until, --wait, or both.");
    }
    if let Some(until) = until {
      validate_date(until, "Deferral date")?;
    }
    if let Some(wait) = wait
      && !valid_wait_slug(wait)
    {
      bail!("Wait condition must match [a-z0-9][a-z0-9._-]*.");
    }
    let raw = self.task_raw_mut(line)?;
    *raw = replace_deferral(raw, until, wait);
    self.reparse(line);
    Ok(())
  }

  pub fn resume(&mut self, line: usize) -> Result<()> {
    let raw = self.task_raw_mut(line)?;
    *raw = replace_deferral(raw, None, None);
    self.reparse(line);
    Ok(())
  }

  fn task_raw_mut(&mut self, line: usize) -> Result<&mut String> {
    let Some(entry) = self.entries.get_mut(line.saturating_sub(1)) else {
      bail!("todo.txt line {line} does not exist.");
    };
    match entry {
      TodoEntry::Task { raw, .. } => Ok(raw),
      TodoEntry::Empty(_) => bail!("todo.txt line {line} does not contain a task."),
    }
  }

  fn reparse(&mut self, line: usize) {
    if let Some(TodoEntry::Task { raw, parsed }) = self.entries.get_mut(line - 1) {
      *parsed = parse_task(raw);
    }
  }

  fn remove(&mut self, line: usize) -> Result<TodoEntry> {
    let index = line.saturating_sub(1);
    match self.entries.get(index) {
      None => bail!("todo.txt line {line} does not exist."),
      Some(TodoEntry::Empty(_)) => bail!("todo.txt line {line} does not contain a task."),
      Some(TodoEntry::Task { .. }) => {}
    }
    Ok(self.entries.remove(index))
  }

  fn push(&mut self, task: TodoEntry) {
    debug_assert!(matches!(&task, TodoEntry::Task { .. }));
    self.entries.push(task);
    if self.entries.len() == 1 {
      self.trailing_newline = true;
    }
  }
}

impl TodoTask {
  fn from_task(line: usize, raw: &str, task: &Simple) -> Self {
    let mut metadata = task.tags.clone();
    if let Some(date) = task.due_date {
      metadata.insert("due".to_owned(), date.format("%Y-%m-%d").to_string());
    }
    if let Some(date) = task.threshold_date {
      metadata.insert("t".to_owned(), date.format("%Y-%m-%d").to_string());
    }
    Self {
      line,
      raw: raw.to_owned(),
      completed: task.finished,
      priority: (!task.priority.is_lowest()).then(|| task.priority.to_string()),
      creation_date: task
        .create_date
        .map(|date| date.format("%Y-%m-%d").to_string()),
      completion_date: task
        .finish_date
        .map(|date| date.format("%Y-%m-%d").to_string()),
      projects: task.projects.clone(),
      contexts: task.contexts.clone(),
      metadata,
      digest: hex_digest(Sha256::digest(raw.as_bytes()).as_slice()),
    }
  }

  pub fn is_deferred_on(&self, date: NaiveDate) -> bool {
    if self.completed || self.metadata.contains_key("wait") {
      return true;
    }
    self
      .metadata
      .get("t")
      .and_then(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
      .is_some_and(|threshold| date < threshold)
  }
}

pub fn inspect_tasks(content: &str) -> Vec<TodoTask> {
  TodoDocument::parse(content).tasks()
}

pub fn add_task(content: &str, task: &str) -> Result<String> {
  let mut document = TodoDocument::parse(content);
  document.add(task)?;
  Ok(document.render())
}

pub fn complete_task(content: &str, line: usize, completion_date: &str) -> Result<String> {
  let mut document = TodoDocument::parse(content);
  document.complete(line, completion_date)?;
  Ok(document.render())
}

pub fn move_task(source: &str, target: &str, line: usize) -> Result<(String, String)> {
  let mut source = TodoDocument::parse(source);
  let task = source.remove(line)?;
  let mut target = TodoDocument::parse(target);
  target.push(task);
  Ok((source.render(), target.render()))
}

pub fn defer_task(
  content: &str,
  line: usize,
  until: Option<&str>,
  wait: Option<&str>,
) -> Result<String> {
  let mut document = TodoDocument::parse(content);
  document.defer(line, until, wait)?;
  Ok(document.render())
}

pub fn resume_task(content: &str, line: usize) -> Result<String> {
  let mut document = TodoDocument::parse(content);
  document.resume(line)?;
  Ok(document.render())
}

fn validate_date(value: &str, label: &str) -> Result<NaiveDate> {
  NaiveDate::parse_from_str(value, "%Y-%m-%d")
    .ok()
    .filter(|_| value.len() == 10)
    .ok_or_else(|| anyhow::anyhow!("{label} must use YYYY-MM-DD."))
}

fn valid_wait_slug(value: &str) -> bool {
  let mut chars = value.chars();
  chars
    .next()
    .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-'))
}

fn replace_deferral(raw: &str, until: Option<&str>, wait: Option<&str>) -> String {
  let leading_len = raw.len() - raw.trim_start().len();
  let trailing_len = raw.len() - raw.trim_end().len();
  let leading = &raw[..leading_len];
  let end = raw.len().saturating_sub(trailing_len);
  let trailing = &raw[end..];
  let middle = &raw[leading_len..end];
  let mut tokens = middle
    .split_whitespace()
    .filter(|token| !token.starts_with("t:") && !token.starts_with("wait:"))
    .map(str::to_owned)
    .collect::<Vec<_>>();
  if let Some(until) = until {
    tokens.push(format!("t:{until}"));
  }
  if let Some(wait) = wait {
    tokens.push(format!("wait:{wait}"));
  }
  format!("{leading}{}{trailing}", tokens.join(" "))
}

fn hex_digest(bytes: &[u8]) -> String {
  bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn parse_task(line: &str) -> Simple {
  let mut task = match line.parse::<Simple>() {
    Ok(task) => task,
    Err(error) => match error {},
  };
  normalize_single_completed_date(line, &mut task);
  task
}

fn normalize_single_completed_date(line: &str, task: &mut Simple) {
  if !task.finished || task.finish_date.is_some() {
    return;
  }
  let after_state = line.strip_prefix("x ").unwrap_or(line);
  let after_priority = after_state
    .strip_prefix('(')
    .and_then(|value| value.get(2..).filter(|value| value.starts_with(") ")))
    .and_then(|value| value.get(2..))
    .unwrap_or(after_state);
  let first = after_priority.split_whitespace().next();
  if first.is_some_and(|value| NaiveDate::parse_from_str(value, "%Y-%m-%d").is_ok()) {
    task.finish_date = task.create_date.take();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_and_serializes_the_complete_document() {
    let content =
      "\u{feff}(A) 2026-08-18 Ship +Arcantry @digitalroom owner:maxie\r\n\r\nFree form @desk\r\n";
    let document = TodoDocument::parse(content);
    let tasks = document.tasks();

    assert_eq!(tasks[0].priority.as_deref(), Some("A"));
    assert_eq!(tasks[0].creation_date.as_deref(), Some("2026-08-18"));
    assert_eq!(tasks[0].projects, ["Arcantry"]);
    assert_eq!(tasks[0].contexts, ["digitalroom"]);
    assert_eq!(
      tasks[0].metadata.get("owner").map(String::as_str),
      Some("maxie")
    );
    assert_eq!(document.render(), content);
  }

  #[test]
  fn preserves_noncanonical_untouched_lines() {
    let content = "\u{feff}x completed without date\r\n(a) lowercase priority\r\n  spaced task  \r\n   \r\nowner:one owner:two +P +P @C @C\r\n";
    let completion_content = "\u{feff}First\r\n(a) lowercase priority\r\n  spaced task  \r\n   \r\nowner:one owner:two +P +P @C @C\r\n";

    assert_eq!(
      add_task(content, "New task").unwrap(),
      format!("{content}New task\r\n")
    );
    assert_eq!(
      complete_task(completion_content, 1, "2026-08-20").unwrap(),
      "\u{feff}x 2026-08-20 First\r\n(a) lowercase priority\r\n  spaced task  \r\n   \r\nowner:one owner:two +P +P @C @C\r\n"
    );
  }

  #[test]
  fn completes_and_moves_structured_tasks() {
    assert_eq!(
      complete_task("(B) 2026-08-17 Verify +Arcantry @desk\n", 1, "2026-08-18").unwrap(),
      "x 2026-08-18 2026-08-17 Verify +Arcantry @desk pri:B\n"
    );
    assert_eq!(
      move_task("First\r\nSecond @desk\r\n", "\u{feff}Private\r\n", 2).unwrap(),
      (
        "First\r\n".to_owned(),
        "\u{feff}Private\r\nSecond @desk\r\n".to_owned()
      )
    );
  }

  #[test]
  fn reads_a_single_completed_date_as_the_completion_date() {
    let tasks = inspect_tasks("x 2026-08-18 Completed +Arcantry\n");
    assert_eq!(tasks[0].completion_date.as_deref(), Some("2026-08-18"));
    assert_eq!(tasks[0].creation_date, None);
  }

  #[test]
  fn rejects_invalid_changes() {
    assert!(
      add_task("", "one\ntwo")
        .unwrap_err()
        .to_string()
        .contains("one non-empty line")
    );
    assert!(
      complete_task("one\n", 2, "2026-08-18")
        .unwrap_err()
        .to_string()
        .contains("does not exist")
    );
  }

  #[test]
  fn defers_and_resumes_without_changing_document_format() {
    let content = "\u{feff}  Keep spacing owner:maxie  \r\nSecond\r\n";
    let deferred = defer_task(content, 1, Some("2026-10-01"), Some("legal-review")).unwrap();
    assert_eq!(
      deferred,
      "\u{feff}  Keep spacing owner:maxie t:2026-10-01 wait:legal-review  \r\nSecond\r\n"
    );
    assert_eq!(
      resume_task(&deferred, 1).unwrap(),
      "\u{feff}  Keep spacing owner:maxie  \r\nSecond\r\n"
    );
  }

  #[test]
  fn reports_digest_and_filters_only_future_or_waiting_tasks() {
    let tasks = inspect_tasks("Today t:2026-09-11\nFuture t:2026-09-12\nManual wait:owner\n");
    let today = NaiveDate::from_ymd_opt(2026, 9, 11).unwrap();
    assert!(!tasks[0].is_deferred_on(today));
    assert!(tasks[1].is_deferred_on(today));
    assert!(tasks[2].is_deferred_on(today));
    assert_eq!(tasks[0].digest.len(), 64);
  }
}
