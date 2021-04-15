use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt;
use std::str;
// use std::marker::Copy;

use regex::Regex;

/// Cirru uses nested Vecters and Strings as data structure
#[derive(Clone, Hash)]
pub enum CirruNode {
  CirruLeaf(String),
  CirruList(Vec<CirruNode>),
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

use CirruNode::{CirruLeaf, CirruList};

impl fmt::Display for CirruNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::CirruLeaf(a) => {
        let re = Regex::new(r"^[\w\d\-\?!]+$").unwrap();
        if re.is_match(a) {
          write!(f, "{}", a)
        } else {
          write!(f, "\"{}\"", str::escape_debug(a))
        }
      }
      Self::CirruList(xs) => {
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

impl fmt::Debug for CirruNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // just use fn from Display
    write!(f, "{}", format!("{}", self))?;
    Ok(())
  }
}

impl Eq for CirruNode {}

impl PartialEq for CirruNode {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::CirruLeaf(a), Self::CirruLeaf(b)) => a == b,
      (Self::CirruList(a), Self::CirruList(b)) => a == b,
      (_, _) => false,
    }
  }
}

impl Ord for CirruNode {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (CirruLeaf(a), CirruLeaf(b)) => a.cmp(b),
      (CirruLeaf(_), CirruList(_)) => Ordering::Less,
      (CirruList(_), CirruLeaf(_)) => Ordering::Greater,
      (CirruList(a), CirruList(b)) => a.cmp(b),
    }
  }
}

impl PartialOrd for CirruNode {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl CirruNode {
  pub fn len(&self) -> usize {
    match self {
      CirruLeaf(s) => s.len(),
      CirruList(xs) => xs.len(),
    }
  }
}
