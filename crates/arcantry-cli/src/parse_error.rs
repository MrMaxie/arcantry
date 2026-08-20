use clap::{Arg, Command};

pub fn render(arguments: &[String]) -> Option<String> {
  if arguments.len() == 1 && matches!(arguments[0].as_str(), "--version" | "-V") {
    return None;
  }
  if arguments
    .iter()
    .any(|argument| matches!(argument.as_str(), "--help" | "-h"))
  {
    return None;
  }

  let root = crate::cli::command();
  for argument in root
    .get_arguments()
    .filter(|argument| argument.is_global_set())
  {
    if missing_value(arguments, argument) {
      return error(
        format!("option '{}' argument missing", option_term(argument)),
        crate::help::root(),
      );
    }
  }

  let normalized = without_global_options(arguments, &root);
  let first = normalized.first()?;
  if first.starts_with('-') {
    return error(format!("unknown option '{first}'"), crate::help::root());
  }
  let Some(group) = root.find_subcommand(first) else {
    return error(format!("unknown command '{first}'"), crate::help::root());
  };
  if normalized.len() == 1 {
    return Some(command_help(&[first]));
  }

  let command_name = &normalized[1];
  if command_name.starts_with('-') {
    return error(
      format!("unknown option '{command_name}'"),
      command_help(&[first]),
    );
  }
  let Some(command) = group.find_subcommand(command_name) else {
    return error(
      format!("unknown command '{command_name}'"),
      command_help(&[first]),
    );
  };
  let help = command_help(&[first, command_name]);

  if let Some(option) = normalized.iter().skip(2).find(|value| {
    value.as_str() != "-" && value.starts_with('-') && find_option(command, value).is_none()
  }) {
    return error(format!("unknown option '{option}'"), help);
  }
  for argument in command
    .get_arguments()
    .filter(|argument| argument.get_index().is_none() && argument.get_action().takes_values())
  {
    if missing_value(&normalized, argument) {
      return error(
        format!("option '{}' argument missing", option_term(argument)),
        help,
      );
    }
  }
  if let Some(argument) = command
    .get_arguments()
    .filter(|argument| {
      (argument.get_long().is_some() || argument.get_short().is_some())
        && argument.is_required_set()
    })
    .find(|argument| !has_option(&normalized, argument))
  {
    return error(
      format!("required option '{}' not specified", option_term(argument)),
      help,
    );
  }

  let expected = command.get_positionals().count();
  let actual = positional_count(&normalized[2..], command);
  if actual > expected {
    return error(
      format!(
        "too many arguments for '{command_name}'. Expected {expected} arguments but got {}.",
        actual
      ),
      help,
    );
  }
  if let Some(argument) = command
    .get_positionals()
    .filter(|argument| argument.is_required_set())
    .nth(actual)
  {
    return error(
      format!("missing required argument '{}'", argument.get_id()),
      help,
    );
  }
  None
}

fn error(message: String, help: String) -> Option<String> {
  Some(format!("error: {message}\n\n{help}"))
}

fn command_help(path: &[&String]) -> String {
  let mut arguments = path
    .iter()
    .map(|value| (*value).clone())
    .collect::<Vec<_>>();
  arguments.push("--help".to_owned());
  crate::help::render(&arguments).unwrap_or_default()
}

fn without_global_options(arguments: &[String], root: &Command) -> Vec<String> {
  let mut normalized = Vec::new();
  let mut index = 0;
  while index < arguments.len() {
    if root
      .get_arguments()
      .filter(|argument| argument.is_global_set())
      .any(|argument| option_matches(&arguments[index], argument))
    {
      if !arguments[index].contains('=') {
        index += 1;
      }
    } else {
      normalized.push(arguments[index].clone());
    }
    index += 1;
  }
  normalized
}

fn positional_count(arguments: &[String], command: &Command) -> usize {
  let mut positionals = 0;
  let mut index = 0;
  while index < arguments.len() {
    if let Some(option) = find_option(command, &arguments[index]) {
      if option.get_action().takes_values() && !arguments[index].contains('=') {
        let maximum = option
          .get_num_args()
          .map(|range| range.max_values())
          .unwrap_or(1);
        let mut consumed = 0;
        while consumed < maximum
          && arguments
            .get(index + 1)
            .is_some_and(|value| !value.starts_with('-'))
        {
          index += 1;
          consumed += 1;
        }
      }
    } else if !arguments[index].starts_with('-') {
      positionals += 1;
    }
    index += 1;
  }
  positionals
}

fn find_option<'a>(command: &'a Command, value: &str) -> Option<&'a Arg> {
  command
    .get_arguments()
    .find(|argument| option_matches(value, argument))
}

fn option_matches(value: &str, argument: &Arg) -> bool {
  let flag = value.split('=').next().unwrap_or(value);
  argument
    .get_long()
    .is_some_and(|long| flag == format!("--{long}"))
    || argument
      .get_short()
      .is_some_and(|short| flag == format!("-{short}"))
}

fn has_option(arguments: &[String], argument: &Arg) -> bool {
  arguments
    .iter()
    .any(|value| option_matches(value, argument))
}

fn missing_value(arguments: &[String], argument: &Arg) -> bool {
  arguments
    .iter()
    .position(|value| option_matches(value, argument) && !value.contains('='))
    .is_some_and(|index| {
      arguments
        .get(index + 1)
        .is_none_or(|value| value.starts_with("--"))
    })
}

fn option_term(argument: &Arg) -> String {
  let long = argument
    .get_long()
    .map(|value| format!("--{value}"))
    .or_else(|| argument.get_short().map(|value| format!("-{value}")))
    .unwrap_or_else(|| argument.get_id().to_string());
  if argument.get_action().takes_values() {
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
    format!("{long} <{name}{suffix}>")
  } else {
    long
  }
}
