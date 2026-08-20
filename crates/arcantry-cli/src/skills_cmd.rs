use crate::{SkillDoctorOptions, SkillLinkOptions, SkillUnlinkOptions, SkillsCommand, embedded};
use anyhow::{Context, Result, bail};
use arcantry_core::{catalog, repository};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute(command: SkillsCommand, cwd: &Path) -> Result<i32> {
  match command {
    SkillsCommand::List {
      catalog_root,
      scope,
    } => {
      if scope == "private" {
        let root = repository::resolve_repository_root(cwd)?;
        for skill in catalog::list_private(&root)? {
          println!("{}\tprivate\t{}", skill.name, skill.description);
        }
      } else if scope == "public" {
        let entries = if let Some(root) = catalog_root {
          catalog::load(&if root.is_absolute() {
            root
          } else {
            cwd.join(root)
          })?
          .skills
        } else if let Ok(root) = catalog::find_root(cwd) {
          catalog::load(&root)?.skills
        } else {
          embedded::public_catalog()?.skills
        };
        for entry in entries {
          println!(
            "{}\t{}\t{}",
            entry.name,
            entry.family,
            entry.tags.join(", ")
          );
        }
      } else {
        bail!("--scope must be public or private.");
      }
      Ok(0)
    }
    SkillsCommand::Inspect {
      name,
      catalog_root,
      scope,
    } => {
      if scope == "private" {
        let skill = catalog::inspect_private(&repository::resolve_repository_root(cwd)?, &name)?;
        println!(
          "{}\n{}\nVisibility: private\nSource: {}",
          skill.name,
          skill.description,
          skill.directory.display()
        );
      } else if scope == "public" {
        let (entry, metadata) = if let Some(root) = catalog_root {
          let skill = catalog::inspect(
            &if root.is_absolute() {
              root
            } else {
              cwd.join(root)
            },
            &name,
          )?;
          (skill.entry, skill.metadata)
        } else if let Ok(root) = catalog::find_root(cwd) {
          let skill = catalog::inspect(&root, &name)?;
          (skill.entry, skill.metadata)
        } else {
          embedded::public_skill(&name)?
        };
        println!(
          "{}\n{}\nFamily: {}\nTags: {}",
          entry.name,
          metadata.summary,
          entry.family,
          entry.tags.join(", ")
        );
        for scenario in metadata.scenarios {
          println!("- {}: {}", scenario.title, scenario.outcome);
        }
      } else {
        bail!("--scope must be public or private.");
      }
      Ok(0)
    }
    SkillsCommand::Link {
      name,
      options,
      replace,
    } => {
      let options = SkillOptions::from(options);
      let operation = operation(cwd, &name, &options)?;
      let results = catalog::link(&operation.source, &name, &operation.targets, replace)?;
      if let Some(root) = operation.private_root {
        exclude_private_links(&root, &name, &operation.targets)?;
      }
      println!(
        "{}: {name}",
        if results.iter().all(|result| result.status == "unchanged") {
          "Already linked"
        } else {
          "Linked"
        }
      );
      for result in results {
        if let Some(backup) = result.backup {
          println!("Backup: {}", backup.display());
        }
      }
      Ok(0)
    }
    SkillsCommand::Unlink { name, options } => {
      let options = SkillOptions::from(options);
      let operation = operation(cwd, &name, &options)?;
      let results = catalog::unlink(&operation.source, &name, &operation.targets)?;
      println!(
        "{}: {name}",
        if results.iter().all(|result| result.status == "unchanged") {
          "Already unlinked"
        } else {
          "Unlinked"
        }
      );
      Ok(0)
    }
    SkillsCommand::Doctor { options } => doctor(cwd, options.into()),
  }
}

struct SkillOptions {
  catalog_root: Option<PathBuf>,
  scope: Option<String>,
  compat: Option<String>,
  target: Option<PathBuf>,
}

macro_rules! convert_skill_options {
  ($source:ty) => {
    impl From<$source> for SkillOptions {
      fn from(options: $source) -> Self {
        Self {
          catalog_root: options.catalog_root,
          scope: options.scope,
          compat: options.compat,
          target: options.target,
        }
      }
    }
  };
}

convert_skill_options!(SkillLinkOptions);
convert_skill_options!(SkillUnlinkOptions);
convert_skill_options!(SkillDoctorOptions);

