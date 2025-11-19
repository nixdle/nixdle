//! nixdle - wordle but it's nix functions

use rand::prelude::IndexedRandom;
use serde::Deserialize;
use sqlx::{prelude::FromRow, types::chrono};

#[allow(dead_code)]
const NEXT_CLUE_ATTEMPTS: u8 = 5;

#[allow(dead_code)]
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

pub fn parse_builtin_types(data: &str) -> Result<Vec<(String, String)>, serde_json::Error> {
  let map: serde_json::Value = serde_json::from_str(data)?;
  let mut result = Vec::new();

  if let serde_json::Value::Object(obj) = map {
    for (key, value) in obj {
      if let serde_json::Value::Object(inner_obj) = value
        && let Some(serde_json::Value::String(fn_type)) = inner_obj.get("fn_type")
      {
        result.push((key, fn_type.clone()));
      }
    }
  }

  Ok(result)
}

pub fn new(functions: Vec<Function>, builtin_types: Vec<(String, String)>) -> Game {
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
      let (first, last) = match signature_to_types(signature) {
        Some((f, l)) => (f, l),
        None => {
          continue;
        }
      };

      return Game {
        id: true,
        func: func.meta.path.join("."),
        description,
        args,
        input: first,
        output: last,
        nix_commit: "".to_string(), // TODO
        created_at: chrono::Utc::now().naive_utc(),
      };
    } else if func.meta.path.len() == 2 && func.meta.path[0] == "builtins" {
      let builtin_name = &func.meta.path[1];

      if let Some((input, output)) = builtin_types
        .iter()
        .find(|(name, _)| name == builtin_name)
        .and_then(|(_, fn_type)| signature_to_types(fn_type))
      {
        return Game {
          id: true,
          func: func.meta.path.join("."),
          description,
          args,
          input,
          output,
          nix_commit: "".to_string(), // TODO
          created_at: chrono::Utc::now().naive_utc(),
        };
      }
    } else {
      for alias in func.meta.aliases.clone().unwrap_or_default() {
        if alias.len() == 2 && alias[0] == "builtins" {
          let builtin_name = &alias[1];

          if let Some((input, output)) = builtin_types
            .iter()
            .find(|(name, _)| name == builtin_name)
            .and_then(|(_, fn_type)| signature_to_types(fn_type))
          {
            return Game {
              id: true,
              func: func.meta.path.join("."),
              description,
              args,
              input,
              output,
              nix_commit: "".to_string(), // TODO
              created_at: chrono::Utc::now().naive_utc(),
            };
          }
        }
      }
    }
  }
}

fn signature_to_types(signature: &str) -> Option<(String, String)> {
  let s = signature.trim().to_lowercase();
  let sig = match s.split_once(" :: ").map(|(_, sig)| sig) {
    Some(sig) => sig,
    None => {
      return None;
    }
  };

  let parts: Vec<&str> = sig.split(" -> ").map(|p| p.trim()).collect();

  let first = match parts.first() {
    Some(f) if f.starts_with('{') => "attrset",
    Some(f) => f,
    None => {
      return None;
    }
  };
  let last = match parts.last() {
    Some(l) if l.ends_with('}') => "attrset",
    Some(l) => l,
    None => {
      return None;
    }
  };

  if first.starts_with('(') || first.contains(' ') || last.ends_with(')') || last.contains(' ') {
    return None;
  }

  Some((first.to_string(), last.to_string()))
}
