mod tree;
mod types;

use modulo::Mod;

use tree::{push_to_list, resolve_comma, resolve_dollar};
use types::CirruLexItem::*;
use types::CirruLexState::*;
use types::CirruNode::*;
use types::{CirruLexItem, CirruLexItemList, CirruNode};

fn build_exprs(tokens: Vec<CirruLexItem>) -> Vec<CirruNode> {
  let mut acc: Vec<CirruNode> = vec![];
  let mut pointer = 0;
  let mut pull_token = || {
    let c = tokens[pointer].clone();
    pointer += 1;
    return c;
  };
  loop {
    let chunk = pull_token(); // TODO Option

    match chunk {
      LexItemOpen => {
        let mut pointer: Vec<CirruNode> = vec![];
        let mut pointer_stack: Vec<Vec<CirruNode>> = vec![];
        loop {
          let cursor: CirruLexItem = pull_token(); // TODO Option
          match cursor {
            LexItemClose => {
              if pointer_stack.len() == 0 {
                acc.push(CirruList(pointer.clone()));
                break;
              } else {
                let v = pointer_stack.pop();
                match v {
                  Some(collected) => pointer.push(CirruList(collected)),
                  None => unreachable!(),
                }
              }
            }
            LexItemOpen => {
              pointer_stack.push(pointer);
              pointer = vec![];
            }
            LexItemString(s) => pointer.push(CirruLeaf(s)),
            LexItemIndent(_) => unreachable!(),
          }
        }
      }
      _ => unreachable!("unknown chunk"),
    }
  }
}

fn parse_indentation(buffer: String) -> usize {
  let size = buffer.len();
  if size.modulo(2) == 1 {
    panic!("odd indentation size") // TODO
  }
  return size / 2;
}

fn lex(initial_code: String) -> CirruLexItemList {
  let mut acc: CirruLexItemList = vec![];
  let mut state = LexStateIndent;
  let mut buffer = String::from("");
  let code = initial_code;
  let mut count = 0;

  let mut pointer = 0;

  loop {
    count += 1;
    if count > 1000 {
      panic!("looped too many times")
    }
    if pointer >= code.len() {
      match state {
        LexStateSpace => return acc,
        LexStateToken => acc.push(LexItemString(buffer.clone())), // TODO why clone?
        LexStateEscape => panic!("unknown escape"),
        LexStateIndent => return acc,
        LexStateString => panic!("finished at string"),
      }
    } else {
      let c = code.chars().nth(pointer).unwrap();
      pointer += 1;
      match state {
        LexStateSpace => match c {
          ' ' => {
            state = LexStateSpace;
            buffer = String::from("");
          }
          '\n' => {
            state = LexStateIndent;
            buffer = String::from("");
          }
          '(' => {
            acc.push(LexItemOpen);
            state = LexStateSpace;
            buffer = String::from("")
          }
          ')' => {
            acc.push(LexItemClose);
            state = LexStateSpace;
            buffer = String::from("")
          }
          '"' => {
            state = LexStateString;
            buffer = String::from("");
          }
          _ => {
            state = LexStateToken;
            buffer = String::from(c)
          }
        },
        LexStateToken => match c {
          ' ' => {
            acc.push(LexItemString(buffer));
            state = LexStateSpace;
            buffer = String::from("");
          }
          '"' => {
            acc.push(LexItemString(buffer));
            state = LexStateString;
            buffer = String::from("");
          }
          '\n' => {
            acc.push(LexItemString(buffer));
            state = LexStateIndent;
            buffer = String::from("");
          }
          '(' => {
            acc.push(LexItemOpen);
            state = LexStateSpace;
            buffer = String::from("")
          }
          ')' => {
            acc.push(LexItemClose);
            state = LexStateSpace;
            buffer = String::from("")
          }
          _ => {
            state = LexStateToken;
            buffer = format!("{}{}", buffer, c);
          }
        },
        LexStateString => match c {
          '"' => {
            acc.push(LexItemString(buffer));
            state = LexStateSpace;
            buffer = String::from("");
          }
          '\\' => {
            state = LexStateEscape;
          }
          '\n' => {
            panic!("unexpected newline in string");
          }
          _ => {
            state = LexStateString;
            buffer = format!("{}{}", buffer, c);
          }
        },
        LexStateEscape => match c {
          '"' => {
            state = LexStateString;
            buffer = format!("{}{}", buffer, '"');
          }
          't' => {
            state = LexStateString;
            buffer = format!("{}{}", buffer, '\t');
          }
          'n' => {
            state = LexStateString;
            buffer = format!("{}{}", buffer, '\n');
          }
          '\\' => {
            state = LexStateString;
            buffer = format!("{}{}", buffer, '\\');
          }
          _ => panic!("unexpected character during string escaping"),
        },
        LexStateIndent => match c {
          ' ' => {
            state = LexStateIndent;
            buffer = format!("{}{}", buffer, c);
          }
          '\n' => {
            state = LexStateIndent;
            buffer = String::from("");
          }
          '"' => {
            // acc.push(parseIndentation(buffer)) // TODO
            state = LexStateString;
            buffer = String::from("");
          }
          '(' => {
            // acc.push(parseIndentation(buffer), ELexControl.open); // TODO
            state = LexStateSpace;
            buffer = String::from("");
          }
          ')' => {
            panic!("unexpected ) at line start")
          }
          _ => {
            // acc.push(parseIndentation(buffer)); // TODO
            state = LexStateToken;
            buffer = String::from(c);
          }
        },
      }
    }
  }
}