fn doctor(cwd: &Path, options: SkillOptions) -> Result<i32> {
  validate_options(&options)?;
  if options.scope.as_deref() == Some("private") {
    let root = repository::resolve_repository_root(cwd)?;
    let public_root = resolve_catalog_root(cwd, options.catalog_root)?;
    let public: BTreeSet<_> = catalog::load(&public_root)?
      .skills
      .into_iter()
      .map(|entry| entry.name)
      .collect();
    let private = catalog::list_private(&root)?;
    let mut errors = Vec::new();
    for skill in &private {
      if public.contains(&skill.name) {
        errors.push(format!(
          "Skill name conflict: {} exists in both the public catalog and .local/skills.",
          skill.name
        ));
      }
    }
    for error in &errors {
      eprintln!("ERROR: {error}");
    }
    if errors.is_empty() {
      println!("Skill catalog is valid.");
      Ok(0)
    } else {
      Ok(1)
    }
  } else {
    let root = resolve_catalog_root(cwd, options.catalog_root)?;
    let (valid, errors, _) = catalog::validate(&root);
    for error in errors {
      eprintln!("ERROR: {error}");
    }
    if valid {
      println!("Skill catalog is valid.");
      Ok(0)
    } else {
      Ok(1)
    }
  }
}

struct Operation {
  source: PathBuf,
  targets: Vec<PathBuf>,
  private_root: Option<PathBuf>,
}

fn operation(cwd: &Path, name: &str, options: &SkillOptions) -> Result<Operation> {
  validate_options(options)?;
  let repository_root = repository::resolve_repository_root(cwd).ok();
  let (source, private_root) = if options.scope.as_deref() == Some("private") {
    let root = repository_root
      .clone()
      .context("Private skill scope requires a Git repository.")?;
    let private = catalog::inspect_private(&root, name)?;
    let public_root = resolve_catalog_root(cwd, options.catalog_root.clone())?;
    if catalog::load(&public_root)?
      .skills
      .iter()
      .any(|entry| entry.name == name)
    {
      bail!("Skill name conflict: {name} exists in both the public catalog and .local/skills.");
    }
    (private.directory, Some(root))
  } else {
    let catalog_root = resolve_catalog_root(cwd, options.catalog_root.clone())?;
    if options.scope.as_deref() == Some("repo")
      && repository_root
        .as_ref()
        .is_some_and(|root| root.join(".local").join("skills").join(name).is_dir())
    {
      bail!("Skill name conflict: {name} exists in both the public catalog and .local/skills.");
    }
    (catalog::inspect(&catalog_root, name)?.directory, None)
  };
  let targets = if let Some(target) = &options.target {
    vec![if target.is_absolute() {
      target.clone()
    } else {
      cwd.join(target)
    }]
  } else {
    let compat = options.compat.as_deref() == Some("claude");
    match options.scope.as_deref() {
      Some("user") => {
        let mut targets = vec![catalog::user_target()?];
        if compat {
          targets.push(catalog::user_claude_target()?);
        }
        targets
      }
      Some("repo" | "private") => {
        let root = repository_root.context("Repository skill scope requires a Git repository.")?;
        let mut targets = vec![catalog::repo_target(&root)];
        if compat {
          targets.push(catalog::repo_claude_target(&root));
        }
        targets
      }
      _ => bail!("Choose --scope user|repo|private or provide --target."),
    }
  };
  Ok(Operation {
    source,
    targets,
    private_root,
  })
}

fn validate_options(options: &SkillOptions) -> Result<()> {
  if options.target.is_some() && (options.scope.is_some() || options.compat.is_some()) {
    bail!("--target cannot be combined with --scope or --compat.");
  }
  if options.compat.is_some() && options.scope.is_none() {
    bail!("--compat requires --scope user|repo|private.");
  }
  if options
    .compat
    .as_deref()
    .is_some_and(|value| value != "claude")
  {
    bail!("Invalid compatibility: only claude is supported.");
  }
  if options
    .scope
    .as_deref()
    .is_some_and(|value| !matches!(value, "user" | "repo" | "private"))
  {
    bail!("--scope must be user, repo, or private.");
  }
  Ok(())
}

fn exclude_private_links(root: &Path, name: &str, targets: &[PathBuf]) -> Result<()> {
  let output = duct::cmd("git", ["rev-parse", "--git-path", "info/exclude"])
    .dir(root)
    .read()?;
  let candidate = PathBuf::from(output.trim());
  let path = if candidate.is_absolute() {
    candidate
  } else {
    root.join(candidate)
  };
  let mut content = fs::read_to_string(&path).unwrap_or_default();
  let mut entries = vec![".local/".to_owned()];
  for target in targets {
    if let Ok(relative) = target.strip_prefix(root) {
      entries.push(format!(
        "{}/{}",
        relative.to_string_lossy().replace('\\', "/"),
        name
      ));
    }
  }
  for entry in entries {
    if !content.lines().any(|line| line == entry) {
      if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
      }
      content.push_str(&entry);
      content.push('\n');
    }
  }
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::write(path, content)?;
  Ok(())
}

fn resolve_catalog_root(cwd: &Path, explicit: Option<PathBuf>) -> Result<PathBuf> {
  if let Some(path) = explicit {
    return Ok(if path.is_absolute() {
      path
    } else {
      cwd.join(path)
    });
  }
  catalog::find_root(cwd).or_else(|_| embedded::materialize_catalog())
}
