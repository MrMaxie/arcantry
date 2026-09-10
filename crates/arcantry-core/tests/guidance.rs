use arcantry_core::{config::resolve_project, guidance};
use std::fs;

#[test]
fn three_answers_preserve_project_rules_and_dependency_order_without_approval() {
  let root = tempfile::tempdir().unwrap();
  fs::create_dir_all(root.path().join("openspec/changes/first")).unwrap();
  fs::create_dir_all(root.path().join("openspec/changes/second")).unwrap();
  fs::create_dir_all(root.path().join("openspec/schemas/custom/templates")).unwrap();
  fs::write(root.path().join("arcantry.toml"), "config_version = 1\n[workflow]\norder = ['second', 'first']\n[workflow.dependencies]\nsecond = ['first']\n[context]\nfocus = ['documentation']\n").unwrap();
  fs::write(
    root.path().join("AGENTS.md"),
    "Preserve existing project conventions.",
  )
  .unwrap();
  fs::write(root.path().join("openspec/config.yaml"), "schema: custom\n").unwrap();
  fs::write(
    root
      .path()
      .join("openspec/schemas/custom/templates/tasks.md"),
    "# Project-specific checklist\n",
  )
  .unwrap();
  for id in ["first", "second"] {
    fs::write(
      root.path().join(format!("openspec/changes/{id}/tasks.md")),
      "- [ ] Verify the user outcome.\n",
    )
    .unwrap();
  }
  let project = resolve_project(root.path(), None, true, None).unwrap();
  let state = guidance::context(&project).unwrap();
  assert_eq!(state["changes"].as_array().unwrap().len(), 2);
  assert_eq!(
    guidance::next(&project, None).unwrap()["change"]["id"],
    "first"
  );
  let blocked = guidance::next(&project, Some("second")).unwrap();
  assert_eq!(blocked["authority"], "unknown");
  assert_eq!(blocked["blockers"].as_array().unwrap().len(), 1);
  let explanation = guidance::explain(&project, "tasks").unwrap();
  assert!(
    explanation["projectSources"]
      .to_string()
      .contains("Project-specific checklist")
  );
  assert!(
    explanation["rules"]
      .to_string()
      .contains("Preserve existing project conventions")
  );
}

#[test]
fn empty_project_needs_no_configuration_and_unknown_change_is_not_invented() {
  let root = tempfile::tempdir().unwrap();
  let project = resolve_project(root.path(), None, true, None).unwrap();
  assert_eq!(
    guidance::next(&project, None).unwrap()["command"],
    "arcantry explain proposal"
  );
  assert!(guidance::next(&project, Some("missing")).is_err());
  assert!(guidance::explain(&project, "invented-format").is_err());
  assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}