fn repeat<T: Clone>(times: usize, x: T) -> Vec<T> {
  let mut acc: Vec<T> = vec![];
  for _ in 0..times {
    acc.push(x.clone());
  }
  acc
}

fn resolve_indentations(initial_tokens: CirruLexItemList) -> CirruLexItemList {
  let mut acc: CirruLexItemList = vec![];
  let mut level = 0;
  let tokens: CirruLexItemList = initial_tokens;
  let mut pointer = 0;
  loop {
    if pointer >= tokens.len() {
      if acc.len() == 0 {
        return vec![];
      } else {
        acc.insert(0, LexItemOpen);
        acc = push_to_list(acc, vec![repeat(level, LexItemClose), vec![LexItemClose]]);
        return acc;
      }
    } else {
      let cursor = tokens[pointer].clone();
      match cursor {
        LexItemString(s) => {
          acc.push(LexItemString(s));
          pointer += 1;
        }
        LexItemOpen => {
          acc.push(LexItemOpen);
          pointer += 1;
        }
        LexItemClose => {
          acc.push(LexItemClose);
          pointer += 1;
        }
        LexItemIndent(n) => {
          if n > level {
            let delta = n - level;
            acc = push_to_list(acc, vec![repeat(delta, LexItemOpen)]);
            pointer += 1;
            level = n;
          } else if n < level {
            let delta = level - n;
            acc = push_to_list(
              acc,
              vec![repeat(delta, LexItemClose), vec![LexItemClose, LexItemOpen]],
            );
            pointer += 1;
            level = n;
          } else {
            if acc.len() == 0 {
              acc = vec![];
            } else {
              acc.push(LexItemClose);
              acc.push(LexItemOpen);
            }
          }
        }
      }
    }
  }
}

pub fn parse(code: String) -> CirruNode {
  // TODO
  let tokens = resolve_indentations(lex(code));
  // return resolveComma(resolveDollar(buildExprs(pullToken)));
  return CirruList(resolve_comma(resolve_dollar(build_exprs(tokens))));
}

#[cfg(test)]
mod test_parser {
  use super::parse;
  use super::CirruNode;

  #[test]
  fn parse_a() {
    // assert_eq!(
    //   parse(String::from("demo")),
    //   CirruNode::CirruLeaf(String::from("TODO"))
    // );
  }
}
