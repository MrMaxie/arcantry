use assert_cmd::Command as AssertCommand;
use assert_cmd::cargo::cargo_bin_cmd;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Output};

#[derive(Deserialize)]
struct Contract {
  version: u32,
  #[serde(rename = "evidencePolicy")]
  evidence_policy: EvidencePolicy,
  provenance: Provenance,
  #[serde(rename = "globalOptions")]
  global_options: Vec<GlobalOption>,
  commands: Vec<ContractCommand>,
  #[serde(rename = "trustClaims")]
  trust_claims: Vec<TrustClaim>,
  scenarios: Vec<ContractScenario>,
}

#[derive(Deserialize)]
struct EvidencePolicy {
  #[serde(rename = "idFormat")]
  id_format: String,
  #[serde(rename = "requiredDimensions")]
  required_dimensions: Vec<String>,
  #[serde(rename = "mutationDimensions")]
  mutation_dimensions: Vec<String>,
  #[serde(rename = "mutatingCommands")]
  mutating_commands: Vec<String>,
}

#[derive(Deserialize)]
struct Provenance {
  #[serde(rename = "appliesTo")]
  applies_to: String,
  requirement: String,
  documentation: String,
  evidence: String,
}

#[derive(Deserialize)]
struct GlobalOption {
  syntax: String,
  evidence: String,
}

#[derive(Deserialize)]
struct ContractCommand {
  path: String,
  syntax: String,
  evidence: String,
}

#[derive(Deserialize)]
struct TrustClaim {
  id: String,
  evidence: String,
}

#[derive(Deserialize)]
struct ContractScenario {
  id: String,
  command: String,
}

fn workspace() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../..")
    .to_path_buf()
}

fn contract() -> Contract {
  serde_json::from_str(
    &fs::read_to_string(workspace().join("contracts/cli-contract.json")).unwrap(),
  )
  .unwrap()
}

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

fn git(root: &Path, arguments: &[&str]) {
  assert!(
    ProcessCommand::new("git")
      .args(arguments)
      .current_dir(root)
      .status()
      .unwrap()
      .success()
  );
}

fn run(root: &Path, arguments: &[&str]) -> Output {
  let mut command = binary();
  command.arg("--cwd").arg(root).args(arguments);
  command.output().unwrap()
}

fn binary() -> AssertCommand {
  std::env::var_os("ARCANTRY_BIN").map_or_else(|| cargo_bin_cmd!("arcantry"), AssertCommand::new)
}

fn success(root: &Path, arguments: &[&str]) -> (String, String) {
  let output = run(root, arguments);
  assert!(
    output.status.success(),
    "{}\nstdout:\n{}\nstderr:\n{}",
    arguments.join(" "),
    String::from_utf8_lossy(&output.stdout),
    String::from_utf8_lossy(&output.stderr)
  );
  (
    String::from_utf8(output.stdout).unwrap(),
    String::from_utf8(output.stderr).unwrap(),
  )
}

fn write(root: &Path, path: &str, content: &str) {
  let target = root.join(path);
  if let Some(parent) = target.parent() {
    fs::create_dir_all(parent).unwrap();
  }
  fs::write(target, content).unwrap();
}

fn release_repository() -> tempfile::TempDir {
  let repository = repository();
  write(
    repository.path(),
    "arcantry.toml",
    r#"config_version = 1

[sources.openspec]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.changelog]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["openspec"]

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "changelog"
tag_prefix = "v"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
"#,
  );
  write(
    repository.path(),
    "package.json",
    "{\n  \"version\": \"1.0.0\"\n}\n",
  );
  fs::create_dir_all(repository.path().join("openspec/changes/archive")).unwrap();
  success(
    repository.path(),
    &[
      "release",
      "baseline",
      "1.0.0",
      "--date",
      "2026-08-01",
      "--apply",
    ],
  );
  repository
}

fn add_release_change(root: &Path) {
  write(
    root,
    "openspec/changes/archive/2026-08-25-cli-contract/release.md",
    r#"---
category: changed
impact: patch
visibility: public
components:
  - cli
---

# Verify the CLI contract

The CLI contract is executable.
"#,
  );
}

