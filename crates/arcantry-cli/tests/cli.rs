use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::process::Command as ProcessCommand;

fn repository() -> tempfile::TempDir {
  let directory = tempfile::tempdir().unwrap();
  assert!(
    ProcessCommand::new("git")
      .args(["init", "--quiet"])
      .current_dir(directory.path())
      .status()
      .unwrap()
      .success()
  );
  directory
}

fn arcantry() -> Command {
  cargo_bin_cmd!("arcantry")
}

fn private_skill(repository: &tempfile::TempDir, name: &str) {
  let directory = repository.path().join(".local").join("skills").join(name);
  fs::create_dir_all(&directory).unwrap();
  fs::write(
    directory.join("SKILL.md"),
    format!(
      "---\nname: {name}\ndescription: Private test skill with enough detail for validation.\n---\n"
    ),
  )
  .unwrap();
}

#[test]
fn native_binary_runs_without_language_runtimes_on_path() {
  let empty_path = tempfile::tempdir().unwrap();
  arcantry()
    .env("PATH", empty_path.path())
    .arg("--version")
    .assert()
    .success()
    .stdout("1.0.0\n");
  arcantry()
    .env("PATH", empty_path.path())
    .arg("--help")
    .assert()
    .success();
}

#[test]
fn initializes_validates_updates_and_removes_private_repository_state() {
  let repository = repository();
  arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "init",
      "--scope",
      "private",
    ])
    .assert()
    .success();
  assert!(
    fs::read_to_string(repository.path().join(".local/arcantry.toml"))
      .unwrap()
      .contains("config_version = 1")
  );
  assert!(
    fs::read_to_string(repository.path().join(".git/info/exclude"))
      .unwrap()
      .contains(".local/")
  );
  arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "validate",
    ])
    .assert()
    .success();
  arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "update",
      "--scope",
      "private",
    ])
    .assert()
    .success()
    .stdout("No changes required.\n");
  arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "remove",
      "--scope",
      "private",
    ])
    .assert()
    .success();
  assert!(!repository.path().join(".local/arcantry.toml").exists());
}

#[test]
fn applies_todo_add_and_move_without_javascript() {
  let repository = repository();
  let root = repository.path().to_str().unwrap();
  arcantry()
    .args([
      "--cwd",
      root,
      "todo",
      "add",
      "Move me +project @desk",
      "--source",
      "root",
      "--apply",
    ])
    .assert()
    .success();
  arcantry()
    .args([
      "--cwd", root, "todo", "move", "1", "--from", "root", "--to", "local", "--apply",
    ])
    .assert()
    .success();
  assert_eq!(
    fs::read_to_string(repository.path().join(".local/todo.txt")).unwrap(),
    "Move me +project @desk\n"
  );
  assert!(
    fs::read_to_string(repository.path().join(".git/info/exclude"))
      .unwrap()
      .contains(".local/")
  );
}

#[test]
fn discovers_the_project_root_from_an_implicit_nested_cwd() {
  let repository = repository();
  let nested = repository.path().join("nested");
  fs::create_dir_all(&nested).unwrap();
  fs::write(
    repository.path().join("arcantry.toml"),
    r#"config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "todo.txt"
adapter = "todo-txt@1"
"#,
  )
  .unwrap();

  arcantry()
    .current_dir(&nested)
    .args(["todo", "add", "Nested task", "--source", "tasks", "--apply"])
    .assert()
    .success();

  assert_eq!(
    fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
    "Nested task\n"
  );
  assert!(!nested.join("todo.txt").exists());
}

#[test]
fn leaves_no_partial_links_when_a_compatibility_target_is_blocked() {
  let repository = repository();
  private_skill(&repository, "private-helper");
  fs::create_dir_all(repository.path().join(".claude")).unwrap();
  fs::write(repository.path().join(".claude/skills"), "not a directory").unwrap();

  let output = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "skills",
      "link",
      "private-helper",
      "--scope",
      "private",
      "--compat",
      "claude",
    ])
    .output()
    .unwrap();
  assert!(!output.status.success());

  assert!(
    !repository
      .path()
      .join(".agents/skills/private-helper")
      .exists(),
    "{}",
    String::from_utf8_lossy(&output.stderr)
  );
}

