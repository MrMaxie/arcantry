use crate::config::{ReleaseTopology, ResolvedProject};
use anyhow::{Context, Result, bail};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ReadAuthority {
  root: PathBuf,
  allowed_external_paths: Vec<PathBuf>,
}

impl ReadAuthority {
  pub fn new(root: &Path) -> Result<Self> {
    Ok(Self {
      root: canonicalize_candidate(root)?,
      allowed_external_paths: Vec::new(),
    })
  }

  pub fn for_project(project: &ResolvedProject) -> Result<Self> {
    let root = canonicalize_candidate(&project.root)?;
    let mut allowed_external_paths = Vec::new();
    if project.allow_external_paths {
      if let Some(path) = &project.config_path {
        let resolved = canonicalize_candidate(path)?;
        if !path_is_within(&resolved, &root) {
          allowed_external_paths.push(resolved);
        }
      }
      if let Some(config) = &project.config {
        for source in config.sources.values() {
          allow_explicit_absolute(&root, &source.path, &mut allowed_external_paths)?;
        }
        if let Some(release) = &config.release {
          if release.topology == ReleaseTopology::Single {
            for path in release
              .manifests_path
              .iter()
              .chain(release.changelog_template.iter())
              .chain(release.version_sources.iter().map(|source| &source.path))
            {
              allow_explicit_absolute(&root, path, &mut allowed_external_paths)?;
            }
          } else {
            for unit in release.units.values() {
              allow_explicit_absolute(&root, &unit.manifests_path, &mut allowed_external_paths)?;
              if let Some(path) = &unit.changelog_template {
                allow_explicit_absolute(&root, path, &mut allowed_external_paths)?;
              }
              for source in &unit.version_sources {
                allow_explicit_absolute(&root, &source.path, &mut allowed_external_paths)?;
              }
            }
          }
        }
      }
    }
    allowed_external_paths.sort();
    allowed_external_paths.dedup();
    Ok(Self {
      root,
      allowed_external_paths,
    })
  }

  pub fn authorize(&self, path: &Path) -> Result<PathBuf> {
    let resolved = canonicalize_candidate(path)?;
    if path_is_within(&resolved, &self.root)
      || self
        .allowed_external_paths
        .iter()
        .any(|allowed| path_is_within(&resolved, allowed))
    {
      return Ok(resolved);
    }
    bail!(
      "Path escapes the trusted project read boundary: {}",
      path.display()
    )
  }

  pub fn read_to_string(&self, path: &Path) -> Result<String> {
    let path = self.authorize(path)?;
    fs::read_to_string(&path).with_context(|| format!("Could not read {}.", path.display()))
  }

  pub fn read_to_string_bounded(&self, path: &Path, limit: u64) -> Result<Option<String>> {
    if fs::symlink_metadata(path).is_err() {
      return Ok(None);
    }
    let path = self.authorize(path)?;
    let mut text = String::new();
    fs::File::open(&path)?
      .take(limit + 1)
      .read_to_string(&mut text)?;
    if text.len() as u64 > limit {
      bail!(
        "{} exceeds the contextual read limit of {limit} bytes.",
        path.display()
      );
    }
    Ok(Some(text))
  }

  pub fn read_dir(&self, path: &Path) -> Result<fs::ReadDir> {
    let path = self.authorize(path)?;
    Ok(fs::read_dir(path)?)
  }
}

fn allow_explicit_absolute(root: &Path, value: &str, allowed: &mut Vec<PathBuf>) -> Result<()> {
  let path = Path::new(value);
  if path.is_absolute() {
    let resolved = canonicalize_candidate(path)?;
    if !path_is_within(&resolved, root) {
      allowed.push(resolved);
    }
  }
  Ok(())
}

pub fn ensure_within(root: &Path, path: &Path, label: &str) -> Result<PathBuf> {
  let root = canonicalize_candidate(root)?;
  let path = canonicalize_candidate(path)?;
  if !path_is_within(&path, &root) {
    bail!("{label} must stay within the project.");
  }
  Ok(path)
}

pub fn canonicalize_candidate(path: &Path) -> Result<PathBuf> {
  let absolute = if path.is_absolute() {
    path.to_path_buf()
  } else {
    std::env::current_dir()?.join(path)
  };
  let normalized = crate::config::normalize_path_lexically(&absolute);
  let mut existing = normalized.as_path();
  let mut suffix = Vec::new();
  while fs::symlink_metadata(existing).is_err() {
    let name = existing
      .file_name()
      .context("Path has no existing ancestor.")?;
    suffix.push(name.to_os_string());
    existing = existing
      .parent()
      .context("Path has no existing ancestor.")?;
  }
  let mut resolved = dunce::canonicalize(existing)?;
  for component in suffix.iter().rev() {
    resolved.push(component);
  }
  Ok(crate::config::normalize_path_lexically(&resolved))
}

fn path_is_within(path: &Path, root: &Path) -> bool {
  if cfg!(windows) {
    let path = path.to_string_lossy().to_lowercase();
    let root = root.to_string_lossy().to_lowercase();
    path == root
      || path
        .strip_prefix(&root)
        .is_some_and(|suffix| suffix.starts_with(['/', '\\']))
  } else {
    path.starts_with(root)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn rejects_a_link_that_leaves_the_trusted_root() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    fs::write(outside.path().join("secret.txt"), "secret").unwrap();
    let link = root.path().join("linked");
    #[cfg(windows)]
    junction::create(outside.path(), &link).unwrap();
    #[cfg(not(windows))]
    std::os::unix::fs::symlink(outside.path(), &link).unwrap();

    let authority = ReadAuthority {
      root: canonicalize_candidate(root.path()).unwrap(),
      allowed_external_paths: Vec::new(),
    };

    assert!(
      authority
        .read_to_string(&link.join("secret.txt"))
        .unwrap_err()
        .to_string()
        .contains("trusted project read boundary")
    );
  }
}