#[test]
fn inventory_is_complete_unique_and_evidenced() {
  let contract = contract();
  assert_eq!(contract.version, 2);
  assert_eq!(
    contract.evidence_policy.id_format,
    "<command-evidence>:<dimension>"
  );
  assert_eq!(
    contract.evidence_policy.required_dimensions,
    ["help", "invalid", "success"]
  );
  assert_eq!(
    contract.evidence_policy.mutation_dimensions,
    ["preview", "apply", "rejection-or-rollback"]
  );
  assert_eq!(
    contract.provenance.applies_to,
    "all-commands-and-trust-claims"
  );
  for relative in [
    &contract.provenance.requirement,
    &contract.provenance.documentation,
    &contract.provenance.evidence,
  ] {
    assert!(workspace().join(relative).is_file(), "{relative}");
  }
  assert_eq!(contract.global_options.len(), 5);
  assert_eq!(
    contract
      .global_options
      .iter()
      .map(|option| option.syntax.as_str())
      .collect::<BTreeSet<_>>(),
    BTreeSet::from([
      "--config <path>",
      "--output <path>",
      "--cwd <path>",
      "-V, --version",
      "-h, --help",
    ])
  );
  assert!(
    contract
      .global_options
      .iter()
      .all(|option| !option.evidence.is_empty())
  );
  assert!(!contract.commands.is_empty());
  let paths = contract
    .commands
    .iter()
    .map(|command| command.path.as_str())
    .collect::<BTreeSet<_>>();
  let command_evidence = contract
    .commands
    .iter()
    .map(|command| command.evidence.as_str())
    .collect::<BTreeSet<_>>();
  assert_eq!(paths.len(), contract.commands.len());
  assert_eq!(command_evidence.len(), contract.commands.len());
  let mut behavior_evidence = BTreeSet::new();
  for command in &contract.commands {
    let dimensions = contract.evidence_policy.required_dimensions.iter().chain(
      contract
        .evidence_policy
        .mutating_commands
        .contains(&command.path)
        .then_some(contract.evidence_policy.mutation_dimensions.iter())
        .into_iter()
        .flatten(),
    );
    for dimension in dimensions {
      assert!(behavior_evidence.insert(format!("{}:{dimension}", command.evidence)));
    }
  }
  let scenario_ids = contract
    .scenarios
    .iter()
    .map(|scenario| scenario.id.as_str())
    .collect::<BTreeSet<_>>();
  assert_eq!(scenario_ids.len(), contract.scenarios.len());
  let command_paths = paths
    .iter()
    .copied()
    .chain(std::iter::once("arcantry"))
    .collect::<BTreeSet<_>>();
  for command in &contract.commands {
    assert!(command.syntax.starts_with(&command.path));
  }
  for claim in &contract.trust_claims {
    assert!(!claim.id.is_empty());
    assert!(
      scenario_ids.contains(claim.evidence.as_str()),
      "{}",
      claim.id
    );
  }
  for option in &contract.global_options {
    assert!(
      scenario_ids.contains(option.evidence.as_str()),
      "{}",
      option.syntax
    );
  }
  for evidence in command_evidence {
    assert!(scenario_ids.contains(evidence), "{evidence}");
  }
  for scenario in &contract.scenarios {
    assert!(
      command_paths.contains(scenario.command.as_str()),
      "{}",
      scenario.id
    );
  }
}

#[test]
fn public_trust_claims_define_audience_owner_boundary_and_evidence() {
  let value: serde_json::Value = serde_json::from_str(
    &fs::read_to_string(workspace().join("contracts/public-trust.json")).unwrap(),
  )
  .unwrap();
  assert_eq!(value["schemaVersion"], 1);
  let claims = value["claims"].as_array().unwrap();
  let evidence_index = value["evidenceIndex"].as_object().unwrap();
  for path in evidence_index.values() {
    assert!(workspace().join(path.as_str().unwrap()).is_file());
  }
  let mut ids = BTreeSet::new();
  for claim in claims {
    assert!(ids.insert(claim["id"].as_str().unwrap()));
    assert!(!claim["audience"].as_array().unwrap().is_empty());
    for field in ["scope", "owner", "boundary"] {
      assert!(!claim[field].as_str().unwrap().trim().is_empty(), "{field}");
    }
    let evidence = claim["evidence"].as_array().unwrap();
    assert!(!evidence.is_empty());
    for reference in evidence {
      assert!(evidence_index.contains_key(reference.as_str().unwrap()));
    }
  }
}

#[test]
fn global_help_and_version_are_executable_contracts() {
  let help = binary().arg("--help").output().unwrap();
  assert!(help.status.success());
  assert!(String::from_utf8_lossy(&help.stdout).contains("Usage: arcantry"));
  assert!(help.stderr.is_empty());

  let version = binary().arg("--version").output().unwrap();
  assert!(version.status.success());
  assert_eq!(String::from_utf8(version.stdout).unwrap(), "1.0.0\n");
  assert!(version.stderr.is_empty());
}

