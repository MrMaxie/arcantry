use anyhow::{Context, Result, bail};
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogEntry {
  pub name: String,
  pub family: String,
  pub tags: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub skills: Vec<CatalogEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillScenario {
  pub title: String,
  pub prompt: String,
  pub outcome: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub summary: String,
  pub scenarios: Vec<SkillScenario>,
}
#[derive(Debug, Clone)]
pub struct SkillInspection {
  pub entry: CatalogEntry,
  pub metadata: SkillMetadata,
  pub directory: PathBuf,
}
#[derive(Debug, Clone)]
pub struct PrivateSkillInspection {
  pub name: String,
  pub description: String,
  pub directory: PathBuf,
}
#[derive(Debug, Clone)]
pub struct LinkResult {
  pub status: &'static str,
  pub source: PathBuf,
  pub target: PathBuf,
  pub backup: Option<PathBuf>,
}

pub fn load(root: &Path) -> Result<Catalog> {
  let catalog: Catalog = serde_json::from_str(&fs::read_to_string(root.join("catalog.json"))?)?;
  if catalog.schema != "./schemas/catalog.schema.json" {
    bail!("catalog.json has an unsupported schema.");
  }
  Ok(catalog)
}

pub fn inspect(root: &Path, name: &str) -> Result<SkillInspection> {
  let catalog = load(root)?;
  let entry = catalog
    .skills
    .into_iter()
    .find(|entry| entry.name == name)
    .with_context(|| format!("Skill is not present in catalog.json: {name}"))?;
  let directory = root.join("skills").join(name);
  let metadata: SkillMetadata =
    serde_json::from_str(&fs::read_to_string(directory.join("arcantry.json"))?)?;
  if metadata.schema != "../../schemas/skill-metadata.schema.json" {
    bail!("skills/{name}/arcantry.json has an unsupported schema.");
  }
  Ok(SkillInspection {
    entry,
    metadata,
    directory,
  })
}

pub fn inspect_private(root: &Path, name: &str) -> Result<PrivateSkillInspection> {
  validate_name(name)?;
  let directory = root.join(".local").join("skills").join(name);
  let skill_file = directory.join("SKILL.md");
  if !directory.is_dir() || !skill_file.is_file() {
    bail!("Private skill is missing: .local/skills/{name}/SKILL.md");
  }
  let (frontmatter_name, description) = read_frontmatter(&fs::read_to_string(skill_file)?)?;
  if frontmatter_name != name {
    bail!("Private skill frontmatter name must match .local/skills/{name}.");
  }
  Ok(PrivateSkillInspection {
    name: name.to_owned(),
    description,
    directory,
  })
}

pub fn list_private(root: &Path) -> Result<Vec<PrivateSkillInspection>> {
  let directory = root.join(".local").join("skills");
  if !directory.exists() {
    return Ok(Vec::new());
  }
  if !directory.is_dir() {
    bail!(".local/skills must be a directory.");
  }
  let mut names: Vec<_> = fs::read_dir(directory)?
    .filter_map(|entry| entry.ok())
    .filter(|entry| {
      entry
        .file_type()
        .is_ok_and(|kind| kind.is_dir() || kind.is_symlink())
    })
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .collect();
  names.sort();
  names
    .iter()
    .map(|name| inspect_private(root, name))
    .collect()
}

pub fn find_root(start: &Path) -> Result<PathBuf> {
  for directory in start.ancestors() {
    if directory.join("catalog.json").is_file() && directory.join("skills").is_dir() {
      return Ok(directory.to_path_buf());
    }
  }
  bail!(
    "No Arcantry catalog found from {}. Use --catalog-root.",
    start.display()
  )
}

pub fn validate(root: &Path) -> (bool, Vec<String>, Option<Catalog>) {
  let catalog = match load(root) {
    Ok(catalog) => catalog,
    Err(error) => return (false, vec![error.to_string()], None),
  };
  let mut errors = Vec::new();
  let names: Vec<_> = catalog
    .skills
    .iter()
    .map(|entry| entry.name.clone())
    .collect();
  let mut sorted = names.clone();
  sorted.sort();
  if names != sorted {
    errors.push("catalog.json skills must be sorted by name.".to_owned());
  }
  if names.iter().collect::<BTreeSet<_>>().len() != names.len() {
    errors.push("catalog.json skill names must be unique.".to_owned());
  }
  for entry in &catalog.skills {
    if validate_name(&entry.name).is_err() {
      errors.push(format!("skills/{} has an invalid name.", entry.name));
    }
    if entry.tags.is_empty() || entry.tags.iter().collect::<BTreeSet<_>>().len() != entry.tags.len()
    {
      errors.push(format!(
        "skills/{} tags must be non-empty and unique.",
        entry.name
      ));
    }
    let directory = root.join("skills").join(&entry.name);
    for required in ["SKILL.md", "arcantry.json", "agents/openai.yaml"] {
      if !directory.join(required).is_file() {
        errors.push(format!("skills/{} is missing {required}.", entry.name));
      }
    }
    if let Ok((name, description)) = fs::read_to_string(directory.join("SKILL.md"))
      .and_then(|source| read_frontmatter(&source).map_err(std::io::Error::other))
    {
      if name != entry.name {
        errors.push(format!(
          "skills/{} frontmatter name must match its directory.",
          entry.name
        ));
      }
      if description.len() < 30 {
        errors.push(format!("skills/{} description is too short.", entry.name));
      }
    }
    if let Ok(source) = fs::read_to_string(directory.join("agents").join("openai.yaml"))
      && !source.contains(&format!("${}", entry.name))
    {
      errors.push(format!(
        "skills/{}/agents/openai.yaml must mention ${}.",
        entry.name, entry.name
      ));
    }
  }
  (errors.is_empty(), errors, Some(catalog))
}

pub fn link(
  source: &Path,
  name: &str,
  target_roots: &[PathBuf],
  replace: bool,
) -> Result<Vec<LinkResult>> {
  validate_source(source, name)?;
  let source = dunce::canonicalize(source)?;
  let mut results = Vec::new();
  for root in unique_paths(target_roots) {
    let target = root.join(name);
    if target.exists() || fs::symlink_metadata(&target).is_ok() {
      if points_to(&target, &source) {
        results.push(LinkResult {
          status: "unchanged",
          source: source.clone(),
          target,
          backup: None,
        });
        continue;
      }
      if !replace {
        bail!(
          "{} is not an exact link. Use --replace to back it up before linking.",
          target.display()
        );
      }
      let backup = next_backup(&target);
      fs::rename(&target, &backup)?;
      if let Err(error) = create_directory_link(&source, &target) {
        fs::rename(&backup, &target).ok();
        return Err(error);
      }
      results.push(LinkResult {
        status: "linked",
        source: source.clone(),
        target,
        backup: Some(backup),
      });
    } else {
      create_directory_link(&source, &target)?;
      results.push(LinkResult {
        status: "linked",
        source: source.clone(),
        target,
        backup: None,
      });
    }
  }
  Ok(results)
}

pub fn unlink(source: &Path, name: &str, target_roots: &[PathBuf]) -> Result<Vec<LinkResult>> {
  validate_source(source, name)?;
  let source = dunce::canonicalize(source)?;
  let roots = unique_paths(target_roots);
  for root in &roots {
    let target = root.join(name);
    if (target.exists() || fs::symlink_metadata(&target).is_ok()) && !points_to(&target, &source) {
      bail!(
        "{} is not an exact link to the selected Arcantry skill; nothing was removed.",
        target.display()
      );
    }
  }
  let mut results = Vec::new();
  for root in roots {
    let target = root.join(name);
    if target.exists() || fs::symlink_metadata(&target).is_ok() {
      remove_directory_link(&target)?;
      results.push(LinkResult {
        status: "unlinked",
        source: source.clone(),
        target,
        backup: None,
      });
    } else {
      results.push(LinkResult {
        status: "unchanged",
        source: source.clone(),
        target,
        backup: None,
      });
    }
  }
  Ok(results)
}

pub fn user_target() -> Result<PathBuf> {
  Ok(
    UserDirs::new()
      .context("Could not resolve the user home directory.")?
      .home_dir()
      .join(".agents")
      .join("skills"),
  )
}
pub fn user_claude_target() -> Result<PathBuf> {
  Ok(
    UserDirs::new()
      .context("Could not resolve the user home directory.")?
      .home_dir()
      .join(".claude")
      .join("skills"),
  )
}
pub fn repo_target(root: &Path) -> PathBuf {
  root.join(".agents").join("skills")
}
pub fn repo_claude_target(root: &Path) -> PathBuf {
  root.join(".claude").join("skills")
}

fn validate_source(source: &Path, name: &str) -> Result<()> {
  let skill = source.join("SKILL.md");
  if !skill.is_file() {
    bail!("Skill package is missing SKILL.md: {}", source.display());
  }
  let (frontmatter_name, _) = read_frontmatter(&fs::read_to_string(skill)?)?;
  if frontmatter_name != name {
    bail!("Skill frontmatter name must match {name}.");
  }
  Ok(())
}
fn read_frontmatter(source: &str) -> Result<(String, String)> {
  let body = source
    .strip_prefix("---\n")
    .or_else(|| source.strip_prefix("---\r\n"))
    .context("Missing YAML frontmatter.")?;
  let end = body.find("\n---").context("Missing YAML frontmatter.")?;
  let frontmatter = &body[..end];
  let scalar = |key: &str| -> Result<String> {
    let prefix = format!("{key}:");
    let value = frontmatter
      .lines()
      .find_map(|line| line.strip_prefix(&prefix))
      .context(format!("Missing {key} in YAML frontmatter."))?
      .trim()
      .trim_matches(['\'', '"'])
      .to_owned();
    Ok(value)
  };
  Ok((scalar("name")?, scalar("description")?))
}
fn validate_name(name: &str) -> Result<()> {
  if name.is_empty()
    || name.len() > 63
    || name.split('-').any(|part| {
      part.is_empty()
        || !part
          .chars()
          .all(|value| value.is_ascii_lowercase() || value.is_ascii_digit())
    })
  {
    bail!("Invalid skill name: {name}");
  }
  Ok(())
}
fn points_to(target: &Path, source: &Path) -> bool {
  dunce::canonicalize(target)
    .ok()
    .zip(dunce::canonicalize(source).ok())
    .is_some_and(|(left, right)| {
      if cfg!(windows) {
        left
          .to_string_lossy()
          .eq_ignore_ascii_case(&right.to_string_lossy())
      } else {
        left == right
      }
    })
}
fn unique_paths(paths: &[PathBuf]) -> Vec<PathBuf> {
  let mut seen = BTreeSet::new();
  paths
    .iter()
    .filter_map(|path| {
      let absolute = if path.is_absolute() {
        path.clone()
      } else {
        std::env::current_dir().ok()?.join(path)
      };
      let key = if cfg!(windows) {
        absolute.to_string_lossy().to_lowercase()
      } else {
        absolute.to_string_lossy().into_owned()
      };
      seen.insert(key).then_some(absolute)
    })
    .collect()
}
fn next_backup(target: &Path) -> PathBuf {
  let base = PathBuf::from(format!("{}.backup", target.display()));
  if !base.exists() {
    return base;
  }
  (2..)
    .map(|index| PathBuf::from(format!("{}.backup-{index}", target.display())))
    .find(|path| !path.exists())
    .unwrap()
}
#[cfg(windows)]
fn create_directory_link(source: &Path, target: &Path) -> Result<()> {
  if let Some(parent) = target.parent() {
    fs::create_dir_all(parent)?;
  }
  junction::create(source, target)?;
  Ok(())
}
#[cfg(not(windows))]
fn create_directory_link(source: &Path, target: &Path) -> Result<()> {
  if let Some(parent) = target.parent() {
    fs::create_dir_all(parent)?;
  }
  std::os::unix::fs::symlink(source, target)?;
  Ok(())
}
#[cfg(windows)]
fn remove_directory_link(target: &Path) -> Result<()> {
  junction::delete(target)?;
  Ok(())
}
#[cfg(not(windows))]
fn remove_directory_link(target: &Path) -> Result<()> {
  fs::remove_file(target)?;
  Ok(())
}
