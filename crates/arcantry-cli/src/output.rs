use std::fmt::Write;

pub(crate) fn terminal_text(value: &str) -> String {
  let mut sanitized = String::with_capacity(value.len());
  for character in value.chars() {
    if character == '\n' || character == '\t' || !character.is_control() {
      sanitized.push(character);
    } else {
      write!(sanitized, "\\u{{{:x}}}", character as u32).expect("writing to a String cannot fail");
    }
  }
  sanitized
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn preserves_line_structure_and_escapes_terminal_controls() {
    assert_eq!(
      terminal_text("task\n\t\u{1b}]8;;https://example.test\u{7}\r\u{7f}\u{80}"),
      "task\n\t\\u{1b}]8;;https://example.test\\u{7}\\u{d}\\u{7f}\\u{80}"
    );
  }
}
