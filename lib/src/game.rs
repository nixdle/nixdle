use crate::function::Type;

#[cfg(feature = "sqlx")]
use sqlx::types::{Json, chrono};

/// represents a game instance
#[derive(Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Game {
  /// this is alwals true
  id: bool,
  /// full path to the function
  func: String,
  /// short description of what the function does
  description: String,
  /// number of arguments the function takes
  args: u8,
  /// input type
  #[cfg(feature = "sqlx")]
  input: Json<Type>,
  #[cfg(not(feature = "sqlx"))]
  input: Type,
  /// output type
  #[cfg(feature = "sqlx")]
  output: Json<Type>,
  #[cfg(not(feature = "sqlx"))]
  output: Type,
  /// nix commit hash
  nix_commit: String,
  /// when the game was initialized
  created_at: chrono::NaiveDateTime,
}

impl Game {
  /// creates a new game instance
  pub fn new(func: String, description: String, args: u8, input: Type, output: Type) -> Self {
    Self {
      id: true,
      func,
      description,
      args,
      #[cfg(feature = "sqlx")]
      input: Json(input),
      #[cfg(not(feature = "sqlx"))]
      input,
      #[cfg(feature = "sqlx")]
      output: Json(output),
      #[cfg(not(feature = "sqlx"))]
      output,
      nix_commit: String::new(), // TODO: implement this
      created_at: chrono::Utc::now().naive_utc(),
    }
  }

  pub fn get_func(&self) -> &str {
    &self.func
  }
  pub fn get_func_name(&self) -> Option<&str> {
    self.func.split('.').next_back()
  }
  pub fn get_clues(&self) -> Vec<String> {
    let clues = self.func.split('.').collect::<Vec<&str>>();
    clues[0..clues.len().saturating_sub(1)]
      .iter()
      .map(|s| s.to_string())
      .collect()
  }
  pub fn get_description(&self) -> &str {
    &self.description
  }
  pub fn get_args_count(&self) -> u8 {
    self.args
  }
  pub fn get_types(&self) -> (&Type, &Type) {
    (&self.input, &self.output)
  }
  pub fn get_nix_commit(&self) -> &str {
    &self.nix_commit
  }
  pub fn get_date(&self) -> String {
    self.created_at.format("%Y-%m-%d").to_string()
  }
}
