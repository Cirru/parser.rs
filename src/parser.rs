/*! # Cirru Parser
This tiny parser parses indentation based syntax into nested a vector,
then it could used as S-Expressions for evaluation or codegen.

```cirru
defn fib (x)
  if (<= x 2) 1
    +
      fib $ dec x
      fib $ - x 2
```

parses to:

```edn
[ ["defn" "fib" [ "x" ]
    [ "if" [ "<=" "x" "2" ] "1"
      [ "+" [ "fib" ["dec" "x"] ] [ "fib" ["-" "x" "2"] ] ]
    ]
] ]
```

find more on <http://text.cirru.org/> .
*/

mod tree;
mod types;
mod writer;

mod json;

use modulo::Mod;

use tree::{push_to_list, resolve_comma, resolve_dollar};
use types::CirruLexItem::*;
use types::CirruLexState::*;
use types::CirruNode::*;

pub use json::*;
pub use types::{CirruLexItem, CirruLexItemList, CirruNode};
pub use writer::*;

fn build_exprs(tokens: Vec<CirruLexItem>) -> Vec<CirruNode> {
  let mut acc: Vec<CirruNode> = vec![];
  let mut idx = 0;
  let mut pull_token = || {
    if idx >= tokens.len() {
      return None;
    }
    let c = tokens[idx].clone();
    idx += 1;
    Some(c)
  };
  loop {
    let chunk = pull_token(); // TODO Option

    match chunk {
      Some(LexItemOpen) => {
        let mut pointer: Vec<CirruNode> = vec![];
        let mut pointer_stack: Vec<Vec<CirruNode>> = vec![];
        loop {
          let cursor = pull_token(); // TODO Option
          match cursor {
            Some(LexItemClose) => {
              if pointer_stack.is_empty() {
                acc.push(CirruList(pointer.clone()));
                break;
              } else {
                let v = pointer_stack.pop();
                let prev_p = pointer;
                match v {
                  Some(collected) => {
                    pointer = collected;
                    pointer.push(CirruList(prev_p))
                  }
                  None => unreachable!(),
                }
              }
            }
            Some(LexItemOpen) => {
              pointer_stack.push(pointer);
              pointer = vec![];
            }
            Some(LexItemString(s)) => pointer.push(CirruLeaf(s)),
            Some(LexItemIndent(_)) => unreachable!(),
            None => unreachable!("unexpected end of file"),
          }
        }
      }
      Some(LexItemClose) => unreachable!("unexpected \")\""),
      Some(a) => unreachable!(format!("unknown item: {:?}", a)),
      None => return acc,
    }
  }
}

fn parse_indentation(buffer: String) -> CirruLexItem {
  let size = buffer.len();
  if size.modulo(2) == 1 {
    panic!("odd indentation size")
  }
  LexItemIndent(size / 2)
}

/// internal function for lexing
pub fn lex(initial_code: String) -> CirruLexItemList {
  let mut acc: CirruLexItemList = vec![];
  let mut state = LexStateIndent;
  let mut buffer = String::from("");
  let code = initial_code;

  let mut pointer = 0;

  loop {
    if pointer >= code.len() {
      match state {
        LexStateSpace => return acc,
        LexStateToken => {
          acc.push(LexItemString(buffer)); // TODO why clone?
          return acc;
        }
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
            acc.push(LexItemString(buffer));
            acc.push(LexItemOpen);
            state = LexStateSpace;
            buffer = String::from("")
          }
          ')' => {
            acc.push(LexItemString(buffer));
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
            acc.push(parse_indentation(buffer));
            state = LexStateString;
            buffer = String::from("");
          }
          '(' => {
            acc.push(parse_indentation(buffer));
            acc.push(LexItemOpen);
            state = LexStateSpace;
            buffer = String::from("");
          }
          ')' => {
            panic!("unexpected ) at line start")
          }
          _ => {
            acc.push(parse_indentation(buffer));
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

/// internal function for figuring out indentations after lexing
pub fn resolve_indentations(initial_tokens: CirruLexItemList) -> CirruLexItemList {
  let mut acc: CirruLexItemList = vec![];
  let mut level = 0;
  let tokens: CirruLexItemList = initial_tokens;
  let mut pointer = 0;
  loop {
    if pointer >= tokens.len() {
      if acc.is_empty() {
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
        LexItemIndent(n) => match n {
          _ if n > level => {
            let delta = n - level;
            acc = push_to_list(acc, vec![repeat(delta, LexItemOpen)]);
            pointer += 1;
            level = n;
          }
          _ if n < level => {
            let delta = level - n;
            acc = push_to_list(
              acc,
              vec![repeat(delta, LexItemClose), vec![LexItemClose, LexItemOpen]],
            );
            pointer += 1;
            level = n;
          }
          _ => {
            if acc.is_empty() {
              acc = vec![];
            } else {
              acc.push(LexItemClose);
              acc.push(LexItemOpen);
            }
            pointer += 1;
          }
        },
      }
    }
  }
}

/// parse function, parse String to CirruNode.
///
/// ```rs
/// parse_cirru(String::from("def a 1"))
/// ```
pub fn parse_cirru(code: String) -> Result<CirruNode, String> {
  let tokens = resolve_indentations(lex(code));
  // println!("{:?}", tokens);
  let tree = build_exprs(tokens);
  // println!("tree {:?}", tree);
  let v = CirruList(resolve_comma(resolve_dollar(tree)));
  Ok(v)
}

#[deprecated(since = "0.0.8", note = "use parse_cirru instead")]
pub fn parse(code: String) -> Result<CirruNode, String> {
  parse_cirru(code)
}
