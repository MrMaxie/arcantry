use arcantry_cli::cli::Cli;
use clap::Parser;
use pulldown_cmark::{CodeBlockKind, Event, Parser as MarkdownParser, Tag, TagEnd};
use std::fs;
use std::path::{Path, PathBuf};

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
    for event in MarkdownParser::new(&source) {
      match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => code_block = Some(String::new()),
        Event::Text(text) if code_block.is_some() => code_block.as_mut().unwrap().push_str(&text),
        Event::End(TagEnd::CodeBlock) => {
          for line in code_block.take().unwrap_or_default().lines() {
            if line.trim_start().starts_with("arcantry ") {
              assert_parses(line.trim(), &path);
            }
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