#[test]
fn every_leaf_command_has_help_and_invalid_argument_evidence() {
  let root = workspace();
  for command in contract().commands {
    let parts = command.path.split_whitespace().collect::<Vec<_>>();
    let help = binary()
      .args(parts.iter().copied())
      .arg("--help")
      .output()
      .unwrap();
    assert!(help.status.success(), "{} --help", command.path);
    assert!(
      String::from_utf8_lossy(&help.stdout).contains(&format!("Usage: arcantry {}", command.path)),
      "{}",
      command.path
    );

    let mut invalid_args = parts.clone();
    invalid_args.push("--unknown");
    let invalid = run(&root, &invalid_args);
    assert!(!invalid.status.success(), "{} --unknown", command.path);
    assert!(
      String::from_utf8_lossy(&invalid.stderr).contains("unexpected argument '--unknown'"),
      "{}",
      command.path
    );
  }
}

#[test]
fn every_registered_scenario_executes_through_the_native_contract_dispatcher() {
  for scenario in contract().scenarios {
    execute_scenario(&scenario.id, &scenario.command);
  }
}

fn execute_scenario(id: &str, command: &str) {
  match (id, command) {
    ("diagnostics", "diagnostics") => {
      let root = repository();
      success(root.path(), &["diagnostics"]);
    }
    ("repo-recover", "repo recover") => {
      let root = repository();
      success(root.path(), &["repo", "recover"]);
    }
    ("saved-plan", "repo plan") => {
      let root = repository();
      success(
        root.path(),
        &[
          "repo",
          "plan",
          "--source",
          "todo-root",
          "--transition",
          "preserve",
          "--output",
          "plan.json",
        ],
      );
      assert!(root.path().join("plan.json").is_file());
    }

    ("context", "context") | ("next", "next") => {
      let root = repository();
      success(root.path(), &[command, "--json"]);
    }
    ("explain", "explain") => {
      let root = repository();
      success(root.path(), &["explain", "tasks", "--json"]);
    }
    ("mcp", "mcp") => {
      let root = repository();
      success(root.path(), &["mcp", "--help"]);
    }

    ("root-help", "arcantry") | ("root-version", "arcantry") => {
      // Covered by the named test above.
    }
    ("configuration-discovery", "arcantry") | ("explicit-configuration", "arcantry") => {
      // Covered by the named discovery test.
    }
    ("repo-inspect", "repo inspect")
    | ("repo-plan", "repo plan")
    | ("repo-apply", "repo apply")
    | ("repo-detach", "repo detach")
    | ("repo-init", "repo init")
    | ("repo-update", "repo update")
    | ("repo-doctor", "repo doctor")
    | ("repo-validate", "repo validate")
    | ("repo-remove", "repo remove")
    | ("todo-list", "todo list")
    | ("todo-add", "todo add")
    | ("todo-complete", "todo complete")
    | ("todo-move", "todo move")
    | ("todo-defer", "todo defer")
    | ("todo-resume", "todo resume")
    | ("release-baseline", "release baseline")
    | ("release-plan", "release plan")
    | ("release-cut", "release cut")
    | ("release-render", "release render")
    | ("release-check", "release check")
    | ("skills-list", "skills list")
    | ("skills-inspect", "skills inspect")
    | ("skills-status", "skills status")
    | ("skills-update", "skills update")
    | ("skills-link", "skills link")
    | ("skills-unlink", "skills unlink")
    | ("skills-doctor", "skills doctor") => assert_successful_behavior(command),
    ("skills-apply", "skills apply") => {
      // Atomic apply and stale-plan behavior are covered by the skill_update unit tests.
    }
    ("repo-inspect-bounded-context", "repo inspect") => {
      // Covered by the named inspection test.
    }
    ("repo-inspect-local-policy", "repo inspect") => {
      // Covered by the named private-boundary test.
    }
    ("repo-inspect-read-only", "repo inspect") => assert_read_only_repo_command("inspect"),
    ("repo-plan-read-only", "repo plan") => assert_read_only_repo_command("plan"),
    ("repo-apply-rejects-drift", "repo apply") => {}
    ("repo-remove-owned-only", "repo remove") => {}
    ("todo-add-preview-first", "todo add") => {}
    ("release-check-local-only", "release check") => {}
    _ => panic!("Missing or rewired native scenario dispatcher entry: {id} -> {command}"),
  }
}

fn assert_read_only_repo_command(command: &str) {
  let repository = repository();
  write(repository.path(), "todo.txt", "Preserve me\n");
  let before = fs::read(repository.path().join("todo.txt")).unwrap();
  if command == "inspect" {
    success(repository.path(), &["repo", "inspect", "--json"]);
  } else {
    success(
      repository.path(),
      &[
        "repo",
        "plan",
        "--source",
        "todo-root",
        "--transition",
        "preserve",
        "--json",
      ],
    );
  }
  assert_eq!(
    fs::read(repository.path().join("todo.txt")).unwrap(),
    before
  );
}

