use arcantry_cli::cli::Cli;
use clap::Parser;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser as MarkdownParser, Tag, TagEnd};
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct Contract {
  #[serde(rename = "globalOptions")]
  global_options: Vec<GlobalOption>,
  commands: Vec<ContractCommand>,
  #[serde(rename = "trustClaims")]
  trust_claims: Vec<TrustClaim>,
  scenarios: Vec<ContractScenario>,
}

#[derive(Deserialize)]
struct GlobalOption {
  syntax: String,
}

#[derive(Deserialize)]
struct ContractCommand {
  syntax: String,
}

#[derive(Deserialize)]
struct TrustClaim {
  id: String,
  evidence: String,
}

#[derive(Deserialize)]
struct ContractScenario {
  id: String,
}

fn workspace() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../..")
    .to_path_buf()
}

fn documentation_files(root: &Path, files: &mut Vec<PathBuf>) {
  for entry in fs::read_dir(root).unwrap() {
    let path = entry.unwrap().path();
    if path.is_dir() {
      documentation_files(&path, files);
    } else if matches!(
      path.extension().and_then(|value| value.to_str()),
      Some("md" | "mdx")
    ) {
      files.push(path);
    }
  }
}

fn contract() -> Contract {
  serde_json::from_str(
    &fs::read_to_string(workspace().join("contracts/cli-contract.json")).unwrap(),
  )
  .unwrap()
}

fn markdown(source: &str) -> MarkdownParser<'_> {
  MarkdownParser::new_ext(source, Options::ENABLE_TABLES)
}

fn normalize_syntax(value: &str) -> String {
  value
    .trim()
    .strip_prefix("arcantry ")
    .unwrap_or(value.trim())
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
}

fn is_command_syntax(value: &str) -> bool {
  ["repo ", "todo ", "release ", "skills "]
    .iter()
    .any(|prefix| value.starts_with(prefix))
}

fn code_block_commands(block: &str) -> Vec<String> {
  let mut commands = Vec::new();
  let mut current = String::new();
  for line in block.lines().map(str::trim).filter(|line| !line.is_empty()) {
    if line.starts_with("arcantry ") {
      if !current.is_empty() {
        commands.push(current);
      }
      current = line.to_owned();
    } else if !current.is_empty() {
      current.push(' ');
      current.push_str(line);
    }
  }
  if !current.is_empty() {
    commands.push(current);
  }
  commands
}

fn documented_syntaxes(source: &str) -> BTreeSet<String> {
  let mut syntaxes = BTreeSet::new();
  let mut code_block = None::<String>;
  let mut table_cell_index = 0;
  let mut in_first_table_cell = false;

  for event in markdown(source) {
    match event {
      Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => code_block = Some(String::new()),
      Event::Text(text) if code_block.is_some() => code_block.as_mut().unwrap().push_str(&text),
      Event::End(TagEnd::CodeBlock) => {
        for command in code_block_commands(&code_block.take().unwrap_or_default()) {
          let syntax = normalize_syntax(&command);
          if is_command_syntax(&syntax) {
            syntaxes.insert(syntax);
          }
        }
      }
      Event::Start(Tag::TableHead | Tag::TableRow) => table_cell_index = 0,
      Event::Start(Tag::TableCell) => in_first_table_cell = table_cell_index == 0,
      Event::End(TagEnd::TableCell) => {
        table_cell_index += 1;
        in_first_table_cell = false;
      }
      Event::Code(code) if in_first_table_cell => {
        let syntax = normalize_syntax(&code);
        if is_command_syntax(&syntax) {
          syntaxes.insert(syntax);
        }
      }
      _ => {}
    }
  }

  syntaxes
}

fn documented_global_options(source: &str) -> BTreeSet<String> {
  let mut options = BTreeSet::new();
  let mut table_cell_index = 0;
  let mut in_first_table_cell = false;
  for event in markdown(source) {
    match event {
      Event::Start(Tag::TableHead | Tag::TableRow) => table_cell_index = 0,
      Event::Start(Tag::TableCell) => in_first_table_cell = table_cell_index == 0,
      Event::End(TagEnd::TableCell) => {
        table_cell_index += 1;
        in_first_table_cell = false;
      }
      Event::Code(code) if in_first_table_cell => {
        let option = normalize_syntax(&code);
        if option.starts_with('-') {
          options.insert(option);
        }
      }
      _ => {}
    }
  }
  options
}

fn assert_consistent_tables(source: &str, path: &Path) {
  let mut expected_cells = None;
  let mut current_cells = 0;
  let mut counting_cells = false;

  for event in markdown(source) {
    match event {
      Event::Start(Tag::Table(_)) => expected_cells = None,
      Event::Start(Tag::TableHead | Tag::TableRow) => {
        current_cells = 0;
        counting_cells = true;
      }
      Event::Start(Tag::TableCell) if counting_cells => current_cells += 1,
      Event::End(TagEnd::TableHead) => {
        assert!(
          current_cells > 0,
          "Markdown table has no header in {}",
          path.display()
        );
        expected_cells = Some(current_cells);
        counting_cells = false;
      }
      Event::End(TagEnd::TableRow) => {
        assert_eq!(
          Some(current_cells),
          expected_cells,
          "Markdown table has an inconsistent row in {}",
          path.display()
        );
        counting_cells = false;
      }
      Event::End(TagEnd::Table) => {
        assert!(
          expected_cells.is_some(),
          "Markdown table has no header in {}",
          path.display()
        );
      }
      _ => {}
    }
  }
}

