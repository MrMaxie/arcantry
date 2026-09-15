use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;

const PROJECTIONS: [&str; 8] = [
  ".claude-plugin",
  ".codex-plugin",
  "assets",
  "skills",
  "schemas",
  "catalog.json",
  "LICENSE",
  "README.md",
];

pub fn prepare(root: &Path) -> Result<()> {
  let root = fs::canonicalize(root).context("could not resolve the package workspace root")?;
  let package_root = root.join("packages/arcantry");
  if !package_root.is_dir() {
    bail!(
      "main package directory does not exist: {}",
      package_root.display()
    );
  }

  for name in PROJECTIONS {
    remove_projection(&package_root, &package_root.join(name))?;
  }

  copy_tree(
    &root.join(".claude-plugin"),
    &package_root.join(".claude-plugin"),
  )?;
  copy_tree(
    &root.join(".codex-plugin"),
    &package_root.join(".codex-plugin"),
  )?;
  copy_file(
    &root.join("catalog.json"),
    &package_root.join("catalog.json"),
  )?;
  copy_tree(&root.join("skills"), &package_root.join("skills"))?;
  copy_tree(&root.join("schemas"), &package_root.join("schemas"))?;
  copy_tree(
    &root.join("openspec/schemas/arcantry"),
    &package_root.join("assets/openspec"),
  )?;
  copy_file(&root.join("LICENSE"), &package_root.join("LICENSE"))?;
  copy_file(&root.join("README.md"), &package_root.join("README.md"))?;

  println!("Prepared package projections from canonical Arcantry sources.");
  Ok(())
}

fn remove_projection(package_root: &Path, path: &Path) -> Result<()> {
  if path == package_root || !path.starts_with(package_root) {
    bail!(
      "refusing to clean package projection outside {}: {}",
      package_root.display(),
      path.display()
    );
  }
  let Ok(metadata) = fs::symlink_metadata(path) else {
    return Ok(());
  };
  if metadata.file_type().is_symlink() || metadata.is_file() {
    fs::remove_file(path)?;
  } else if metadata.is_dir() {
    fs::remove_dir_all(path)?;
  }
  Ok(())
}

fn copy_tree(source: &Path, destination: &Path) -> Result<()> {
  let metadata = fs::symlink_metadata(source).with_context(|| {
    format!(
      "package projection source does not exist: {}",
      source.display()
    )
  })?;
  if metadata.file_type().is_symlink() {
    bail!(
      "package projection source cannot be a symlink: {}",
      source.display()
    );
  }
  if metadata.is_file() {
    return copy_file(source, destination);
  }
  fs::create_dir_all(destination)?;
  let mut entries = fs::read_dir(source)?.collect::<std::io::Result<Vec<_>>>()?;
  entries.sort_by_key(|entry| entry.file_name());
  for entry in entries {
    copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
  }
  Ok(())
}

fn copy_file(source: &Path, destination: &Path) -> Result<()> {
  if let Some(parent) = destination.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::copy(source, destination).with_context(|| {
    format!(
      "could not copy package projection from {}",
      source.display()
    )
  })?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn replaces_stale_package_projections_from_canonical_sources() {
    let root = tempfile::tempdir().unwrap();
    for (path, content) in [
      (".claude-plugin/plugin.json", "claude"),
      (".codex-plugin/plugin.json", "codex"),
      ("catalog.json", "catalog"),
      ("skills/example/SKILL.md", "skill"),
      ("schemas/example.json", "schema"),
      ("openspec/schemas/arcantry/schema.yaml", "openspec"),
      ("LICENSE", "license"),
      ("README.md", "readme"),
    ] {
      let path = root.path().join(path);
      fs::create_dir_all(path.parent().unwrap()).unwrap();
      fs::write(path, content).unwrap();
    }
    let package_root = root.path().join("packages/arcantry");
    fs::create_dir_all(package_root.join("skills/stale")).unwrap();
    fs::write(package_root.join("skills/stale/SKILL.md"), "stale").unwrap();

    prepare(root.path()).unwrap();

    assert_eq!(
      fs::read_to_string(package_root.join("skills/example/SKILL.md")).unwrap(),
      "skill"
    );
    assert!(!package_root.join("skills/stale").exists());
    assert_eq!(
      fs::read_to_string(package_root.join("assets/openspec/schema.yaml")).unwrap(),
      "openspec"
    );
    for name in PROJECTIONS {
      assert!(
        package_root.join(name).exists(),
        "missing projection {name}"
      );
    }
  }

  #[test]
  fn refuses_to_remove_the_package_root_or_an_outside_path() {
    let root = tempfile::tempdir().unwrap();
    let package_root = root.path().join("packages/arcantry");
    fs::create_dir_all(&package_root).unwrap();
    assert!(remove_projection(&package_root, &package_root).is_err());
    assert!(remove_projection(&package_root, root.path()).is_err());
  }
}
