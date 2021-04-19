use crate::types;

pub use types::*;

pub use types::CirruNode;
pub use types::CirruNode::*;

// mutable to acc
pub fn push_to_list<T: Clone>(acc: Vec<T>, xss: Vec<Vec<T>>) -> Vec<T> {
  let mut result = acc;
  for xs in xss {
    for x in xs {
      result.push(x)
    }
  }
  result
}

pub fn resolve_comma(xs: &[CirruNode]) -> Vec<CirruNode> {
  if xs.is_empty() {
    return vec![];
  } else {
    comma_helper(xs)
  }
}

fn comma_helper(intial_after: &[CirruNode]) -> Vec<CirruNode> {
  let mut before: Vec<CirruNode> = vec![];
  let after: &[CirruNode] = intial_after;

  let mut pointer = 0;

  loop {
    if pointer >= after.len() {
      return before;
    }
    let cursor = &after[pointer];
    match cursor {
      CirruList(xs) => {
        if !xs.is_empty() {
          let head = &xs[0];
          match head {
            CirruList(_) => {
              before.push(CirruList(resolve_comma(&xs)));
            }
            CirruLeaf(s) => {
              if s == "," {
                before.extend(resolve_comma(&xs[1..]))
              } else {
                before.push(CirruList(resolve_comma(&xs)));
              }
            }
          }
        } else {
          before.push(CirruList(vec![]));
        }
      }
      CirruLeaf(_) => {
        before.push(cursor.clone());
      }
    }
    pointer += 1;
  }
}

pub fn resolve_dollar(xs: &[CirruNode]) -> Vec<CirruNode> {
  if xs.is_empty() {
    return vec![];
  } else {
    dollar_helper(xs)
  }
}

fn dollar_helper(initial_after: &[CirruNode]) -> Vec<CirruNode> {
  let mut before: Vec<CirruNode> = vec![];
  let after: &[CirruNode] = initial_after;

  let mut pointer = 0;

  loop {
    if pointer >= after.len() {
      return before;
    } else {
      let cursor = &after[pointer];

      match cursor {
        CirruList(xs) => {
          before.push(CirruList(resolve_dollar(&xs)));
          pointer += 1;
        }
        CirruLeaf(s) => {
          if s == "$" {
            before.push(CirruList(resolve_dollar(&after[pointer + 1..])));
            pointer = after.len();
          } else {
            before.push(CirruLeaf(s.to_string()));
            pointer += 1;
          }
        }
      }
    }
  }
}