fn materialize(documented: &str) -> Vec<String> {
  let mut command = documented.trim().to_owned();
  if let Some(index) = command.find(" > ") {
    command.truncate(index);
  }
  for (source, value) in [
    ("<path|->", "plan.json"),
    ("<shared|private>", "shared"),
    ("<user|repo|private>", "user"),
    ("<public|private>", "public"),
    ("<source...>", "todo-root"),
    ("<YYYY-MM-DD>", "2026-08-25"),
    ("<strategy>", "preserve"),
    ("<compatibility>", "claude"),
    ("<version>", "1.0.0"),
    ("<adapter>", "todo-txt@1"),
    ("<source>", "todo-root"),
    ("<scope>", "shared"),
    ("<path>", "fixture"),
    ("<name>", "adopt-arcantry"),
    ("<task>", "task"),
    ("<line>", "1"),
    ("<date>", "2026-08-25"),
    ("<id>", "root"),
  ] {
    command = command.replace(source, value);
  }
  command = command.replace("[--detailed|--json]", "[--detailed]");
  if (command.contains("skills link") || command.contains("skills unlink"))
    && let (Some(start), Some(end)) = (command.find("(--scope"), command.find(")"))
  {
    command.replace_range(start..=end, "--scope user");
  }
  command = command.replace(['[', ']'], "");
  shell_words::split(&command)
    .unwrap_or_else(|error| panic!("Could not tokenize documented command `{documented}`: {error}"))
}

fn assert_parses(documented: &str, path: &Path) {
  let arguments = materialize(documented);
  Cli::try_parse_from(&arguments).unwrap_or_else(|error| {
    panic!(
      "Documented command does not parse through Clap in {}: `{documented}`\n{error}",
      path.display()
    )
  });
}

#[test]
fn every_documented_arcantry_command_parses_through_the_shared_clap_model() {
  let docs = workspace().join("apps/docs/src/content/docs");
  let mut files = Vec::new();
  documentation_files(&docs, &mut files);
  files.sort();
  assert!(!files.is_empty());

  for path in files {
    let source = fs::read_to_string(&path).unwrap();
    let mut code_block = None::<String>;
    let mut in_table_cell = false;
    for event in markdown(&source) {
      match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => code_block = Some(String::new()),
        Event::Text(text) if code_block.is_some() => code_block.as_mut().unwrap().push_str(&text),
        Event::End(TagEnd::CodeBlock) => {
          for command in code_block_commands(&code_block.take().unwrap_or_default()) {
            assert_parses(&command, &path);
          }
        }
        Event::Start(Tag::TableCell) => in_table_cell = true,
        Event::End(TagEnd::TableCell) => in_table_cell = false,
        Event::Code(code) if in_table_cell && code.trim_start().starts_with("arcantry ") => {
          assert_parses(&code, &path)
        }
        _ => {}
      }
    }
  }
}

#[test]
fn cli_reference_renders_every_contract_syntax_once() {
  let contract = contract();
  let path = workspace().join("apps/docs/src/content/docs/reference/cli.md");
  let source = fs::read_to_string(&path).unwrap();
  let expected = contract
    .commands
    .into_iter()
    .map(|command| command.syntax)
    .collect::<BTreeSet<_>>();
  let expected_global_options = contract
    .global_options
    .into_iter()
    .map(|option| option.syntax)
    .collect::<BTreeSet<_>>();

  assert_eq!(documented_syntaxes(&source), expected);
  assert_eq!(documented_global_options(&source), expected_global_options);
}

#[test]
fn every_markdown_table_has_consistent_rows() {
  let docs = workspace().join("apps/docs/src/content/docs");
  let mut files = Vec::new();
  documentation_files(&docs, &mut files);
  files.sort();

  for path in files {
    let source = fs::read_to_string(&path).unwrap();
    assert_consistent_tables(&source, &path);
  }
}

#[test]
fn every_public_trust_claim_has_executable_documented_evidence() {
  let contract = contract();
  let scenario_ids = contract
    .scenarios
    .into_iter()
    .map(|scenario| scenario.id)
    .collect::<BTreeSet<_>>();
  let documentation = [
    "apps/docs/src/content/docs/reference/cli.md",
    "apps/docs/src/content/docs/reference/repository-workflow.md",
    "apps/docs/src/content/docs/guides/todo-txt.md",
    "apps/docs/src/content/docs/lifecycle/releases.mdx",
  ]
  .into_iter()
  .map(|path| fs::read_to_string(workspace().join(path)).unwrap())
  .collect::<Vec<_>>()
  .join("\n");

  for claim in contract.trust_claims {
    assert!(
      scenario_ids.contains(&claim.evidence),
      "Trust claim {} has no executable evidence",
      claim.id
    );
    assert!(
      documentation.contains(&format!("cli-evidence: {}", claim.id)),
      "Trust claim {} is not documented",
      claim.id
    );
  }
}

#[test]
fn todo_writing_skills_preserve_explicit_source_conventions() {
  for (name, convention) in [
    (
      "capture-project-work",
      "explicitly required project or source format",
    ),
    (
      "promote-todo-to-openspec",
      "explicit compatible source convention",
    ),
  ] {
    let source =
      fs::read_to_string(workspace().join("skills").join(name).join("SKILL.md")).unwrap();
    assert!(source.contains(convention), "{name}");
    assert!(source.contains("official"), "{name}");
    assert!(source.contains("optional"), "{name}");
  }
}
