use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::str;
// use std::marker::Copy;

use regex::Regex;

/// Cirru uses nested Vecters and Strings as data structure
#[derive(Clone)]
pub enum Cirru {
  Leaf(String),
  List(Vec<Cirru>),
}

#[derive(fmt::Debug, PartialEq)]
pub enum CirruLexState {
  LexStateSpace,
  LexStateToken,
  LexStateEscape,
  LexStateIndent,
  LexStateString,
}

/// internal control item during lexing
#[derive(fmt::Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum CirruLexItem {
  LexItemOpen,
  LexItemClose,
  LexItemIndent(usize),
  LexItemString(String),
}

pub type CirruLexItemList = Vec<CirruLexItem>;

impl fmt::Display for Cirru {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Cirru::Leaf(a) => {
        lazy_static! {
          static ref RE_SIMPLE_TOKEN: Regex = Regex::new(r"^[\w\d\-\?!]+$").unwrap();
        }
        if RE_SIMPLE_TOKEN.is_match(a) {
          write!(f, "{}", a)
        } else {
          write!(f, "\"{}\"", str::escape_debug(a))
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
        write!(f, ")")?;
        Ok(())
      }
    }
  }
}

impl fmt::Debug for Cirru {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // just use fn from Display
    write!(f, "{}", format!("{}", self))?;
    Ok(())
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
}
