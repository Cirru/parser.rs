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

const DEFAULT_EXPR_CAPACITY: usize = 8; // Added for default capacity

use std::cmp::Ordering::*;

use primes::CirruLexState;
use tree::{resolve_comma, resolve_dollar};

pub use primes::{Cirru, CirruLexItem, CirruLexItemList, escape_cirru_leaf};
pub use s_expr::format_to_lisp;
pub use writer::{CirruWriterOptions, format};

/// builds a tree from a flat list of tokens
fn build_exprs(tokens: &[CirruLexItem]) -> Result<Vec<Cirru>, String> {
  let mut acc: Vec<Cirru> = Vec::with_capacity(tokens.len() / 6 + 1);
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

    match &chunk {
      None => return Ok(acc),
      Some(ck) => {
        match ck {
          CirruLexItem::Open => {
            let mut pointer: Vec<Cirru> = Vec::with_capacity(DEFAULT_EXPR_CAPACITY);
            // guess a nested level of 16
            let mut pointer_stack: Vec<Vec<Cirru>> = Vec::with_capacity(16);
            loop {
              let cursor = pull_token();

              match &cursor {
                None => return Err(String::from("unexpected end of file")),
                Some(c) => match c {
                  CirruLexItem::Close => match pointer_stack.pop() {
                    None => {
                      acc.push(Cirru::List(pointer));
                      break;
                    }
                    Some(v) => {
                      let prev_p = pointer;
                      pointer = v;
                      pointer.push(Cirru::List(prev_p));
                    }
                  },
                  CirruLexItem::Open => {
                    pointer_stack.push(pointer);
                    pointer = Vec::with_capacity(DEFAULT_EXPR_CAPACITY);
                  }
                  CirruLexItem::Str(s) => pointer.push(Cirru::Leaf((**s).into())),
                  CirruLexItem::Indent(n) => return Err(format!("unknown indent: {n}")),
                },
              }
            }
          }
          CirruLexItem::Close => return Err(String::from("unexpected \")\"")),
          a => return Err(format!("unknown item: {a:?}")),
        }
      }
    }
  }
}

fn parse_indentation(size: u8) -> Result<CirruLexItem, String> {
  if size & 0x1 == 0x0 {
    // even number
    Ok(CirruLexItem::Indent(size >> 1))
  } else {
    Err(format!("odd indentation size, {size}"))
  }
}

const DEFAULT_BUFFER_CAPACITY: usize = 8;

