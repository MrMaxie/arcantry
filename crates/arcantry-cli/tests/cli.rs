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
fn mcp_answers_while_stdin_remains_open_without_writing_project_files() {
  use std::io::{BufRead, BufReader, Write};
  use std::process::Stdio;
  use std::sync::mpsc;
  use std::time::Duration;
  let root = tempfile::tempdir().unwrap();
  let mut child = ProcessCommand::new(env!("CARGO_BIN_EXE_arcantry"))
    .arg("--cwd")
    .arg(root.path())
    .arg("mcp")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::null())
    .spawn()
    .unwrap();
  let stdout = child.stdout.take().unwrap();
  let (send, receive) = mpsc::channel();
  let reader = std::thread::spawn(move || {
    for line in BufReader::new(stdout).lines() {
      if send.send(line).is_err() {
        break;
      }
    }
  });
  let result = (|| -> Result<(), String> {
    let input = child.stdin.as_mut().unwrap();
    for message in [
      serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"contract-test","version":"1"}}}),
      serde_json::json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
      serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"next","arguments":{}}}),
    ] {
      writeln!(input, "{message}").map_err(|e| e.to_string())?;
      input.flush().map_err(|e| e.to_string())?;
      if message.get("id").is_some() {
        let line = receive
          .recv_timeout(Duration::from_secs(10))
          .map_err(|e| e.to_string())?
          .map_err(|e| e.to_string())?;
        let response: serde_json::Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        if response.get("error").is_some() || response["result"]["isError"] == true {
          return Err(line);
        }
        if message["id"] == 2 && !line.contains("No active OpenSpec change or todo.txt task") {
          return Err(line);
        }
      }
    }
    Ok(())
  })();
  let _ = child.kill();
  let _ = child.wait();
  reader.join().unwrap();
  assert!(result.is_ok(), "{result:?}");
  assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
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
fn human_guidance_and_todo_output_escape_terminal_controls() {
  let repository = repository();
  let change = repository.path().join("openspec/changes/escape-output");
  fs::create_dir_all(&change).unwrap();
  fs::write(
    change.join("tasks.md"),
    "- [ ] Show safe \u{1b}]8;;https://example.test\u{7}output\r\n",
  )
  .unwrap();
  fs::write(
    repository.path().join("todo.txt"),
    "Review \u{1b}[31munsafe\u{7} task\r\n",
  )
  .unwrap();

  let next = arcantry()
    .args(["--cwd", repository.path().to_str().unwrap(), "next"])
    .output()
    .unwrap();
  assert!(next.status.success());
  assert!(!next.stdout.contains(&0x1b));
  assert!(!next.stdout.contains(&0x07));
  assert!(!next.stdout.contains(&0x0d));
  let next_text = String::from_utf8(next.stdout).unwrap();
  assert!(next_text.contains("\\u{1b}"));
  assert!(next_text.contains("\\u{7}"));

  let schema = repository.path().join("openspec/schemas/test/templates");
  fs::create_dir_all(&schema).unwrap();
  fs::write(
    repository.path().join("openspec/config.yaml"),
    "schema: test\n",
  )
  .unwrap();
  fs::write(
    schema.join("tasks.md"),
    "Template \u{1b}]8;;https://example.test\u{7}text\r\n",
  )
  .unwrap();
  let explain = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "explain",
      "tasks",
    ])
    .output()
    .unwrap();
  assert!(explain.status.success());
  assert!(!explain.stdout.contains(&0x1b));
  assert!(!explain.stdout.contains(&0x07));
  assert!(!explain.stdout.contains(&0x0d));
  assert!(
    String::from_utf8(explain.stdout)
      .unwrap()
      .contains("\\u{1b}")
  );

  fs::remove_dir_all(repository.path().join("openspec")).unwrap();
  let todo = arcantry()
    .args(["--cwd", repository.path().to_str().unwrap(), "todo", "list"])
    .output()
    .unwrap();
  assert!(todo.status.success());
  assert!(!todo.stdout.contains(&0x1b));
  assert!(!todo.stdout.contains(&0x07));
  assert!(!todo.stdout.contains(&0x0d));
  assert!(String::from_utf8(todo.stdout).unwrap().contains("\\u{1b}"));
}

#[test]
fn json_guidance_preserves_repository_values() {
  let repository = repository();
  let change = repository.path().join("openspec/changes/escape-output");
  fs::create_dir_all(&change).unwrap();
  fs::write(change.join("tasks.md"), "- [ ] Keep \u{1b} exact\n").unwrap();

  let output = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "next",
      "--json",
    ])
    .output()
    .unwrap();
  assert!(output.status.success());
  let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  assert_eq!(value["change"]["pending"][0], "Keep \u{1b} exact");
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

#[test]
fn rejects_relative_relocation_target_linked_outside_the_repository() {
  let repository = repository();
  let outside = tempfile::tempdir().unwrap();
  fs::write(repository.path().join("todo.txt"), "Keep this task\n").unwrap();
  let linked = repository.path().join("linked");
  #[cfg(windows)]
  junction::create(outside.path(), &linked).unwrap();
  #[cfg(not(windows))]
  std::os::unix::fs::symlink(outside.path(), &linked).unwrap();

  let output = arcantry()
    .args([
      "--cwd",
      repository.path().to_str().unwrap(),
      "repo",
      "plan",
      "--source",
      "todo-root",
      "--transition",
      "relocate",
      "--to-path",
      "linked/copied.txt",
    ])
    .output()
    .unwrap();

  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stderr).contains("Relocate target"));
  assert!(!outside.path().join("copied.txt").exists());
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

#[cfg(target_os = "linux")]
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

#[test]
fn saved_detachment_plan_preserves_source_and_refuses_stale_inputs() {
  let root = repository();
  fs::write(root.path().join("todo.txt"), "Keep exactly this task\r\n").unwrap();
  fs::write(root.path().join("arcantry.toml"), "config_version = 1\n[sources.work]\nkind = 'todo-txt'\npath = 'todo.txt'\nadapter = 'todo-txt@1'\nmanagement = 'manage'\n").unwrap();
  arcantry()
    .arg("--cwd")
    .arg(root.path())
    .args([
      "repo",
      "detach",
      "--capability",
      "source:work",
      "--output",
      "plan.json",
    ])
    .assert()
    .success();
  assert!(!root.path().join("arcantry-detached-work.md").exists());
  arcantry()
    .arg("--cwd")
    .arg(root.path())
    .args([
      "repo",
      "detach",
      "--capability",
      "source:work",
      "--output",
      "plan.json",
    ])
    .assert()
    .failure();
  fs::write(root.path().join("todo.txt"), "New input\n").unwrap();
  arcantry()
    .arg("--cwd")
    .arg(root.path())
    .args(["repo", "apply", "--plan", "plan.json"])
    .assert()
    .failure();
  fs::write(root.path().join("todo.txt"), "Keep exactly this task\r\n").unwrap();
  arcantry()
    .arg("--cwd")
    .arg(root.path())
    .args(["repo", "apply", "--plan", "plan.json"])
    .assert()
    .success();
  assert_eq!(
    fs::read(root.path().join("todo.txt")).unwrap(),
    b"Keep exactly this task\r\n"
  );
  assert!(
    !fs::read_to_string(root.path().join("arcantry.toml"))
      .unwrap()
      .contains("sources.work")
  );
  assert!(
    fs::read_to_string(root.path().join("PROJECT_CAPABILITIES.md"))
      .unwrap()
      .contains("preserve the license and attribution")
  );
}
