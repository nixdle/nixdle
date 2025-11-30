use super::theme::Theme;

#[derive(clap::Parser)]
#[command(version, about)]
pub struct Cli {
  /// api url to use
  #[arg(long, value_name = "url", default_value = crate::DEFAULT_API_URL)]
  pub api: String,
  /// cli theme to use
  #[arg(short, long, value_name = "theme", default_value_t, value_enum)]
  pub theme: Theme,
  /// don't show rules text
  #[arg(long)]
  pub hide_rules: bool,
}
