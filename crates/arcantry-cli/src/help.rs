use clap::{Arg, Command};

type Item = (String, String);

pub fn render(arguments: &[String]) -> Option<String> {
  if !arguments
    .iter()
    .any(|value| matches!(value.as_str(), "-h" | "--help"))
  {
    return None;
  }
  let root = crate::cli::command();
  let (command, path) = resolve_command(&root, arguments);
  Some(format_help(command, &path))
}

pub fn root() -> String {
  let command = crate::cli::command();
  format_help(&command, &[])
}

fn resolve_command<'a>(root: &'a Command, arguments: &[String]) -> (&'a Command, Vec<String>) {
  let mut command = root;
  let mut path = Vec::new();
  let mut skip_value = false;
  for argument in arguments {
    if skip_value {
      skip_value = false;
      continue;
    }
    if matches!(argument.as_str(), "-h" | "--help") {
      break;
    }
    if let Some(option) = argument.strip_prefix("--") {
      let option = option.split('=').next().unwrap_or(option);
      skip_value = !argument.contains('=')
        && command
          .get_arguments()
          .find(|candidate| candidate.get_long() == Some(option))
          .is_some_and(takes_value);
      continue;
    }
    if argument.starts_with('-') {
      continue;
    }
    if let Some(child) = command.find_subcommand(argument) {
      path.push(argument.clone());
      command = child;
    }
  }
  (command, path)
}

fn format_help(command: &Command, path: &[String]) -> String {
  let description = command
    .get_about()
    .map(ToString::to_string)
    .unwrap_or_default();
  let mut output = format!(
    "Usage: {}\n\n{}\n",
    usage(command, path),
    wrap_paragraph(&description, 0)
  );
  let options = options(command, path.is_empty());
  let commands = commands(command);
  let width = options
    .iter()
    .chain(commands.iter())
    .map(|(term, _)| term.len())
    .max()
    .unwrap_or(0);
  append_section(&mut output, "Options", &options, width);
  if !commands.is_empty() {
    append_section(&mut output, "Commands", &commands, width);
  }
  output
}

fn usage(command: &Command, path: &[String]) -> String {
  let mut usage = vec!["arcantry".to_owned()];
  usage.extend(path.iter().cloned());
  usage.push("[options]".to_owned());
  if command.has_subcommands() {
    usage.push("[command]".to_owned());
  } else {
    usage.extend(command.get_positionals().map(positional_term));
  }
  usage.join(" ")
}

fn options(command: &Command, root: bool) -> Vec<Item> {
  let mut items = Vec::new();
  if root {
    items.push((
      "-V, --version".to_owned(),
      "output the version number".to_owned(),
    ));
  }
  items.extend(
    command
      .get_arguments()
      .filter(|argument| argument.get_index().is_none())
      .filter(|argument| root || !matches!(argument.get_id().as_str(), "cwd" | "config"))
      .filter_map(option_item),
  );
  items.push((
    "-h, --help".to_owned(),
    "display help for command".to_owned(),
  ));
  items
}

fn option_item(argument: &Arg) -> Option<Item> {
  if argument.is_hide_set() {
    return None;
  }
  let mut flags = Vec::new();
  if let Some(short) = argument.get_short() {
    flags.push(format!("-{short}"));
  }
  if let Some(long) = argument.get_long() {
    flags.push(format!("--{long}"));
  }
  if flags.is_empty() {
    return None;
  }
  let mut term = flags.join(", ");
  if takes_value(argument) {
    term.push(' ');
    term.push_str(&value_term(argument));
  }
  let mut description = argument
    .get_help()
    .map(ToString::to_string)
    .unwrap_or_default();
  if let Some(default) = argument.get_default_values().first() {
    let default = default.to_string_lossy();
    description.push_str(&format!(" (default: \"{default}\")"));
  }
  Some((term, description))
}

fn commands(command: &Command) -> Vec<Item> {
  if !command.has_subcommands() {
    return Vec::new();
  }
  let mut items = command
    .get_subcommands()
    .filter(|child| !child.is_hide_set())
    .map(|child| {
      let mut term = child.get_name().to_owned();
      if !child.has_subcommands() {
        let has_arguments = child
          .get_arguments()
          .any(|argument| !matches!(argument.get_id().as_str(), "cwd" | "config"));
        if has_arguments {
          term.push_str(" [options]");
        }
        for positional in child.get_positionals() {
          term.push(' ');
          term.push_str(&positional_term(positional));
        }
      }
      let description = child
        .get_about()
        .map(ToString::to_string)
        .unwrap_or_default();
      (term, description)
    })
    .collect::<Vec<_>>();
  items.push((
    "help [command]".to_owned(),
    "display help for command".to_owned(),
  ));
  items
}

fn takes_value(argument: &Arg) -> bool {
  argument.get_action().takes_values()
}

fn value_term(argument: &Arg) -> String {
  let name = argument
    .get_value_names()
    .and_then(|names| names.first())
    .map(ToString::to_string)
    .unwrap_or_else(|| argument.get_id().to_string());
  let suffix = if argument
    .get_num_args()
    .is_some_and(|range| range.max_values() > 1)
  {
    "..."
  } else {
    ""
  };
  format!("<{name}{suffix}>")
}

fn positional_term(argument: &Arg) -> String {
  let value = value_term(argument);
  if argument.is_required_set() {
    value
  } else {
    format!("[{value}]")
  }
}

fn append_section(output: &mut String, heading: &str, items: &[Item], width: usize) {
  output.push_str(&format!("\n{heading}:\n"));
  let description_column = width + 4;
  for (term, description) in items {
    let prefix = format!("  {term}{}", " ".repeat(width - term.len() + 2));
    let lines = wrap(description, 80 - description_column);
    output.push_str(&prefix);
    output.push_str(&lines[0]);
    output.push('\n');
    for line in &lines[1..] {
      output.push_str(&" ".repeat(description_column));
      output.push_str(line);
      output.push('\n');
    }
  }
}

fn wrap_paragraph(value: &str, indent: usize) -> String {
  wrap(value, 80 - indent).join(&format!("\n{}", " ".repeat(indent)))
}

fn wrap(value: &str, width: usize) -> Vec<String> {
  let mut lines = Vec::new();
  let mut current = String::new();
  for word in value.split_whitespace() {
    if !current.is_empty() && current.len() + 1 + word.len() > width {
      lines.push(current);
      current = word.to_owned();
    } else {
      if !current.is_empty() {
        current.push(' ');
      }
      current.push_str(word);
    }
  }
  lines.push(current);
  lines
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn renders_help_from_clap_command_metadata() {
    assert!(root().starts_with("Usage: arcantry [options] [command]\n\n"));
    assert!(root().contains("help [command]"));
    assert!(root().contains("display help for command"));
    let output = render(&["repo".to_owned(), "plan".to_owned(), "--help".to_owned()]).unwrap();
    assert!(output.contains("--transition <transition>"));
  }
}
