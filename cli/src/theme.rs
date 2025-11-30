use console::{Color, style};
use dialoguer::theme::Theme as DialogTheme;
use std::fmt;

#[derive(clap::ValueEnum, Clone, Copy, Default)]
pub enum Theme {
  #[default]
  Nix,
  Lix,
}

pub struct BaseTheme {
  pub base: Color,
  pub alt: Color,
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

impl From<Theme> for BaseTheme {
  fn from(theme: Theme) -> Self {
    match theme {
      Theme::Nix => BaseTheme {
        base: Color::Blue,
        alt: Color::Magenta,
      },
      Theme::Lix => BaseTheme {
        base: Color::Magenta,
        alt: Color::Blue,
      },
    }
  }
}
