use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn check(root: &Path) -> Result<()> {
  let expected = parse_inventory(&fs::read_to_string(
    root.join("contracts/typescript-boundary.txt"),
  )?);
  let output = Command::new("git")
    .args([
      "ls-files",
      "--cached",
      "--others",
      "--exclude-standard",
      "--",
      "*.ts",
    ])
    .current_dir(root)
    .output()
    .context("TypeScript boundary requires Git inventory")?;
  if !output.status.success() {
    bail!(
      "TypeScript boundary could not inventory repository files: {}",
      String::from_utf8_lossy(&output.stderr).trim()
    );
  }
  let actual = String::from_utf8(output.stdout)?
    .lines()
    .map(|line| line.replace('\\', "/"))
    .filter(|line| root.join(line).is_file())
    .collect::<BTreeSet<_>>();
  validate(&expected, &actual)
}

fn parse_inventory(content: &str) -> BTreeSet<String> {
  content
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .map(str::to_owned)
    .collect()
}

fn validate(expected: &BTreeSet<String>, actual: &BTreeSet<String>) -> Result<()> {
  let unexpected = actual.difference(expected).cloned().collect::<Vec<_>>();
  let removed = expected.difference(actual).cloned().collect::<Vec<_>>();
  if !unexpected.is_empty() {
    bail!(
      "TypeScript exists outside the reviewed migration boundary: {}",
      unexpected.join(", ")
    );
  }
  if !removed.is_empty() {
    bail!(
      "Shrink contracts/typescript-boundary.txt after removing: {}",
      removed.join(", ")
    );
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rejects_new_typescript_and_requires_the_inventory_to_shrink() {
    let expected = parse_inventory("apps/docs/config.ts\ntooling/legacy.ts\n");
    assert!(validate(&expected, &expected).is_ok());

    let mut unexpected = expected.clone();
    unexpected.insert("tooling/new.ts".to_owned());
    assert!(
      validate(&expected, &unexpected)
        .unwrap_err()
        .to_string()
        .contains("tooling/new.ts")
    );

    let removed = parse_inventory("apps/docs/config.ts\n");
    assert!(
      validate(&expected, &removed)
        .unwrap_err()
        .to_string()
        .contains("tooling/legacy.ts")
    );
  }
}
