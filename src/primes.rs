use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str;
// use std::marker::Copy;

#[cfg(feature = "use-serde")]
use serde::de::{SeqAccess, Visitor};
#[cfg(feature = "use-serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "use-serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::s_expr;

/// Cirru uses nested Vecters and Strings as data structure
#[derive(Clone)]
pub enum Cirru {
  Leaf(Box<str>),
  List(Vec<Cirru>),
}

#[derive(fmt::Debug, PartialEq)]
pub enum CirruLexState {
  Space,
  Token,
  Escape,
  Indent,
  Str,
}

/// internal control item during lexing
#[derive(fmt::Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum CirruLexItem {
  Open,
  Close,
  Indent(usize),
  Str(String),
}

pub type CirruLexItemList = Vec<CirruLexItem>;

impl fmt::Display for Cirru {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Cirru::Leaf(a) => {
        if is_normal_str(a) {
          write!(f, "{}", a)
        } else {
          write!(f, "{}", escape_cirru_leaf(a))
        }
      }
      Cirru::List(xs) => {
        write!(f, "(")?;
        for (idx, x) in xs.iter().enumerate() {
          if idx > 0 {
            write!(f, " ")?;
          }
          write!(f, "{}", x)?;
        }
        write!(f, ")")
      }
    }
  }
}

fn is_normal_str(tok: &str) -> bool {
  for s in tok.chars() {
    if !matches!(s, 'A'..='Z' | 'a'..='z'|'0'..='9' | '-' | '?' |'!'|'+'|'*'|'$'|'@'|'#'|'%'|'&'|'_'|'='|'|'|':'|'.'|'<'|'>') {
      return false;
    }
  }

  true
}

/// common API for turning Cirru leaf with strings escaped
/// ```rust
/// use cirru_parser::escape_cirru_leaf;
/// escape_cirru_leaf("a"); // "\"a\""
/// escape_cirru_leaf("a b"); // "\"a b\""
/// ```
pub fn escape_cirru_leaf(s: &str) -> String {
  let mut chunk = String::with_capacity(s.len() + 1);
  chunk.push('\"');
  if is_normal_str(s) {
    chunk.push_str(s);
  } else {
    for c in s.chars() {
      match c {
        '\n' => chunk.push_str("\\n"),
        '\t' => chunk.push_str("\\t"),
        '\"' => chunk.push_str("\\\""),
        '\\' => chunk.push_str("\\\\"),
        '\'' => chunk.push_str("\\'"),
        _ => chunk.push(c),
      }
    }
  }
  chunk.push('"');
  chunk
}

impl fmt::Debug for Cirru {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // just use fn from Display
    write!(f, "{}", self)
  }
}

impl Eq for Cirru {}

impl PartialEq for Cirru {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Leaf(a), Self::Leaf(b)) => a == b,
      (Self::List(a), Self::List(b)) => a == b,
      (_, _) => false,
    }
  }
}

impl Ord for Cirru {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::Leaf(a), Self::Leaf(b)) => a.cmp(b),
      (Self::Leaf(_), Self::List(_)) => Ordering::Less,
      (Self::List(_), Self::Leaf(_)) => Ordering::Greater,
      (Self::List(a), Self::List(b)) => a.cmp(b),
    }
  }
}

impl PartialOrd for Cirru {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Hash for Cirru {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Self::Leaf(s) => {
        s.hash(state);
      }
      Self::List(xs) => {
        xs.hash(state);
      }
    }
  }
}

impl Cirru {
  pub fn len(&self) -> usize {
    match self {
      Self::Leaf(s) => s.len(),
      Self::List(xs) => xs.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    match self {
      Self::Leaf(s) => s.len() == 0,
      Self::List(xs) => xs.len() == 0,
    }
  }

  pub fn to_lisp(&self) -> Result<String, String> {
    match self {
      Cirru::Leaf(_) => Err(format!("expected list to convert to Lisp, got {}", self)),
      Cirru::List(xs) => s_expr::format_to_lisp(xs),
    }
  }

  pub fn leaf<T: Into<String>>(s: T) -> Self {
    Cirru::Leaf(s.into().into_boxed_str())
  }
}

#[cfg(feature = "use-serde")]
impl Serialize for Cirru {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Cirru::Leaf(s) => serializer.serialize_str(s),
      Cirru::List(xs) => {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in xs {
          seq.serialize_element(e)?;
        }
        seq.end()
      }
    }
  }
}

#[cfg(feature = "use-serde")]
impl<'de> Visitor<'de> for Cirru {
  type Value = Cirru;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a seq for Cirru")
  }

  fn visit_seq<M>(self, mut access: M) -> Result<Self::Value, M::Error>
  where
    M: SeqAccess<'de>,
  {
    let mut seq = Vec::with_capacity(access.size_hint().unwrap_or(0));
    while let Some(el) = access.next_element()? {
      seq.push(el);
    }

    Ok(Cirru::List(seq))
  }

  fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> {
    Ok(Cirru::leaf(s))
  }
}

#[cfg(feature = "use-serde")]
impl<'de> Deserialize<'de> for Cirru {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_any(Cirru::List(Vec::new()))
  }
}
