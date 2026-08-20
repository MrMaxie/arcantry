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
