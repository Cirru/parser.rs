use std::clone::Clone;
use std::fmt;
// use std::marker::Copy;

#[derive(Clone)]
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

#[derive(fmt::Debug, PartialEq, Clone)]
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
      Self::CirruLeaf(a) => write!(f, "{}", a),
      Self::CirruList(xs) => {
        write!(f, "(")?;
        for (idx, x) in xs.iter().enumerate() {
          if idx > 0 {
            write!(f, ", ")?;
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

impl PartialEq for CirruNode {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::CirruLeaf(a), Self::CirruLeaf(b)) => a == b,
      (Self::CirruList(a), Self::CirruList(b)) => a == b,
      (_, _) => false,
    }
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
