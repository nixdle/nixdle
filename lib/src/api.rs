#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::function::Type;

/// data from client to server on attempt
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AttemptData {
  pub input: String,
  pub attempts: usize,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StartMessage {
  pub date: String,
  pub attempt_url: String,
  pub possible_clues: u8,
  pub rules: String,
  pub version: String,
  pub nix_commit: String,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AttemptMessage {
  pub success: bool,
  pub func: Option<String>,
  pub description: Option<String>,
  pub clues: Vec<String>,
  pub args: Matches,
  pub input: bool,
  pub output: bool,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Matches {
  TooLow,
  TooHigh,
  JustRight,
}

impl Matches {
  pub fn check(guess: u8, actual: u8) -> Self {
    if guess < actual {
      Matches::TooLow
    } else if guess > actual {
      Matches::TooHigh
    } else {
      Matches::JustRight
    }
  }

  pub fn check_types(guess: (&Type, &Type), actual: (&Type, &Type)) -> (bool, bool) {
    (guess.0 == actual.0, guess.1 == actual.1)
  }
}

impl std::fmt::Display for Matches {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Matches::TooLow => write!(f, "more"),
      Matches::TooHigh => write!(f, "less"),
      Matches::JustRight => write!(f, "just right :D"),
    }
  }
}
