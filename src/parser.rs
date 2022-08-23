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

mod primes;
mod s_expr;
mod tree;
mod writer;

#[cfg(feature = "use-serde")]
mod json;

#[cfg(feature = "use-serde")]
pub use json::*;

use std::cmp::Ordering::*;

use primes::CirruLexState;
use tree::{resolve_comma, resolve_dollar};

pub use primes::{escape_cirru_leaf, Cirru, CirruLexItem, CirruLexItemList};
pub use s_expr::format_to_lisp;
pub use writer::{format, CirruWriterOptions};

fn build_exprs(tokens: &[CirruLexItem]) -> Result<Vec<Cirru>, String> {
  let mut acc: Vec<Cirru> = vec![];
  let mut idx = 0;
  let mut pull_token = || {
    if idx >= tokens.len() {
      return None;
    }
    let pos = idx;
    idx += 1;
    Some(&tokens[pos])
  };
  loop {
    let chunk = pull_token();

    if chunk == None {
      return Ok(acc);
    }
    match chunk.unwrap() {
      CirruLexItem::Open => {
        let mut pointer: Vec<Cirru> = vec![];
        // guess a nested level of 16
        let mut pointer_stack: Vec<Vec<Cirru>> = Vec::with_capacity(16);
        loop {
          let cursor = pull_token();
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
            CirruLexItem::Str(s) => pointer.push(Cirru::Leaf(s.as_str().into())),
            CirruLexItem::Indent(n) => return Err(format!("unknown indent: {}", n)),
          }
        }
      }
      CirruLexItem::Close => return Err(String::from("unexpected \")\"")),
      a => return Err(format!("unknown item: {:?}", a)),
    }
  }
}

fn parse_indentation(size: u8) -> Result<CirruLexItem, String> {
  if size & 0x1 == 0x0 {
    // even number
    Ok(CirruLexItem::Indent(size >> 1))
  } else {
    Err(format!("odd indentation size, {}", size))
  }
}

/// internal function for lexing
pub fn lex(initial_code: &str) -> Result<CirruLexItemList, String> {
  // guessed an initial length
  let mut acc: CirruLexItemList = Vec::with_capacity(initial_code.len() >> 4);
  let mut state = CirruLexState::Indent;
  let mut buffer = String::from("");
  let code = initial_code;

  for (idx, c) in code.chars().enumerate() {
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
        '\'' => {
          state = CirruLexState::Str;
          buffer.push('\'');
        }
        't' => {
          state = CirruLexState::Str;
          buffer.push('\t');
        }
        'n' => {
          state = CirruLexState::Str;
          buffer.push('\n');
        }
        'r' => {
          state = CirruLexState::Str;
          buffer.push('\r');
        }
        'u' => {
          // not supporting, but don't panic
          let end = idx + 10;
          let peek = if end >= code.len() { &code[idx..] } else { &code[idx..end] };
          println!("Unicode escaping is not supported yet: {:?} ...", peek);
          buffer.push_str("\\u");
          state = CirruLexState::Str;
        }
        '\\' => {
          state = CirruLexState::Str;
          buffer.push('\\');
        }
        _ => return Err(format!("unexpected character during string escaping: {:?}", c)),
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
          let level = parse_indentation(buffer.len() as u8)?;
          acc.push(level);
          state = CirruLexState::Str;
          buffer = String::from("");
        }
        '(' => {
          let level = parse_indentation(buffer.len() as u8)?;
          acc.push(level);
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer = String::from("");
        }
        ')' => return Err(String::from("unexpected ) at line start")),
        _ => {
          let level = parse_indentation(buffer.len() as u8)?;
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
pub fn resolve_indentations(tokens: &[CirruLexItem]) -> CirruLexItemList {
  let mut acc: CirruLexItemList = Vec::with_capacity(tokens.len() >> 1);
  let mut level = 0;
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
      match &tokens[pointer] {
        CirruLexItem::Str(s) => {
          acc.push(CirruLexItem::Str(s.to_owned()));
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
        CirruLexItem::Indent(n) => match n.cmp(&level) {
          Greater => {
            let delta = n - level;
            for _ in 0..delta {
              acc.push(CirruLexItem::Open);
            }
            pointer += 1;
            level = *n;
          }
          Less => {
            let delta = level - n;
            for _ in 0..delta {
              acc.push(CirruLexItem::Close);
            }
            acc.push(CirruLexItem::Close);
            acc.push(CirruLexItem::Open);
            pointer += 1;
            level = *n;
          }
          Equal => {
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
  let tokens = resolve_indentations(&lex(code)?);
  // println!("{:?}", tokens);
  let tree = build_exprs(&tokens)?;
  // println!("tree {:?}", tree);
  Ok(resolve_comma(&resolve_dollar(&tree)))
}

pub fn cirru_to_lisp(code: &str) -> String {
  match parse(code) {
    Ok(tree) => match format_to_lisp(&tree) {
      Ok(s) => s,
      Err(_) => panic!("failed to convert to lisp"),
    },
    Err(_) => panic!("expected a leaf"),
  }
}
