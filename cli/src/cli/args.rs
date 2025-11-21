use super::{
  THEME,
  theme::{BaseTheme, Theme},
};
use clap::Parser;

#[derive(clap::Parser)]
#[command(version, about)]
pub struct Cli {
  /// api url to use
  #[arg(long, value_name = "url", default_value = crate::DEFAULT_API_URL)]
  pub api: String,
  /// cli theme to use
  #[arg(short, long, value_name = "theme", default_value_t, value_enum)]
  theme: Theme,
  /// don't show rules text
  #[arg(long)]
  pub hide_rules: bool,
}

pub fn parse() -> Cli {
  let args = Cli::parse();

  match args.theme {
    Theme::Nix => *THEME.lock().unwrap() = BaseTheme::from_theme(Theme::Nix),
    Theme::Lix => *THEME.lock().unwrap() = BaseTheme::from_theme(Theme::Lix),
  }

  args
}