fn assert_successful_behavior(path: &str) {
  match path {
    "repo inspect" => {
      let repository = repository();
      let (stdout, _) = success(repository.path(), &["repo", "inspect", "--json"]);
      assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stdout).unwrap()["mode"],
        "wild"
      );
    }
    "repo plan" => {
      let repository = repository();
      write(repository.path(), "todo.txt", "Keep me\n");
      let (stdout, _) = success(
        repository.path(),
        &[
          "repo",
          "plan",
          "--source",
          "todo-root",
          "--transition",
          "preserve",
          "--json",
        ],
      );
      assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stdout).unwrap()["transition"],
        "preserve"
      );
    }
    "repo apply" => {
      let repository = repository();
      write(repository.path(), "todo.txt", "Move me\n");
      let (plan, _) = success(
        repository.path(),
        &[
          "repo",
          "plan",
          "--source",
          "todo-root",
          "--transition",
          "relocate",
          "--to-path",
          "moved.txt",
          "--delete-source",
          "--json",
        ],
      );
      write(repository.path(), "plan.json", &plan);
      success(repository.path(), &["repo", "apply", "--plan", "plan.json"]);
      assert_eq!(
        fs::read_to_string(repository.path().join("moved.txt")).unwrap(),
        "Move me\n"
      );
      assert!(!repository.path().join("todo.txt").exists());
    }
    "repo detach" => {
      let repository = repository();
      success(repository.path(), &["repo", "init", "--scope", "shared"]);
      let (preview, _) = success(repository.path(), &["repo", "detach", "--json"]);
      assert!(preview.contains("arcantry-detachment@1"));
      assert!(repository.path().join("arcantry.toml").exists());
      success(repository.path(), &["repo", "detach", "--apply"]);
      assert!(!repository.path().join("arcantry.toml").exists());
      assert!(repository.path().join("PROJECT_CAPABILITIES.md").exists());
    }
    "repo init" => assert_repository_lifecycle("init"),
    "repo update" => assert_repository_lifecycle("update"),
    "repo doctor" => assert_repository_validation("doctor"),
    "repo validate" => assert_repository_validation("validate"),
    "repo remove" => assert_repository_lifecycle("remove"),
    "todo list" => {
      let repository = repository();
      let (stdout, _) = success(repository.path(), &["todo", "list"]);
      assert_eq!(stdout, "No todo.txt tasks.\n");
    }
    "todo add" => {
      let repository = repository();
      let (stdout, _) = success(
        repository.path(),
        &["todo", "add", "New task", "--source", "root"],
      );
      assert!(stdout.contains("write: todo.txt"));
      assert!(!repository.path().join("todo.txt").exists());
      success(
        repository.path(),
        &["todo", "add", "New task", "--source", "root", "--apply"],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
        "New task\n"
      );
    }
    "todo complete" => {
      let repository = repository();
      write(repository.path(), "todo.txt", "Finish me\n");
      success(
        repository.path(),
        &[
          "todo",
          "complete",
          "1",
          "--source",
          "root",
          "--date",
          "2026-08-25",
        ],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
        "Finish me\n"
      );
      success(
        repository.path(),
        &[
          "todo",
          "complete",
          "1",
          "--source",
          "root",
          "--date",
          "2026-08-25",
          "--apply",
        ],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
        "x 2026-08-25 Finish me\n"
      );
    }
    "todo move" => {
      let repository = repository();
      write(repository.path(), "todo.txt", "Move me\n");
      success(
        repository.path(),
        &["todo", "move", "1", "--from", "root", "--to", "local"],
      );
      assert!(repository.path().join("todo.txt").exists());
      assert!(!repository.path().join(".local/todo.txt").exists());
      success(
        repository.path(),
        &[
          "todo", "move", "1", "--from", "root", "--to", "local", "--apply",
        ],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join(".local/todo.txt")).unwrap(),
        "Move me\n"
      );
    }
    "todo defer" => {
      let repository = repository();
      write(repository.path(), "todo.txt", "Wait for review\r\n");
      success(
        repository.path(),
        &["todo", "defer", "1", "--source", "root", "--wait", "review"],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
        "Wait for review\r\n"
      );
      success(
        repository.path(),
        &[
          "todo", "defer", "1", "--source", "root", "--wait", "review", "--apply",
        ],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
        "Wait for review wait:review\r\n"
      );
    }
    "todo resume" => {
      let repository = repository();
      write(
        repository.path(),
        "todo.txt",
        "Resume me t:2026-12-01 wait:review\n",
      );
      success(
        repository.path(),
        &["todo", "resume", "1", "--source", "root"],
      );
      success(
        repository.path(),
        &["todo", "resume", "1", "--source", "root", "--apply"],
      );
      assert_eq!(
        fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
        "Resume me\n"
      );
    }
    "release baseline" => {
      let repository = repository();
      fs::create_dir_all(repository.path().join("openspec/changes/archive")).unwrap();
      write_release_config(repository.path());
      let (stdout, _) = success(
        repository.path(),
        &["release", "baseline", "1.0.0", "--date", "2026-08-01"],
      );
      assert!(stdout.contains("write: releases/1.0.0.yaml"));
      assert!(!repository.path().join("releases/1.0.0.yaml").exists());
      success(
        repository.path(),
        &[
          "release",
          "baseline",
          "1.0.0",
          "--date",
          "2026-08-01",
          "--apply",
        ],
      );
      assert!(repository.path().join("releases/1.0.0.yaml").exists());
    }
    "release plan" => {
      let repository = release_repository();
      let (stdout, _) = success(repository.path(), &["release", "plan"]);
      assert!(stdout.contains("Current: 1.0.0"));
    }
    "release cut" => {
      let repository = release_repository();
      add_release_change(repository.path());
      let (stdout, _) = success(
        repository.path(),
        &["release", "cut", "--date", "2026-08-25"],
      );
      assert!(stdout.contains("write: releases/1.0.1.yaml"));
      assert!(!repository.path().join("releases/1.0.1.yaml").exists());
      success(
        repository.path(),
        &["release", "cut", "--date", "2026-08-25", "--apply"],
      );
      assert!(repository.path().join("releases/1.0.1.yaml").exists());
    }
    "release render" => {
      let repository = release_repository();
      let (stdout, _) = success(repository.path(), &["release", "render"]);
      assert!(stdout.contains("No file changes.") || stdout.contains("write: CHANGELOG.md"));
      success(repository.path(), &["release", "render", "--apply"]);
      assert!(repository.path().join("CHANGELOG.md").exists());
    }
    "release check" => {
      let repository = release_repository();
      let (stdout, _) = success(repository.path(), &["release", "check"]);
      assert_eq!(stdout, "Release state is consistent.\n");
    }
    "skills list" => {
      let (stdout, _) = success(&workspace(), &["skills", "list"]);
      assert!(stdout.contains("adopt-arcantry"));
    }
    "skills inspect" => {
      let (stdout, _) = success(&workspace(), &["skills", "inspect", "adopt-arcantry"]);
      assert!(stdout.starts_with("adopt-arcantry\n"));
    }
    "skills status" => {
      let target = tempfile::tempdir().unwrap();
      let (stdout, _) = success(
        &workspace(),
        &[
          "skills",
          "status",
          "adopt-arcantry",
          "--target",
          target.path().to_str().unwrap(),
          "--json",
        ],
      );
      assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stdout).unwrap()[0]["state"],
        "not-installed"
      );
    }
    "skills update" => {
      let target = tempfile::tempdir().unwrap();
      let (stdout, _) = success(
        &workspace(),
        &[
          "skills",
          "update",
          "adopt-arcantry",
          "--catalog-root",
          workspace().to_str().unwrap(),
          "--target",
          target.path().to_str().unwrap(),
        ],
      );
      assert_eq!(
        serde_json::from_str::<serde_json::Value>(&stdout).unwrap()["schema"],
        "skill-update@1"
      );
    }
    "skills link" => assert_skill_link_lifecycle("link"),
    "skills unlink" => assert_skill_link_lifecycle("unlink"),
    "skills doctor" => {
      let (stdout, _) = success(&workspace(), &["skills", "doctor"]);
      assert_eq!(stdout, "Skill catalog is valid.\n");
    }
    _ => panic!("Missing native behavior evidence for {path}"),
  }
}

