use anyhow::{Result, bail};
use semver::Version;

pub const KEEP_A_CHANGELOG_PREAMBLE: &str = "# Changelog\n\nAll notable changes to this project will be documented in this file.\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n";

pub fn render_empty() -> String {
  format!("{KEEP_A_CHANGELOG_PREAMBLE}\n## [Unreleased]\n")
}

pub fn cutover(content: &str, managed_from: &str) -> Result<String> {
  Version::parse(managed_from).map_err(|_| {
    anyhow::anyhow!("managed_from must be a full SemVer version for changelog cutover.")
  })?;
  if content.contains(&format!("Arcantry manages releases from {managed_from}.")) {
    return Ok(content.to_owned());
  }

  let plain = content.strip_prefix('\u{feff}').unwrap_or(content);
  let first_release = release_heading_offsets(plain)
    .into_iter()
    .next()
    .ok_or_else(|| {
      anyhow::anyhow!("Changelog cutover requires at least one dated version section.")
    })?;
  if let Some(unreleased) = plain.find("## [Unreleased]")
    && unreleased < first_release
  {
    let body = &plain[unreleased + "## [Unreleased]".len()..first_release];
    let remaining = body
      .lines()
      .filter(|line| !line.starts_with("### "))
      .collect::<Vec<_>>()
      .join("\n");
    if !remaining.trim().is_empty() {
      bail!(
        "Changelog cutover cannot preserve a non-empty Unreleased section without explicit OpenSpec meaning."
      );
    }
  }

  let bom = if content.starts_with('\u{feff}') {
    "\u{feff}"
  } else {
    ""
  };
  let marker = format!(
    "<!-- Arcantry manages releases from {managed_from}. Earlier history remains project-owned. -->"
  );
  Ok(format!(
    "{bom}{KEEP_A_CHANGELOG_PREAMBLE}\n## [Unreleased]\n\n{marker}\n\n{}",
    &plain[first_release..]
  ))
}

pub fn migrate_to_v2(content: &str) -> Result<String> {
  if content.contains("keepachangelog.com/en/2.0.0") {
    return Ok(content.to_owned());
  }
  if !content.contains("keepachangelog.com/en/1.") {
    bail!("Full changelog migration requires an identified Keep a Changelog 1.x source.");
  }

  let mut desired = content.to_owned();
  while let Some(start) = desired.find("https://keepachangelog.com/en/1.") {
    let suffix = &desired[start..];
    let end = suffix
      .find(|character: char| {
        character.is_whitespace() || matches!(character, ')' | ']' | '>' | '"' | '\'')
      })
      .unwrap_or(suffix.len());
    desired.replace_range(start..start + end, "https://keepachangelog.com/en/2.0.0/");
  }
  Ok(desired)
}

fn release_heading_offsets(content: &str) -> Vec<usize> {
  content
    .match_indices("## [")
    .filter_map(|(offset, _)| {
      let line = content[offset..].lines().next()?;
      let body = line.strip_prefix("## [")?;
      let (version, date) = body.split_once("] - ")?;
      if Version::parse(version).is_ok()
        && date.len() == 10
        && date.chars().enumerate().all(|(index, value)| {
          if matches!(index, 4 | 7) {
            value == '-'
          } else {
            value.is_ascii_digit()
          }
        })
      {
        Some(offset)
      } else {
        None
      }
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn migrates_keep_a_changelog_one_links() {
    let content = "# Changelog\nhttps://keepachangelog.com/en/1.1.0/\n";
    assert_eq!(
      migrate_to_v2(content).unwrap(),
      "# Changelog\nhttps://keepachangelog.com/en/2.0.0/\n"
    );
  }

  #[test]
  fn cuts_over_legacy_history() {
    let content = "# Changelog\n\n## [Unreleased]\n\n## [1.0.0] - 2026-01-01\n\n- Legacy\n";
    let desired = cutover(content, "1.0.0").unwrap();
    assert!(desired.contains("Arcantry manages releases from 1.0.0."));
    assert!(desired.ends_with("## [1.0.0] - 2026-01-01\n\n- Legacy\n"));
  }
}
