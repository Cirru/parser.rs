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
  LexItemEof,
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
  fn len(&self) -> usize {
    match self {
      CirruLeaf(s) => s.len(),
      CirruList(xs) => xs.len(),
    }
  }
}

#[cfg(test)]
mod test_data {
  use super::CirruNode::{CirruLeaf, CirruList};

  #[test]
  fn test_eq() {
    assert_eq!(CirruLeaf(String::from("a")), CirruLeaf(String::from("a")));
    assert_ne!(CirruLeaf(String::from("b")), CirruLeaf(String::from("a")));
    assert_ne!(
      CirruLeaf(String::from("a")),
      CirruList(vec![CirruLeaf(String::from("a"))]),
    );
    assert_eq!(
      CirruList(vec![CirruLeaf(String::from("a"))]),
      CirruList(vec![CirruLeaf(String::from("a"))]),
    );
  }

  #[test]
  fn test_fmt() {
    let a = CirruLeaf(String::from("na"));
    assert_eq!(format!("{}", a), "na");

    let b = a.clone();
    let c = CirruList(vec![a, b]);
    assert_eq!(format!("{}", c), "(na, na)")
  }

  #[test]
  fn test_len() {
    assert_eq!(1, CirruLeaf(String::from("1")).len());
    assert_eq!(1, CirruList(vec![CirruLeaf(String::from("1"))]).len());
  }
}