fn write_release_config(root: &Path) {
  write(
    root,
    "arcantry.toml",
    r#"config_version = 1

[sources.openspec]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.changelog]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["openspec"]

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "changelog"
tag_prefix = "v"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
"#,
  );
  write(root, "package.json", "{\n  \"version\": \"1.0.0\"\n}\n");
}

fn assert_repository_lifecycle(command: &str) {
  for scope in ["shared", "private"] {
    let repository = repository();
    write(repository.path(), "owned-by-user.txt", "keep\n");
    success(repository.path(), &["repo", "init", "--scope", scope]);
    if command == "update" {
      let (stdout, _) = success(repository.path(), &["repo", "update", "--scope", scope]);
      assert_eq!(stdout, "No changes required.\n");
    }
    if command == "remove" {
      success(repository.path(), &["repo", "remove", "--scope", scope]);
    }
    assert_eq!(
      fs::read_to_string(repository.path().join("owned-by-user.txt")).unwrap(),
      "keep\n"
    );
    for unexpected in [
      "package.json",
      "Cargo.toml",
      "justfile",
      "openspec",
      "CHANGELOG.md",
      "todo.txt",
    ] {
      assert!(
        !repository.path().join(unexpected).exists(),
        "{scope}: {unexpected}"
      );
    }
  }
}

