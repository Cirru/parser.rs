use crate::types;

pub use types::*;

pub use types::Cirru;

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

pub fn resolve_comma(xs: &[Cirru]) -> Vec<Cirru> {
  if xs.is_empty() {
    return vec![];
  } else {
    comma_helper(xs)
  }
}

fn comma_helper(intial_after: &[Cirru]) -> Vec<Cirru> {
  let mut before: Vec<Cirru> = vec![];
  let after: &[Cirru] = intial_after;

  let mut pointer = 0;

  loop {
    if pointer >= after.len() {
      return before;
    }
    let cursor = &after[pointer];
    match cursor {
      Cirru::List(xs) => {
        if !xs.is_empty() {
          let head = &xs[0];
          match head {
            Cirru::List(_) => {
              before.push(Cirru::List(resolve_comma(&xs)));
            }
            Cirru::Leaf(s) => {
              if s == "," {
                before.extend(resolve_comma(&xs[1..]))
              } else {
                before.push(Cirru::List(resolve_comma(&xs)));
              }
            }
          }
        } else {
          before.push(Cirru::List(vec![]));
        }
      }
      Cirru::Leaf(_) => {
        before.push(cursor.clone());
      }
    }
    pointer += 1;
  }
}

pub fn resolve_dollar(xs: &[Cirru]) -> Vec<Cirru> {
  if xs.is_empty() {
    return vec![];
  } else {
    dollar_helper(xs)
  }
}

fn dollar_helper(initial_after: &[Cirru]) -> Vec<Cirru> {
  let mut before: Vec<Cirru> = vec![];
  let after: &[Cirru] = initial_after;

  let mut pointer = 0;

  loop {
    if pointer >= after.len() {
      return before;
    } else {
      let cursor = &after[pointer];

      match cursor {
        Cirru::List(xs) => {
          before.push(Cirru::List(resolve_dollar(&xs)));
          pointer += 1;
        }
        Cirru::Leaf(s) => {
          if s == "$" {
            before.push(Cirru::List(resolve_dollar(&after[pointer + 1..])));
            pointer = after.len();
          } else {
            before.push(Cirru::Leaf(s.to_string()));
            pointer += 1;
          }
        }
      }
    }
  }
}
