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

pub fn resolve_comma(xs: Vec<CirruNode>) -> Vec<CirruNode> {
  if xs.is_empty() {
    return vec![];
  } else {
    comma_helper(xs)
  }
}

fn comma_helper(intial_after: Vec<CirruNode>) -> Vec<CirruNode> {
  let mut before: Vec<CirruNode> = vec![];
  let after: Vec<CirruNode> = intial_after;

  let mut pointer = 0;

  loop {
    if pointer >= after.len() {
      return before;
    }
    let cursor = after[pointer].clone();
    match cursor {
      CirruList(xs) => {
        if !xs.is_empty() {
          let head = xs[0].clone();
          match head {
            CirruList(_) => {
              before.push(CirruList(resolve_comma(xs)));
            }
            CirruLeaf(s) => {
              if s == "," {
                before.extend(resolve_comma(xs[1..].to_vec()))
              } else {
                before.push(CirruList(resolve_comma(xs)));
              }
            }
          }
        } else {
          before.push(CirruList(vec![]));
        }
      }
      CirruLeaf(_) => {
        before.push(cursor);
      }
    }
    pointer += 1;
  }
}

pub fn resolve_dollar(xs: Vec<CirruNode>) -> Vec<CirruNode> {
  if xs.is_empty() {
    return vec![];
  } else {
    dollar_helper(xs)
  }
}

fn dollar_helper(initial_after: Vec<CirruNode>) -> Vec<CirruNode> {
  let mut before: Vec<CirruNode> = vec![];
  let after: Vec<CirruNode> = initial_after;

  let mut pointer = 0;

  loop {
    if pointer >= after.len() {
      return before;
    } else {
      let cursor = after[pointer].clone();

      match cursor {
        CirruList(xs) => {
          before.push(CirruList(resolve_dollar(xs)));
          pointer += 1;
        }
        CirruLeaf(s) => {
          if s == "$" {
            before.push(CirruList(resolve_dollar(after[pointer + 1..].to_vec())));
            pointer = after.len();
          } else {
            before.push(CirruLeaf(s));
            pointer += 1;
          }
        }
      }
    }
  }
}