/// The lexer for Cirru syntax. It scans the code and returns a flat list of tokens.
/// It uses a state machine to handle different parts of the syntax, such as strings,
/// tokens, and indentation.
pub fn lex(initial_code: &str) -> Result<CirruLexItemList, String> {
  // guessed an initial length
  let mut acc: CirruLexItemList = Vec::with_capacity(initial_code.len() >> 4);
  let mut state = CirruLexState::Indent;
  let mut buffer = String::with_capacity(DEFAULT_BUFFER_CAPACITY);
  let code = initial_code;

  for (idx, c) in code.char_indices() {
    match state {
      CirruLexState::Space => match c {
        ' ' => {
          state = CirruLexState::Space;
          buffer.clear();
        }
        '\n' => {
          state = CirruLexState::Indent;
          buffer.clear();
        }
        '(' => {
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer = String::new()
        }
        ')' => {
          acc.push(CirruLexItem::Close);
          state = CirruLexState::Space;
          buffer.clear()
        }
        '"' => {
          state = CirruLexState::Str;
          buffer.clear();
        }
        _ => {
          state = CirruLexState::Token;
          buffer.clear();
          buffer.push(c);
        }
      },
      CirruLexState::Token => match c {
        ' ' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Space;
          buffer = String::with_capacity(DEFAULT_BUFFER_CAPACITY);
        }
        '"' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Str;
          buffer = String::with_capacity(DEFAULT_BUFFER_CAPACITY);
        }
        '\n' => {
          acc.push(CirruLexItem::Str(buffer));
          state = CirruLexState::Indent;
          buffer = String::with_capacity(DEFAULT_BUFFER_CAPACITY);
        }
        '(' => {
          acc.push(CirruLexItem::Str(buffer));
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer = String::new()
        }
        ')' => {
          acc.push(CirruLexItem::Str(buffer));
          acc.push(CirruLexItem::Close);
          state = CirruLexState::Space;
          buffer = String::new()
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
          buffer = String::with_capacity(DEFAULT_BUFFER_CAPACITY);
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
          println!("Unicode escaping is not supported yet: {peek:?} ...");
          buffer.push_str("\\u");
          state = CirruLexState::Str;
        }
        '\\' => {
          state = CirruLexState::Str;
          buffer.push('\\');
        }
        _ => return Err(format!("unexpected character during string escaping: {c:?}")),
      },
      CirruLexState::Indent => match c {
        ' ' => {
          state = CirruLexState::Indent;
          buffer.push(c);
        }
        '\n' => {
          state = CirruLexState::Indent;
          buffer.clear();
        }
        '"' => {
          let level = parse_indentation(buffer.len() as u8)?;
          acc.push(level);
          state = CirruLexState::Str;
          buffer = String::new();
        }
        '(' => {
          let level = parse_indentation(buffer.len() as u8)?;
          acc.push(level);
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer.clear();
        }
        ')' => return Err(String::from("unexpected ) at line start")),
        _ => {
          let level = parse_indentation(buffer.len() as u8)?;
          acc.push(level);
          state = CirruLexState::Token;
          buffer.clear();
          buffer.push(c);
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

/// This function transforms a flat list of tokens into a tree structure
/// by handling indentation. It inserts `Open` and `Close` tokens based on
/// changes in indentation levels.
///
/// # Examples
///
/// ```
/// # use cirru_parser::{CirruLexItem, resolve_indentations};
/// # use cirru_parser::CirruLexItem::*;
/// let tokens = vec![Indent(0), "a".into(), Indent(1), "b".into()];
/// let resolved = resolve_indentations(&tokens);
/// assert_eq!(resolved, vec![Open, "a".into(), Open, "b".into(), Close, Close]);
/// ```
pub fn resolve_indentations(tokens: &[CirruLexItem]) -> CirruLexItemList {
  let mut acc: CirruLexItemList = Vec::with_capacity(tokens.len() * 2);
  let mut level: u8 = 0;

  if tokens.is_empty() {
    return vec![];
  }

  for token in tokens {
    match token {
      CirruLexItem::Str(s) => {
        acc.push(CirruLexItem::Str(s.to_owned()));
      }
      CirruLexItem::Open => {
        acc.push(CirruLexItem::Open);
      }
      CirruLexItem::Close => {
        acc.push(CirruLexItem::Close);
      }
      CirruLexItem::Indent(n) => {
        match n.cmp(&level) {
          Greater => {
            // Indent level increased, push open parenthesis.
            for _ in 0..(n - level) {
              acc.push(CirruLexItem::Open);
            }
          }
          Less => {
            // Indent level decreased, push close parenthesis.
            for _ in 0..(level - n) {
              acc.push(CirruLexItem::Close);
            }
            acc.push(CirruLexItem::Close);
            acc.push(CirruLexItem::Open);
          }
          Equal => {
            // Same indent level, close previous expression and start a new one.
            if !acc.is_empty() {
              acc.push(CirruLexItem::Close);
              acc.push(CirruLexItem::Open);
            }
          }
        }
        level = *n;
      }
    }
  }

  // Close all remaining parenthesis.
  if !acc.is_empty() {
    let mut new_acc = Vec::with_capacity(1 + acc.len() + level as usize + 1);
    new_acc.push(CirruLexItem::Open);
    new_acc.append(&mut acc); // acc is drained

    for _ in 0..level {
      new_acc.push(CirruLexItem::Close);
    }
    new_acc.push(CirruLexItem::Close);
    new_acc
  } else {
    vec![]
  }
}

/// Parses a string of Cirru code into a tree of `Cirru` expressions.
///
/// This is the main entry point for the parser. It performs the following steps:
/// 1. Lexing: The code is tokenized into a flat list of `CirruLexItem`s.
/// 2. Indentation Resolution: The flat list of tokens is transformed into a tree
///    structure by handling indentation.
/// 3. Tree Building: The token tree is converted into a tree of `Cirru` expressions.
/// 4. Syntax Resolution: Special syntax like `$` and `,` is resolved.
///
/// # Examples
///
/// ```
/// # use cirru_parser::{parse, Cirru};
/// let code = "defn main\n  println \"Hello, world!\"";
/// let expected = Ok(vec![
///   Cirru::List(vec![
///     Cirru::Leaf("defn".into()),
///     Cirru::Leaf("main".into()),
///     Cirru::List(vec![
///       Cirru::Leaf("println".into()),
///       Cirru::Leaf("Hello, world!".into()),
///     ]),
///   ]),
/// ]);
/// assert_eq!(parse(code), expected);
/// ```
pub fn parse(code: &str) -> Result<Vec<Cirru>, String> {
  let tokens = resolve_indentations(&lex(code)?);
  // println!("{:?}", tokens);
  let mut tree = build_exprs(&tokens)?;
  // println!("tree {:?}", tree);
  resolve_dollar(&mut tree);
  resolve_comma(&mut tree);
  Ok(tree)
}

/// Converts a string of Cirru code directly to a Lisp-like string.
///
/// This function is a convenience wrapper around `parse` and `format_to_lisp`.
/// It will panic if parsing or formatting fails.
pub fn cirru_to_lisp(code: &str) -> String {
  match parse(code) {
    Ok(tree) => match format_to_lisp(&tree) {
      Ok(s) => s,
      Err(_) => panic!("failed to convert to lisp"),
    },
    Err(_) => panic!("expected a leaf"),
  }
}