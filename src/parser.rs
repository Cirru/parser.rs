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

use tree::{resolve_comma, resolve_dollar};
use types::CirruLexState;

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
    let c = tokens[idx].to_owned();
    idx += 1;
    Some(c)
  };
  loop {
    let chunk = pull_token(); // TODO Option

    if chunk == None {
      return Ok(acc);
    }
    match chunk.unwrap() {
      CirruLexItem::Open => {
        let mut pointer: Vec<Cirru> = vec![];
        let mut pointer_stack: Vec<Vec<Cirru>> = vec![];
        loop {
          let cursor = pull_token(); // TODO Option
          if cursor == None {
            return Err(String::from("unexpected end of file"));
          }
          match cursor.unwrap() {
            CirruLexItem::Close => {
              if pointer_stack.is_empty() {
                acc.push(Cirru::List(pointer.to_owned()));
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
            CirruLexItem::Open => {
              pointer_stack.push(pointer);
              pointer = vec![];
            }
            CirruLexItem::Str(s) => pointer.push(Cirru::Leaf(s)),
            CirruLexItem::Indent(n) => return Err(format!("unknown indent: {}", n)),
          }
        }
      }
      CirruLexItem::Close => return Err(String::from("unexpected \")\"")),
      a => return Err(format!("unknown item: {:?}", a)),
    }
  }
}

fn parse_indentation(buffer: String) -> Result<CirruLexItem, String> {
  let size = buffer.len();
  if size % 2 == 0 {
    Ok(CirruLexItem::Indent(size / 2))
  } else {
    Err(format!("odd indentation size, {}", buffer.escape_default()))
  }
}

/// internal function for lexing
pub fn lex(initial_code: &str) -> Result<CirruLexItemList, String> {
  let mut acc: CirruLexItemList = vec![];
  let mut state = CirruLexState::Indent;
  let mut buffer = String::from("");
  let code = initial_code;

  for c in code.chars() {
    match state {
      CirruLexState::Space => match c {
        ' ' => {
          state = CirruLexState::Space;
          buffer = String::from("");
        }
        '\n' => {
          state = CirruLexState::Indent;
          buffer = String::from("");
        }
        '(' => {
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer = String::from("")
        }
        ')' => {
          acc.push(CirruLexItem::Close);
          state = CirruLexState::Space;
          buffer = String::from("")
        }
        '"' => {
          state = CirruLexState::Str;
          buffer = String::from("");
        }
        _ => {
          state = CirruLexState::Token;
          buffer = String::from(c)
        }
      },
      CirruLexState::Token => match c {
        ' ' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Space;
          buffer = String::from("");
        }
        '"' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Str;
          buffer = String::from("");
        }
        '\n' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Indent;
          buffer = String::from("");
        }
        '(' => {
          acc.push(CirruLexItem::Str(buffer));
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer = String::from("")
        }
        ')' => {
          acc.push(CirruLexItem::Str(buffer));
          acc.push(CirruLexItem::Close);
          state = CirruLexState::Space;
          buffer = String::from("")
        }
        _ => {
          state = CirruLexState::Token;
          buffer.push(c);
        }
      },
      CirruLexState::Str => match c {
        '"' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Space;
          buffer = String::from("");
        }
        '\\' => {
          state = CirruLexState::Escape;
        }
        '\n' => {
          return Err(String::from("unexpected newline in string"));
        }
        _ => {
          state = CirruLexState::Str;
          buffer.push(c);
        }
      },
      CirruLexState::Escape => match c {
        '"' => {
          state = CirruLexState::Str;
          buffer.push('"');
        }
        't' => {
          state = CirruLexState::Str;
          buffer.push('\t');
        }
        'n' => {
          state = CirruLexState::Str;
          buffer.push('\n');
        }
        '\\' => {
          state = CirruLexState::Str;
          buffer.push('\\');
        }
        _ => return Err(String::from("unexpected character during string escaping")),
      },
      CirruLexState::Indent => match c {
        ' ' => {
          state = CirruLexState::Indent;
          buffer.push(c);
        }
        '\n' => {
          state = CirruLexState::Indent;
          buffer = String::from("");
        }
        '"' => {
          let level = parse_indentation(buffer)?;
          acc.push(level);
          state = CirruLexState::Str;
          buffer = String::from("");
        }
        '(' => {
          let level = parse_indentation(buffer)?;
          acc.push(level);
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer = String::from("");
        }
        ')' => return Err(String::from("unexpected ) at line start")),
        _ => {
          let level = parse_indentation(buffer)?;
          acc.push(level);
          state = CirruLexState::Token;
          buffer = String::from(c);
        }
      },
    }
  }

  match state {
    CirruLexState::Space => Ok(acc),
    CirruLexState::Token => {
      acc.push(CirruLexItem::Str(buffer));
      Ok(acc)
    }
    CirruLexState::Escape => Err(String::from("unknown escape")),
    CirruLexState::Indent => Ok(acc),
    CirruLexState::Str => Err(String::from("finished at string")),
  }
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
        acc.insert(0, CirruLexItem::Open);
        for _ in 0..level {
          acc.push(CirruLexItem::Close);
        }
        acc.push(CirruLexItem::Close);
        return acc;
      }
    } else {
      let cursor = tokens[pointer].to_owned();
      match cursor {
        CirruLexItem::Str(s) => {
          acc.push(CirruLexItem::Str(s));
          pointer += 1;
        }
        CirruLexItem::Open => {
          acc.push(CirruLexItem::Open);
          pointer += 1;
        }
        CirruLexItem::Close => {
          acc.push(CirruLexItem::Close);
          pointer += 1;
        }
        CirruLexItem::Indent(n) => match n {
          _ if n > level => {
            let delta = n - level;
            for _ in 0..delta {
              acc.push(CirruLexItem::Open);
            }
            pointer += 1;
            level = n;
          }
          _ if n < level => {
            let delta = level - n;
            for _ in 0..delta {
              acc.push(CirruLexItem::Close);
            }
            acc.push(CirruLexItem::Close);
            acc.push(CirruLexItem::Open);
            pointer += 1;
            level = n;
          }
          _ => {
            if acc.is_empty() {
              acc = vec![];
            } else {
              acc.push(CirruLexItem::Close);
              acc.push(CirruLexItem::Open);
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
pub fn parse(code: &str) -> Result<Vec<Cirru>, String> {
  let tokens = resolve_indentations(lex(code)?);
  // println!("{:?}", tokens);
  let tree = build_exprs(tokens)?;
  // println!("tree {:?}", tree);
  Ok(resolve_comma(&resolve_dollar(&tree)))
}
