//! nixdle - wordle but it's nix functions

use rand::prelude::IndexedRandom;

pub mod api;
mod function;
mod game;

use api::{AttemptMessage, Matches, StartMessage};
use function::Function;
use game::Game;

pub const NEXT_CLUE_ATTEMPTS: usize = 5;

/// contains everything needed to run
#[derive(Clone)]
pub struct State {
  pub game: Option<Game>,
  pub functions: Vec<Function>,
  pub builtin_types: Vec<(String, String)>,
}

impl State {
  /// creates a new state
  pub fn new(functions: Vec<Function>, builtin_types: Vec<(String, String)>) -> Self {
    Self {
      game: None,
      functions,
      builtin_types,
    }
  }

  /// initializes a new random game from available functions
  pub fn init_random_game(&mut self) {
    let rng = &mut rand::rng();

    loop {
      let func = self.functions.choose(rng).expect("where functions??");

      let desc = match func.get_description() {
        Some(d) => d.trim(),
        None => continue,
      };
      let desc = desc
        .split_once("\n\n")
        .map(|(a, _)| a)
        .unwrap_or(desc)
        .replace('\n', " ");

      let args = func.get_args_count() as u8;

      let types = match func.get_types(&self.builtin_types) {
        Some(t) => t,
        None => continue,
      };

      self.game = Some(Game::new(
        func.meta.path.join("."),
        desc.trim().to_string(),
        args,
        types.0,
        types.1,
      ));

      break;
    }
  }

  /// starts a new game attempt
  pub fn start_game(&self, attempt_url: String) -> StartMessage {
    let game = self.game.as_ref().expect("where game??");
    StartMessage {
      date: game.get_date(),
      attempt_url,
      possible_clues: game.get_clues().len() as u8,
      rules: format!(
        "you can guess by full path (e.g. 'lib.replaceStrings')\nor by name (e.g. 'substring' for 'builtins.substring')\nafter each guess, you'll see how close you were to the actual function\nevery {} attempts, you'll get a new path clue",
        NEXT_CLUE_ATTEMPTS
      ),
      version: env!("CARGO_PKG_VERSION").to_string(),
      nix_commit: game.get_nix_commit().to_string(),
    }
  }

  /// attempts to guess the function
  /// returns None if the guess is invalid (i.e. not a known function)
  pub fn attempt_game(&self, input: &str, attempts: usize) -> Option<AttemptMessage> {
    let input = input.trim();
    let game = self.game.as_ref().expect("where game??");
    let func = game.get_func();
    let all_clues = game.get_clues();

    let guess_func = self.find_function(input)?;

    if input == func
      || guess_func.meta.path.join(".") == func
      || guess_func
        .meta
        .aliases
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .any(|a| a.join(".") == func)
    {
      return Some(AttemptMessage {
        success: true,
        func: Some(game.get_func().to_string()),
        description: Some(game.get_description().to_string()),
        clues: all_clues,
        args: Matches::JustRight,
        input: true,
        output: true,
      });
    }

    let guess_types = guess_func.get_types(&self.builtin_types)?;

    let types_match = Matches::check_types((&guess_types.0, &guess_types.1), game.get_types());
    let args_match = Matches::check(guess_func.get_args_count() as u8, game.get_args_count());

    let clues_many = attempts / NEXT_CLUE_ATTEMPTS;
    let clues = all_clues.iter().take(clues_many).cloned().collect();

    Some(AttemptMessage {
      success: false,
      func: None,
      description: None,
      clues,
      args: args_match,
      input: types_match.0,
      output: types_match.1,
    })
  }

  /// finds a function by its full path (e.g. "lib.mapAttrs")
  /// or name (e.g. "substring" for "builtins.substring" or "flip" for "lib.flip")
  /// returns None if not found
  pub fn find_function(&self, path: &str) -> Option<&Function> {
    if path.contains('.') {
      self
        .functions
        .iter()
        .find(|f| f.meta.path.join(".") == path)
    } else {
      self
        .functions
        .iter()
        .find(|f| f.meta.path.len() == 2 && f.meta.path.last() == Some(&path.to_string()))
    }
  }
}

/// parse functions from JSON data and filter out those without description or types
#[cfg(feature = "serde")]
pub fn parse_functions_filtered(
  builtin_types: &[(String, String)],
  data: &str,
) -> Result<Vec<Function>, serde_json::Error> {
  let functions: Vec<Function> = serde_json::from_str(data)?;

  let filtered: Vec<Function> = functions
    .into_iter()
    .filter(|f| f.get_description().is_some() && f.get_types(builtin_types).is_some())
    .collect();

  Ok(filtered)
}

/// parse builtin types from JSON data
#[cfg(feature = "serde")]
pub fn parse_builtin_types(data: &str) -> Result<Vec<(String, String)>, serde_json::Error> {
  let map: serde_json::Value = serde_json::from_str(data)?;
  let mut types = Vec::new();

  if let serde_json::Value::Object(obj) = map {
    for (key, value) in obj {
      if let serde_json::Value::Object(inner) = value
        && let Some(serde_json::Value::String(fn_type)) = inner.get("fn_type")
      {
        types.push((key, fn_type.clone()));
      }
    }
  }

  Ok(types)
}
