//! nixdle - wordle but it's nix functions

use rand::prelude::IndexedRandom;
use serde::Deserialize;
use sqlx::{prelude::FromRow, types::chrono};

const NEXT_CLUE_ATTEMPTS: u8 = 5;

#[derive(Debug, FromRow)]
pub struct Game {
  id: bool,
  func: String,
  description: String,
  args: u8,
  input: String,
  output: String,
  nix_commit: String,
  created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Function {
  meta: FunctionMeta,
  content: Option<FunctionContent>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionMeta {
  path: Vec<String>,
  aliases: Option<Vec<Vec<String>>>,
  signature: Option<String>,
  is_primop: Option<bool>,
  primop_meta: Option<FunctionPrimopMeta>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionPrimopMeta {
  args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct FunctionContent {
  content: Option<String>,
}

pub fn parse_functions(data: &str) -> Result<Vec<Function>, serde_json::Error> {
  let functions: Vec<Function> = serde_json::from_str(data)?;

  Ok(
    functions
      .into_iter()
      .filter(|f| {
        f.meta.is_primop.unwrap_or(false)
          && f
            .meta
            .primop_meta
            .as_ref()
            .and_then(|p| p.args.as_ref())
            .is_some()
          && f
            .content
            .as_ref()
            .and_then(|c| c.content.as_ref())
            .is_some()
      })
      .collect(),
  )
}

pub fn new(functions: Vec<Function>) -> Game {
  let thread_rng = &mut rand::rng();
  loop {
    let func = functions.choose(thread_rng).unwrap();
    let r = "".to_string();
    let description = func
      .content
      .as_ref()
      .and_then(|c| c.content.as_ref())
      .unwrap_or(&r)
      .trim();
    let description = description
      .split_once("\n\n")
      .map(|(first, _)| first.to_string())
      .unwrap_or(description.to_string())
      .replace('\n', " ");
    let args = func
      .meta
      .primop_meta
      .as_ref()
      .and_then(|p| p.args.as_ref())
      .map(|a| a.len() as u8)
      .unwrap_or(0);

    if let Some(signature) = func.meta.signature.as_ref() {
      let s = signature.trim().to_lowercase();
      let sig = match s.split_once(" :: ").map(|(_, sig)| sig) {
        Some(sig) => sig,
        None => {
          continue;
        }
      };

      let parts: Vec<&str> = sig.split(" -> ").map(|p| p.trim()).collect();

      let first = match parts.first() {
        Some(f) if f.starts_with('{') => "attrset",
        Some(f) => f,
        None => {
          continue;
        }
      };
      let last = match parts.last() {
        Some(l) if l.ends_with('}') => "attrset",
        Some(l) => l,
        None => {
          continue;
        }
      };

      if first.starts_with('(') || first.contains(' ') || last.ends_with(')') || last.contains(' ')
      {
        continue;
      }

      return Game {
        id: true,
        func: func.meta.path.join("."),
        description,
        args,
        input: first.to_string(),
        output: last.to_string(),
        nix_commit: "".to_string(), // TODO
        created_at: chrono::Utc::now().naive_utc(),
      };
    } else {
      for alias in func.meta.aliases.clone().unwrap_or_default() {
        if alias.len() == 3
          && alias[0] == "lib"
          && ["attrset", "string", "list"].contains(&alias[1].trim_end_matches('s'))
        {
          return Game {
            id: true,
            func: func.meta.path.join("."),
            description: description,
            args,
            input: alias[1].trim_end_matches('s').to_string(),
            output: "unknown (TODO)".to_string(),
            nix_commit: "".to_string(), // TODO
            created_at: chrono::Utc::now().naive_utc(),
          };
        }
      }

      if func.meta.path.len() == 2 && func.meta.path[0] == "builtins" {
        return Game {
          id: true,
          func: func.meta.path.join("."),
          description: description,
          args,
          input: "unknown (TODO)".to_string(),
          output: "unknown (TODO)".to_string(),
          nix_commit: "".to_string(), // TODO
          created_at: chrono::Utc::now().naive_utc(),
        };
      }
    }
  }
}
