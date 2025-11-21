use console::style;
use dialoguer::theme::Theme as DialogTheme;
use std::fmt;

#[derive(clap::ValueEnum, Clone, Copy, Default)]
pub enum Theme {
  #[default]
  Nix,
  Lix,
}

pub struct BaseTheme {
  pub base: console::Color,
  pub alt: console::Color,
}

impl DialogTheme for BaseTheme {
  fn format_input_prompt(
    &self,
    f: &mut dyn fmt::Write,
    prompt: &str,
    _default: Option<&str>,
  ) -> fmt::Result {
    write!(
      f,
      "{}{} {} ",
      style("󱄅 ").fg(self.base),
      style(prompt).dim(),
      style("?").fg(self.base)
    )
  }
  fn format_input_prompt_selection(
    &self,
    f: &mut dyn fmt::Write,
    _prompt: &str,
    sel: &str,
  ) -> fmt::Result {
    write!(
      f,
      "{}{}",
      style("󱄅 ").fg(self.base),
      style(sel).fg(self.base).dim()
    )
  }
}

impl BaseTheme {
  pub fn from_theme(theme: Theme) -> Self {
    match theme {
      Theme::Nix => BaseTheme {
        base: console::Color::Blue,
        alt: console::Color::Magenta,
      },
      Theme::Lix => BaseTheme {
        base: console::Color::Magenta,
        alt: console::Color::Blue,
      },
    }
  }
}