#[test]
fn rolls_back_a_private_link_when_git_exclusion_cannot_be_updated() {
  let repository = repository();
  private_skill(&repository, "private-helper");
  let exclude = repository.path().join(".git/info/exclude");
  fs::remove_file(&exclude).unwrap();
  fs::create_dir(&exclude).unwrap();

  let output = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "skills",
      "link",
      "private-helper",
      "--scope",
      "private",
    ])
    .output()
    .unwrap();
  assert!(!output.status.success());

  assert!(
    !repository
      .path()
      .join(".agents/skills/private-helper")
      .exists(),
    "{}",
    String::from_utf8_lossy(&output.stderr)
  );
}

#[test]
fn rejects_directory_relocation_that_would_drop_an_empty_directory() {
  let repository = repository();
  fs::create_dir_all(repository.path().join("openspec/empty")).unwrap();
  fs::write(
    repository.path().join("openspec/config.yaml"),
    "schema: arcantry\n",
  )
  .unwrap();

  let output = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "plan",
      "--source",
      "openspec",
      "--transition",
      "relocate",
      "--to-path",
      "moved",
      "--delete-source",
      "--json",
    ])
    .output()
    .unwrap();

  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stdout).contains("empty directory"));
  assert!(repository.path().join("openspec/empty").is_dir());
  assert!(!repository.path().join("moved").exists());
}

#[cfg(unix)]
#[test]
fn rejects_directory_relocation_that_would_drop_a_symbolic_link() {
  use std::os::unix::fs::symlink;

  let repository = repository();
  fs::create_dir(repository.path().join("openspec")).unwrap();
  fs::write(
    repository.path().join("openspec/config.yaml"),
    "schema: arcantry\n",
  )
  .unwrap();
  symlink(
    "config.yaml",
    repository.path().join("openspec/config-reference.yaml"),
  )
  .unwrap();

  let output = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "plan",
      "--source",
      "openspec",
      "--transition",
      "relocate",
      "--to-path",
      "moved",
      "--delete-source",
      "--json",
    ])
    .output()
    .unwrap();

  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stdout).contains("symbolic link"));
  assert!(fs::symlink_metadata(repository.path().join("openspec/config-reference.yaml")).is_ok());
  assert!(!repository.path().join("moved").exists());
}

#[cfg(unix)]
#[test]
fn accepts_a_non_utf8_cwd_path() {
  use std::ffi::OsString;
  use std::os::unix::ffi::OsStringExt;

  let parent = tempfile::tempdir().unwrap();
  let repository = parent
    .path()
    .join(OsString::from_vec(b"repo-\xff".to_vec()));
  fs::create_dir(&repository).unwrap();
  assert!(
    ProcessCommand::new("git")
      .args(["init", "--quiet"])
      .current_dir(&repository)
      .status()
      .unwrap()
      .success()
  );

  arcantry()
    .arg("--cwd")
    .arg(&repository)
    .args(["repo", "inspect"])
    .assert()
    .success();
}

#[test]
fn repository_validation_uses_the_implicitly_configured_project_root() {
  let repository = repository();
  arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "init",
      "--scope",
      "shared",
    ])
    .assert()
    .success();
  let root_guidance = fs::read_to_string(repository.path().join("AGENTS.md")).unwrap();
  let config_path = repository.path().join("arcantry.toml");
  let config = fs::read_to_string(&config_path).unwrap();
  fs::write(
    &config_path,
    format!("{config}\n[project]\nroot = \"app\"\n"),
  )
  .unwrap();
  fs::create_dir(repository.path().join("app")).unwrap();

  arcantry()
    .current_dir(repository.path())
    .args(["repo", "validate"])
    .assert()
    .failure();

  fs::write(repository.path().join("app/AGENTS.md"), root_guidance).unwrap();
  arcantry()
    .current_dir(repository.path())
    .args(["repo", "validate"])
    .assert()
    .success();
}
