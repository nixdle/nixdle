use console::{Term, style};
use dialoguer::Input;
use std::sync::{LazyLock, Mutex};

pub mod args;
mod theme;

use theme::{BaseTheme, Theme};

static TERM: LazyLock<Term> = LazyLock::new(Term::stdout);
static TERM_ERR: LazyLock<Term> = LazyLock::new(Term::stderr);
static STATUS: Mutex<usize> = Mutex::new(0);
lazy_static::lazy_static! {
  static ref THEME:  Mutex<BaseTheme> = Mutex::new(BaseTheme::from_theme(Theme::default()));
}

macro_rules! write_line {
  ($($arg:tt)*) => {
    TERM.write_line(&format!($($arg)*)).ok();
  };
}
macro_rules! ewrite_line {
  ($($arg:tt)*) => {
    TERM_ERR.write_line(&format!($($arg)*)).ok();
  };
}

pub fn dialog(prompt: String) -> String {
  TERM.show_cursor().ok();

  let input = Input::with_theme(&*THEME.lock().unwrap())
    .with_prompt(prompt)
    .interact_text()
    .expect("failed to read user input :c");

  TERM.hide_cursor().ok();
  input
}

pub fn print_welcome() {
  let name = env!("CARGO_BIN_NAME");
  let theme = THEME.lock().unwrap();

  #[cfg(not(debug_assertions))]
  TERM.clear_screen().ok();
  TERM.set_title(name);
  TERM.hide_cursor().ok();

  write_line!(
    "{} v{}",
    style(name).fg(theme.base).bold(),
    env!("CARGO_PKG_VERSION")
  );

  write_line!("\n{}", style("welcome to nixdle!").fg(theme.alt).bold());
  write_line!("try to guess today's nix function\n");
}

pub fn print_rules(text: String) {
  clear_status();
  write_line!("{}", style(text).dim());
  write_line!("{}\n", style("good luck!!").bold());
}

pub fn print_attempt(clues: Vec<String>, args: String, input: bool, output: bool) {
  let theme = THEME.lock().unwrap();

  clear_status();
  if !clues.is_empty() {
    write_line!(
      "  {}        {}",
      style("path:").fg(theme.base),
      style(clues.join(".")).dim()
    );
  }
  write_line!(
    "  {}   {}",
    style("arguments:").fg(theme.base),
    style(args).dim()
  );
  write_line!(
    "  {}  {}",
    style("input type:").fg(theme.base),
    if input {
      style("✔").green().bold()
    } else {
      style("✘").red().bold()
    }
  );
  write_line!(
    "  {} {}",
    style("output type:").fg(theme.base),
    if output {
      style("✔").green().bold()
    } else {
      style("✘").red().bold()
    }
  );

  write_line!("");
}

pub fn print_solved(func: &str, desc: &str, attempts: usize, seconds: u64, date: &str) {
  let theme = THEME.lock().unwrap();
  clear_status();

  write_line!(
    "{0} {6}        {1}\n{0} {7}        {2} seconds\n{0} {8}    {3}\n{0} {9}    {4}\n{0} {10} {5}",
    " ",
    style(date),
    style(seconds),
    style(attempts),
    style(func).bold(),
    style(desc).dim(),
    style("date:").fg(theme.base),
    style("time:").fg(theme.base),
    style("attempts:").fg(theme.base),
    style("function:").fg(theme.base),
    style("description:").fg(theme.base),
  );

  write_line!(
    "\n{}{}{}",
    style("whoa").green().bold(),
    style(", you solved today's nixdle! ").green(),
    style("congrats!").green().bold()
  );
  write_line!("here's your reward: 🍪");
}

pub fn print_already_solved() {
  clear_status();
  write_line!(
    "{}{}",
    style("you already solved today's nixdle, ").green(),
    style("dumbass").green().bold().underlined()
  );
  write_line!("go like {} or something??", style("touch grass").bold());
  write_line!(
    "you {} get another reward!!! (come back tomorrow)",
    style("won't").red().bold().underlined()
  );
}

pub fn print_status(text: String) {
  clear_status();
  *STATUS.lock().unwrap() += text.lines().count();
  ewrite_line!("{}: {}...", style("status").green().bold(), text);
}
pub fn print_warning(text: String) {
  clear_status();
  ewrite_line!("{}: {}", style("warning").yellow().bold(), text);
}
pub fn print_error(text: String) {
  clear_status();
  ewrite_line!("{}: {}", style("error").red().bold(), text);
}

fn clear_status() {
  TERM.clear_last_lines(*STATUS.lock().unwrap()).ok();
  *STATUS.lock().unwrap() = 0;
}

pub fn reset() {
  TERM.show_cursor().ok();
}