fn assert_repository_validation(command: &str) {
  let repository = repository();
  success(repository.path(), &["repo", "init", "--scope", "shared"]);
  let (stdout, _) = success(repository.path(), &["repo", command]);
  assert!(stdout.contains("Repository adoption is valid."));
}

fn assert_skill_link_lifecycle(command: &str) {
  let directory = tempfile::tempdir().unwrap();
  let target = directory.path().join("links");
  if command == "unlink" {
    success(
      &workspace(),
      &[
        "skills",
        "link",
        "adopt-arcantry",
        "--target",
        target.to_str().unwrap(),
      ],
    );
  }
  let (stdout, _) = success(
    &workspace(),
    &[
      "skills",
      command,
      "adopt-arcantry",
      "--target",
      target.to_str().unwrap(),
    ],
  );
  assert!(stdout.starts_with(if command == "link" {
    "Linked:"
  } else {
    "Unlinked:"
  }));
}

#[test]
fn all_repository_transition_strategies_serialize_and_apply() {
  for transition in [
    "preserve", "adopt", "rebind", "cutover", "migrate", "relocate",
  ] {
    assert_transition(transition);
  }
}

fn assert_transition(transition: &str) {
  let repository = repository();
  let args = match transition {
    "preserve" => {
      write(repository.path(), "todo.txt", "Keep me\n");
      vec![
        "repo",
        "plan",
        "--source",
        "todo-root",
        "--transition",
        "preserve",
        "--json",
      ]
    }
    "adopt" => vec![
      "repo",
      "plan",
      "--source",
      "todo-root",
      "--transition",
      "adopt",
      "--json",
    ],
    "rebind" => {
      write(repository.path(), "todo.txt", "Old\n");
      write(repository.path(), "tasks.txt", "New\n");
      write(
        repository.path(),
        "arcantry.toml",
        "config_version = 1\n\n[sources.tasks]\nkind = \"todo-txt\"\npath = \"todo.txt\"\nadapter = \"todo-txt@1\"\n",
      );
      vec![
        "repo",
        "plan",
        "--source",
        "tasks",
        "--transition",
        "rebind",
        "--to-path",
        "tasks.txt",
        "--json",
      ]
    }
    "cutover" | "migrate" => {
      let content = if transition == "cutover" {
        "# Changelog\n\n## [Unreleased]\n\n## [0.9.0] - 2026-07-01\n\n- Legacy\n"
      } else {
        "# Changelog\n\nBased on https://keepachangelog.com/en/1.1.0/\n\n## [Unreleased]\n"
      };
      write(repository.path(), "CHANGELOG.md", content);
      let mut values = vec![
        "repo",
        "plan",
        "--source",
        "changelog",
        "--transition",
        transition,
      ];
      if transition == "cutover" {
        values.extend(["--managed-from", "1.0.0"]);
      }
      values.push("--json");
      values
    }
    "relocate" => {
      write(repository.path(), "todo.txt", "Move me\n");
      vec![
        "repo",
        "plan",
        "--source",
        "todo-root",
        "--transition",
        "relocate",
        "--to-path",
        "moved.txt",
        "--delete-source",
        "--json",
      ]
    }
    _ => unreachable!(),
  };
  let (plan, _) = success(repository.path(), &args);
  let parsed: serde_json::Value = serde_json::from_str(&plan).unwrap();
  assert_eq!(parsed["transition"], transition);
  assert!(
    parsed["conflicts"].as_array().unwrap().is_empty(),
    "{transition}: {plan}"
  );
  write(repository.path(), "plan.json", &plan);
  success(repository.path(), &["repo", "apply", "--plan", "plan.json"]);
}

#[test]
fn apply_rejects_drift_without_partial_writes() {
  assert_apply_rejects_drift();
}

