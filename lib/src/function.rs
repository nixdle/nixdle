#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// represents a nix function
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Function {
  /// metadata
  pub meta: Meta,
  pub content: Option<Content>,
}

impl Function {
  /// gets the function description
  pub fn get_description(&self) -> Option<&str> {
    self.content.as_ref().and_then(|c| c.content.as_deref())
  }

  /// gets the number of arguments the function takes
  pub fn get_args_count(&self) -> usize {
    if let Some(primop) = &self.meta.primop_meta
      && let Some(args) = &primop.args
    {
      return args.len();
    }

    if let Some(signature) = &self.meta.signature {
      let s = signature.trim().to_lowercase().replace(" ", "");
      if let Some((_, input)) = s.split_once("::") {
        let parts: Vec<&str> = input.split("->").map(|p| p.trim()).collect();
        return parts.len().saturating_sub(1);
      }
    }

    0
  }

  /// gets the function input & output types
  pub fn get_types(&self, builtin_types: &[(String, String)]) -> Option<(Type, Type)> {
    if let Some(signature) = &self.meta.signature {
      return types_from_signature(signature);
    }

    if self.meta.path.len() == 2 && self.meta.path.first()? == "builtins" {
      let name = self.meta.path.last()?;
      if let Some((_, sig)) = builtin_types.iter().find(|(n, _)| n == name) {
        return types_from_signature(sig);
      }
      return None;
    }

    for alias in self.meta.aliases.as_ref().unwrap_or(&vec![]) {
      if alias.len() == 2 && alias.first()? == "builtins" {
        let name = alias.last()?;
        if let Some((_, sig)) = builtin_types.iter().find(|(n, _)| n == name) {
          return types_from_signature(sig);
        }
        return None;
      }
    }
    None
  }
}

/// function metadata
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Meta {
  /// full path to the function
  pub path: Vec<String>,
  /// short description of what the function does
  pub aliases: Option<Vec<Vec<String>>>,
  /// function type signature
  pub signature: Option<String>,
  pub is_primop: Option<bool>,
  /// primop metadata
  pub primop_meta: Option<PrimopMeta>,
}

/// primop specific metadata
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct PrimopMeta {
  /// function arguments
  args: Option<Vec<String>>,
}

/// function content
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
pub struct Content {
  /// function description
  content: Option<String>,
}

/// nix type
#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Type {
  Any,
  Attrset,
  Bool,
  Float,
  List(Box<Type>),
  Int,
  Never,
  Path,
  String,
}

impl Type {
  pub fn from_str(s: &str) -> Option<Self> {
    match s.trim().to_lowercase().as_str() {
      "any" => Some(Self::Any),
      "attrset" => Some(Self::Attrset),
      "bool" => Some(Self::Bool),
      "float" => Some(Self::Float),
      "int" => Some(Self::Int),
      "never" => Some(Self::Never),
      "path" => Some(Self::Path),
      "string" => Some(Self::String),
      a if a.starts_with('{') || a.ends_with('}') => Some(Self::Attrset),
      l if l.starts_with('[') && l.ends_with(']') => {
        let inner = &l[1..l.len() - 1];
        let inner_type = Self::from_str(inner)?;
        Some(Self::List(Box::new(inner_type)))
      }
      _ => None,
    }
  }
}

/// turns a signature into input & output types *magic*
pub fn types_from_signature(sig: &str) -> Option<(Type, Type)> {
  let s = sig.trim().to_lowercase().replace(" ", "");
  let s = s.split_once("::").map(|(_, t)| t.trim())?;

  let parts: Vec<&str> = s.split("->").map(|p| p.trim()).collect();
  let input = Type::from_str(parts.first()?)?;
  let output = Type::from_str(parts.get(1)?)?;

  Some((input, output))
}
