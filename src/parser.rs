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

#[macro_use]
extern crate lazy_static;

mod tree;
mod types;
mod writer;

mod json;

use tree::{push_to_list, resolve_comma, resolve_dollar};
use types::CirruLexItem::*;
use types::CirruLexState::*;

pub use json::*;
pub use types::{escape_cirru_leaf, Cirru, CirruLexItem, CirruLexItemList};
pub use writer::*;

fn build_exprs(tokens: Vec<CirruLexItem>) -> Result<Vec<Cirru>, String> {
  let mut acc: Vec<Cirru> = vec![];
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
        let mut pointer: Vec<Cirru> = vec![];
        let mut pointer_stack: Vec<Vec<Cirru>> = vec![];
        loop {
          let cursor = pull_token(); // TODO Option
          match cursor {
            Some(LexItemClose) => {
              if pointer_stack.is_empty() {
                acc.push(Cirru::List(pointer.clone()));
                break;
              } else {
                let v = pointer_stack.pop();
                let prev_p = pointer;
                match v {
                  Some(collected) => {
                    pointer = collected;
                    pointer.push(Cirru::List(prev_p))
                  }
                  None => return Err(String::from("unknown close item")),
                }
              }
            }
            Some(LexItemOpen) => {
              pointer_stack.push(pointer);
              pointer = vec![];
            }
            Some(LexItemString(s)) => pointer.push(Cirru::Leaf(s)),
            Some(LexItemIndent(n)) => return Err(format!("unknown indent: {}", n)),
            None => return Err(String::from("unexpected end of file")),
          }
        }
      }
      Some(LexItemClose) => return Err(String::from("unexpected \")\"")),
      Some(a) => return Err(format!("unknown item: {:?}", a)),
      None => return Ok(acc),
    }
  }
}

fn parse_indentation(buffer: String) -> Result<CirruLexItem, String> {
  let size = buffer.len();
  if size % 2 == 0 {
    Ok(LexItemIndent(size / 2))
  } else {
    Err(format!("odd indentation size, {}", buffer.escape_default()))
  }
}

/// internal function for lexing
pub fn lex(initial_code: &str) -> Result<CirruLexItemList, String> {
  let mut acc: CirruLexItemList = vec![];
  let mut state = LexStateIndent;
  let mut buffer = String::from("");
  let code = initial_code;

  for c in code.chars() {
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
          buffer.push(c);
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
          return Err(String::from("unexpected newline in string"));
        }
        _ => {
          state = LexStateString;
          buffer.push(c);
        }
      },
      LexStateEscape => match c {
        '"' => {
          state = LexStateString;
          buffer.push('"');
        }
        't' => {
          state = LexStateString;
          buffer.push('\t');
        }
        'n' => {
          state = LexStateString;
          buffer.push('\n');
        }
        '\\' => {
          state = LexStateString;
          buffer.push('\\');
        }
        _ => return Err(String::from("unexpected character during string escaping")),
      },
      LexStateIndent => match c {
        ' ' => {
          state = LexStateIndent;
          buffer.push(c);
        }
        '\n' => {
          state = LexStateIndent;
          buffer = String::from("");
        }
        '"' => {
          let level = parse_indentation(buffer)?;
          acc.push(level);
          state = LexStateString;
          buffer = String::from("");
        }
        '(' => {
          let level = parse_indentation(buffer)?;
          acc.push(level);
          acc.push(LexItemOpen);
          state = LexStateSpace;
          buffer = String::from("");
        }
        ')' => return Err(String::from("unexpected ) at line start")),
        _ => {
          let level = parse_indentation(buffer)?;
          acc.push(level);
          state = LexStateToken;
          buffer = String::from(c);
        }
      },
    }
  }

  match state {
    LexStateSpace => Ok(acc),
    LexStateToken => {
      acc.push(LexItemString(buffer));
      Ok(acc)
    }
    LexStateEscape => Err(String::from("unknown escape")),
    LexStateIndent => Ok(acc),
    LexStateString => Err(String::from("finished at string")),
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

/// parse function, parse String to Cirru.
///
/// ```rust
/// cirru_parser::parse("def a 1");
/// ```
pub fn parse(code: &str) -> Result<Cirru, String> {
  let tokens = resolve_indentations(lex(code)?);
  // println!("{:?}", tokens);
  let tree = build_exprs(tokens)?;
  // println!("tree {:?}", tree);
  let v = Cirru::List(resolve_comma(&resolve_dollar(&tree)));
  Ok(v)
}