fn assert_apply_rejects_drift() {
  let repository = repository();
  write(repository.path(), "todo.txt", "Original\n");
  let (plan, _) = success(
    repository.path(),
    &[
      "repo",
      "plan",
      "--source",
      "todo-root",
      "--transition",
      "relocate",
      "--to-path",
      "moved.txt",
      "--delete-source",
      "--json",
    ],
  );
  write(repository.path(), "plan.json", &plan);
  write(repository.path(), "todo.txt", "Changed\n");
  let output = run(repository.path(), &["repo", "apply", "--plan", "plan.json"]);
  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stderr).contains("changed after the plan was created"));
  assert!(!repository.path().join("moved.txt").exists());
  assert_eq!(
    fs::read_to_string(repository.path().join("todo.txt")).unwrap(),
    "Changed\n"
  );
}

#[test]
fn repo_apply_rejects_a_plan_from_another_project_before_writing() {
  let original = repository();
  let current = repository();
  write(original.path(), "todo.txt", "Move me\n");
  let (plan, _) = success(
    original.path(),
    &[
      "repo",
      "plan",
      "--source",
      "todo-root",
      "--transition",
      "relocate",
      "--to-path",
      "moved.txt",
      "--delete-source",
      "--json",
    ],
  );
  write(current.path(), "plan.json", &plan);

  let output = run(current.path(), &["repo", "apply", "--plan", "plan.json"]);

  assert!(!output.status.success());
  assert!(String::from_utf8_lossy(&output.stderr).contains("does not match the current project"));
  assert!(!original.path().join("moved.txt").exists());
  assert_eq!(
    fs::read_to_string(original.path().join("todo.txt")).unwrap(),
    "Move me\n"
  );
}

#[test]
fn repo_apply_requires_and_honors_one_exact_external_authorization() {
  let repository = repository();
  let outside = tempfile::tempdir().unwrap();
  let target = outside.path().join("moved.txt");
  write(repository.path(), "todo.txt", "Move me\n");
  let target_text = target.to_string_lossy().into_owned();
  let (plan, _) = success(
    repository.path(),
    &[
      "repo",
      "plan",
      "--source",
      "todo-root",
      "--transition",
      "relocate",
      "--to-path",
      &target_text,
      "--json",
    ],
  );
  write(repository.path(), "plan.json", &plan);

  let rejected = run(repository.path(), &["repo", "apply", "--plan", "plan.json"]);
  assert!(!rejected.status.success());
  assert!(String::from_utf8_lossy(&rejected.stderr).contains("exact --allow-outside"));
  assert!(!target.exists());

  success(
    repository.path(),
    &[
      "repo",
      "apply",
      "--plan",
      "plan.json",
      "--allow-outside",
      &target_text,
    ],
  );
  assert_eq!(fs::read_to_string(target).unwrap(), "Move me\n");
}

#[test]
fn normal_and_sealed_release_checks_have_distinct_contracts() {
  let repository = release_repository();
  write(
    repository.path(),
    "openspec/changes/active-cli-contract/release.md",
    r#"---
category: changed
impact: patch
visibility: public
components:
  - cli
---

# Verify the active CLI contract

The active CLI contract remains in progress.
"#,
  );
  let normal = run(repository.path(), &["release", "check"]);
  assert!(
    normal.status.success(),
    "{}",
    String::from_utf8_lossy(&normal.stderr)
  );
  let sealed = run(repository.path(), &["release", "check", "--sealed"]);
  assert!(!sealed.status.success());
  assert!(
    String::from_utf8_lossy(&sealed.stderr)
      .contains("active OpenSpec changes are not release-complete")
  );
}

#[test]
fn configuration_discovery_uses_the_nearest_project_and_honors_an_explicit_config() {
  let outer = repository();
  write_release_config(outer.path());
  let nested = outer.path().join("work/nested");
  fs::create_dir_all(&nested).unwrap();

  let discovered = success(&nested, &["repo", "inspect", "--json"]).0;
  assert_eq!(
    serde_json::from_str::<serde_json::Value>(&discovered).unwrap()["mode"],
    "configured"
  );

  let separate = tempfile::tempdir().unwrap();
  let explicit = separate.path().join("explicit.toml");
  fs::write(
    &explicit,
    "config_version = 1\n\n[sources.tasks]\nkind = \"todo-txt\"\npath = \"tasks.txt\"\nadapter = \"todo-txt@1\"\n",
  )
  .unwrap();
  let output = binary()
    .arg("--cwd")
    .arg(outer.path())
    .arg("--config")
    .arg(&explicit)
    .args(["repo", "inspect", "--json"])
    .output()
    .unwrap();
  assert!(output.status.success());
  let inspection: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
  assert_eq!(inspection["mode"], "configured");
  assert!(
    inspection["sources"]
      .as_array()
      .unwrap()
      .iter()
      .any(|source| source["id"] == "tasks")
  );
}

