use color_eyre::{Result, eyre::Context};
use console::{Term, style};
use dialoguer::Input;

use crate::theme::BaseTheme;

macro_rules! write_line {
  ($term: expr, $($arg:tt)*) => {
    $term.write_line(&format!($($arg)*)).wrap_err("IO error")
  };
}

pub struct Ui {
  stderr: Term,
  stdout: Term,
  theme: BaseTheme,
  status_line_count: usize,
}

impl Ui {
  pub fn new(theme: BaseTheme) -> Self {
    Self {
      stderr: Term::stderr(),
      stdout: Term::stdout(),
      theme,
      status_line_count: 0,
    }
  }

  pub fn dialog(&mut self, prompt: &str) -> Result<String> {
    self.stdout.show_cursor()?;

    let input = Input::with_theme(&self.theme)
      .with_prompt(prompt)
      .interact_text()
      .wrap_err("failed to read user input :c")?;

    Ok(input)
  }

  pub fn print_welcome(&mut self) -> Result<()> {
    let name = env!("CARGO_BIN_NAME");
    let term = &mut self.stdout;
    #[cfg(not(debug_assertions))]
    term.clear_screen().ok();
    term.set_title(name);
    term.hide_cursor().ok();

    write_line!(
      self.stdout,
      "{} v{}\n{}\ntry to guess today's nix function\n",
      style(name).fg(self.theme.base).bold(),
      env!("CARGO_PKG_VERSION"),
      style("welcome to nixdle!").fg(self.theme.alt).bold()
    )
  }

  pub fn print_rules(&mut self, text: &str) -> Result<()> {
    write_line!(
      self.stdout,
      "{} {}\n",
      style(text).dim(),
      style("good luck!!").bold()
    )
  }

  pub fn print_attempt(
    &mut self,
    clues: Vec<String>,
    args: String,
    input: bool,
    output: bool,
  ) -> Result<()> {
    self.clear_status()?;
    if !clues.is_empty() {
      write_line!(
        self.stdout,
        "  {}        {}",
        style("path:").fg(self.theme.base),
        style(clues.join(".")).dim()
      )?;
    }

    write_line!(
      self.stdout,
      "  {}   {}\n  {}   {}\n  {}   {}",
      style("arguments:").fg(self.theme.base),
      style(args).dim(),
      style("input type:").fg(self.theme.base),
      if input {
        style("✔").green().bold()
      } else {
        style("✘").red().bold()
      },
      style("output type:").fg(self.theme.base),
      if output {
        style("✔").green().bold()
      } else {
        style("✘").red().bold()
      }
    )
  }

  pub fn print_solved(
    &mut self,
    func: &str,
    desc: &str,
    attempts: usize,
    seconds: u64,
    date: &str,
  ) -> Result<()> {
    self.clear_status()?;

    write_line!(
      self.stdout,
      " {}        {}\n  {}        {} seconds\n  {}    {}\n  {}    {}\n  {} {}\n\n{}{}{}\n{}",
      style("date:").fg(self.theme.base),
      style(date),
      style("time:").fg(self.theme.base),
      style(seconds),
      style("attempts:").fg(self.theme.base),
      style(attempts),
      style("function:").fg(self.theme.base),
      style(func).bold(),
      style("description:").fg(self.theme.base),
      style(desc).dim(),
      style("whoa").green().bold(),
      style(", you solved today's nixdle! ").green(),
      style("congrats!").green().bold(),
      "here's your reward: 🍪"
    )
  }

  pub fn print_already_solved(&mut self) -> Result<()> {
    self.clear_status()?;
    write_line!(
      self.stdout,
      "{}{}\ngo like {} or something??\nyou {} get another reward!!! (come back tomorrow)",
      style("you already solved today's nixdle, ").green(),
      style("dumbass").green().bold().underlined(),
      style("touch grass").bold(),
      style("won't").red().bold().underlined()
    )
  }

  pub fn print_status(&mut self, text: &str) -> Result<()> {
    self.clear_status()?;
    self.status_line_count = text.lines().count();
    write_line!(
      self.stdout,
      "{}: {}...",
      style("status").green().bold(),
      text
    )
  }
  pub fn print_warning(&mut self, text: &str) -> Result<()> {
    self.clear_status()?;
    write_line!(
      self.stderr,
      "{}: {}",
      style("warning").yellow().bold(),
      text
    )
  }

  pub fn print_error(&mut self, text: &str) -> Result<()> {
    self.clear_status()?;
    write_line!(self.stderr, "{}: {}", style("error").red().bold(), text)
  }

  fn clear_status(&mut self) -> Result<()> {
    self.stdout.clear_last_lines(self.status_line_count)?;
    self.status_line_count = 0;
    Ok(())
  }
}
