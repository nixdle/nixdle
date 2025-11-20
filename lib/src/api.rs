#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::function::Type;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StartMessage {
  pub date: String,
  pub attempt_url: String,
  pub clue_attempts: u8,
  pub possible_clues: u8,
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
  pub fn check(actual: u8, guess: u8) -> Self {
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