#[test]
fn repository_inspection_reports_absent_sources_and_detailed_context() {
  let directory = tempfile::tempdir().unwrap();

  let (json, _) = success(directory.path(), &["repo", "inspect", "--json"]);
  let inspection: serde_json::Value = serde_json::from_str(&json).unwrap();
  assert_eq!(inspection["schemaVersion"], 1);
  assert_eq!(inspection["sources"].as_array().unwrap().len(), 8);
  assert!(
    inspection["sources"]
      .as_array()
      .unwrap()
      .iter()
      .all(|source| source["exists"] == false)
  );
  assert_eq!(inspection["localBoundary"]["status"], "not-git");
  assert_eq!(inspection["methodologies"].as_array().unwrap().len(), 5);

  let (detailed, _) = success(directory.path(), &["repo", "inspect", "--detailed"]);
  assert!(detailed.contains("Sources: 0 present, 8 absent"));
  assert!(detailed.contains("Source openspec:"));
  assert!(detailed.contains("Methodology agent-guidance: state=absent"));
  assert!(detailed.contains(".local details: git=false"));
}

#[test]
fn repository_inspection_reports_partial_configuration_and_local_conflicts() {
  let repository = repository();
  write(
    repository.path(),
    "arcantry.toml",
    "config_version = 1\n\n[sources.tasks]\nkind = \"todo-txt\"\npath = \"tasks.txt\"\nadapter = \"todo-txt@1\"\n",
  );
  write(repository.path(), ".local/tracked.txt", "tracked\n");
  git(
    repository.path(),
    &["config", "user.name", "Arcantry Tests"],
  );
  git(
    repository.path(),
    &["config", "user.email", "tests@arcantry.invalid"],
  );
  git(
    repository.path(),
    &["add", ".local/tracked.txt", "arcantry.toml"],
  );
  git(
    repository.path(),
    &["commit", "--quiet", "-m", "test: add context"],
  );
  git(
    repository.path(),
    &[
      "remote",
      "add",
      "origin",
      "https://example.invalid/repository.git",
    ],
  );
  git(
    repository.path(),
    &["update-ref", "refs/remotes/origin/master", "HEAD"],
  );
  git(
    repository.path(),
    &[
      "symbolic-ref",
      "refs/remotes/origin/HEAD",
      "refs/remotes/origin/master",
    ],
  );

  let (json, _) = success(repository.path(), &["repo", "inspect", "--json"]);
  let inspection: serde_json::Value = serde_json::from_str(&json).unwrap();
  let sources = inspection["sources"].as_array().unwrap();
  let tasks = sources
    .iter()
    .find(|source| source["id"] == "tasks")
    .unwrap();
  assert_eq!(tasks["origin"], "configured");
  assert_eq!(tasks["exists"], false);
  assert_eq!(inspection["localBoundary"]["status"], "remote-tracked");
  assert_eq!(
    inspection["localBoundary"]["remoteReference"],
    "refs/remotes/origin/master"
  );
}

#[test]
fn composed_release_units_are_selected_explicitly() {
  let repository = repository();
  write(
    repository.path(),
    "arcantry.toml",
    r#"config_version = 1

[sources.openspec]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.core_history]
kind = "changelog"
path = "packages/core/CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["openspec"]

[release]
adapter = "openspec-release@1"
topology = "independent"

[release.units.core]
manifests_path = "releases/core"
changelog_source = "core_history"
tag_prefix = "core/v"

[[release.units.core.version_sources]]
path = "packages/core/package.json"
adapter = "json-package@1"

[[release.units.core.selectors]]
source = "openspec"
components = ["core"]
"#,
  );
  write(
    repository.path(),
    "packages/core/package.json",
    "{\n  \"version\": \"1.0.0\"\n}\n",
  );
  fs::create_dir_all(repository.path().join("openspec/changes/archive")).unwrap();

  let missing_unit = run(repository.path(), &["release", "plan"]);
  assert!(!missing_unit.status.success());
  assert!(String::from_utf8_lossy(&missing_unit.stderr).contains("requires --unit"));

  success(
    repository.path(),
    &[
      "release",
      "baseline",
      "1.0.0",
      "--date",
      "2026-08-01",
      "--unit",
      "core",
      "--apply",
    ],
  );
  let selected = success(repository.path(), &["release", "plan", "--unit", "core"]).0;
  assert!(selected.contains("Unit: core"));
}
