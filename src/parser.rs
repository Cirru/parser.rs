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

mod error;
mod primes;
mod s_expr;
mod tree;
mod writer;

#[cfg(feature = "serde-json")]
mod json;

pub use error::{CirruError, CirruErrorKind, ErrorContext, SourcePos};

#[cfg(feature = "serde-json")]
pub use json::*;

const DEFAULT_EXPR_CAPACITY: usize = 8; // Added for default capacity

use std::cmp::Ordering::*;

use primes::CirruLexState;
use tree::{resolve_comma, resolve_dollar};

pub use primes::{Cirru, CirruLexItem, CirruLexItemList, escape_cirru_leaf};
pub use s_expr::format_to_lisp;
pub use writer::{CirruOneLinerExt, CirruWriterOptions, format, format_expr_one_liner};

/// Helper function to format and print a detailed error
pub fn print_error(error: &CirruError, source_code: Option<&str>) {
  eprintln!("{}", error.format_detailed(source_code));
}

/// Extension trait for method-style parsing of a one-line Cirru expression.
pub trait CirruOneLinerParseExt {
  fn parse_expr_one_liner(&self) -> Result<Cirru, String>;
}

impl CirruOneLinerParseExt for str {
  fn parse_expr_one_liner(&self) -> Result<Cirru, String> {
    parse_expr_one_liner(self).map_err(|e| e.to_string())
  }
}

/// builds a tree from a flat list of tokens
fn build_exprs(tokens: &[CirruLexItem]) -> Result<Vec<Cirru>, CirruError> {
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
                None => {
                  return Err(CirruError::new(CirruErrorKind::UnexpectedEof));
                }
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
                  CirruLexItem::Indent(n) => {
                    return Err(CirruError::new(CirruErrorKind::Other(format!("unknown indent: {n}"))));
                  }
                },
              }
            }
          }
          CirruLexItem::Close => {
            return Err(CirruError::new(CirruErrorKind::UnexpectedCloseParen));
          }
          a => {
            return Err(CirruError::new(CirruErrorKind::Other(format!("unknown item: {a:?}"))));
          }
        }
      }
    }
  }
}

fn parse_indentation(size: u8, ctx: &LexerContext, code: &str) -> Result<CirruLexItem, CirruError> {
  if size & 0x1 == 0x0 {
    // even number
    Ok(CirruLexItem::Indent(size >> 1))
  } else {
    let pos = ctx.current_pos();
    let snippet = ctx.get_context_snippet(code, 20);
    let error_ctx = ErrorContext::new(pos, Some(snippet), "checking indentation".to_string());
    Err(CirruError::with_context(CirruErrorKind::InvalidIndentation(size), error_ctx))
  }
}

const DEFAULT_BUFFER_CAPACITY: usize = 8;

/// Position tracker for lexical analysis
struct LexerContext {
  line: usize,
  column: usize,
  offset: usize,
}

impl LexerContext {
  fn new() -> Self {
    Self {
      line: 1,
      column: 1,
      offset: 0,
    }
  }

  fn current_pos(&self) -> SourcePos {
    SourcePos::new(self.line, self.column, self.offset)
  }

  fn advance(&mut self, c: char) {
    self.offset += c.len_utf8();
    if c == '\n' {
      self.line += 1;
      self.column = 1;
    } else {
      self.column += 1;
    }
  }

  fn get_context_snippet(&self, code: &str, window: usize) -> String {
    let start = self.offset.saturating_sub(window);
    let end = (self.offset + window).min(code.len());
    let snippet = &code[start..end];
    // Use escape_debug to show special characters like \n, \t, spaces clearly
    let escaped: String = snippet.chars().take(60).flat_map(|c| c.escape_debug()).collect();
    format!("...{escaped}...")
  }
}

/// The lexer for Cirru syntax. It scans the code and returns a flat list of tokens.
/// It uses a state machine to handle different parts of the syntax, such as strings,
/// tokens, and indentation.
pub fn lex(initial_code: &str) -> Result<CirruLexItemList, CirruError> {
  // guessed an initial length
  let mut acc: CirruLexItemList = Vec::with_capacity(initial_code.len() >> 4);
  let mut state = CirruLexState::Indent;
  let mut buffer = String::with_capacity(DEFAULT_BUFFER_CAPACITY);
  let code = initial_code;
  let mut ctx = LexerContext::new();

  for (_idx, c) in code.char_indices() {
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
          let pos = ctx.current_pos();
          let snippet = ctx.get_context_snippet(code, 20);
          let error_ctx = ErrorContext::new(pos, Some(snippet), "in string literal".to_string());
          return Err(CirruError::with_context(CirruErrorKind::UnexpectedNewlineInString, error_ctx));
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
          // Unicode escaping: not fully supported
          let pos = ctx.current_pos();
          let snippet = ctx.get_context_snippet(code, 20);
          let error_ctx = ErrorContext::new(pos, Some(snippet), "in escape sequence".to_string());
          return Err(CirruError::with_context(
            CirruErrorKind::Other("Unicode escape sequences (\\u) are not supported".to_string()),
            error_ctx,
          ));
        }
        '\\' => {
          state = CirruLexState::Str;
          buffer.push('\\');
        }
        _ => {
          let pos = ctx.current_pos();
          let snippet = ctx.get_context_snippet(code, 20);
          let error_ctx = ErrorContext::new(pos, Some(snippet), "invalid escape sequence in string".to_string());
          return Err(CirruError::with_context(CirruErrorKind::InvalidEscape(c), error_ctx));
        }
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
          let level = parse_indentation(buffer.len() as u8, &ctx, code)?;
          acc.push(level);
          state = CirruLexState::Str;
          buffer = String::new();
        }
        '(' => {
          let level = parse_indentation(buffer.len() as u8, &ctx, code)?;
          acc.push(level);
          acc.push(CirruLexItem::Open);
          state = CirruLexState::Space;
          buffer.clear();
        }
        ')' => {
          let pos = ctx.current_pos();
          let snippet = ctx.get_context_snippet(code, 20);
          let error_ctx = ErrorContext::new(pos, Some(snippet), "at line start".to_string());
          return Err(CirruError::with_context(CirruErrorKind::UnexpectedCloseParen, error_ctx));
        }
        _ => {
          let level = parse_indentation(buffer.len() as u8, &ctx, code)?;
          acc.push(level);
          state = CirruLexState::Token;
          buffer.clear();
          buffer.push(c);
        }
      },
    }

    ctx.advance(c);
  }

  match state {
    CirruLexState::Space => Ok(acc),
    CirruLexState::Token => {
      acc.push(CirruLexItem::Str(buffer));
      Ok(acc)
    }
    CirruLexState::Escape => {
      let pos = ctx.current_pos();
      let error_ctx = ErrorContext::new(pos, None, "at end of file".to_string());
      Err(CirruError::with_context(
        CirruErrorKind::Other("incomplete escape sequence".to_string()),
        error_ctx,
      ))
    }
    CirruLexState::Indent => Ok(acc),
    CirruLexState::Str => {
      let pos = ctx.current_pos();
      let error_ctx = ErrorContext::new(pos, None, "unclosed string literal".to_string());
      Err(CirruError::with_context(CirruErrorKind::UnexpectedEof, error_ctx))
    }
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
/// let tree = parse(code).unwrap();
/// let expected = vec![
///   Cirru::List(vec![
///     Cirru::Leaf("defn".into()),
///     Cirru::Leaf("main".into()),
///     Cirru::List(vec![
///       Cirru::Leaf("println".into()),
///       Cirru::Leaf("Hello, world!".into()),
///     ]),
///   ]),
/// ];
/// assert_eq!(tree, expected);
/// ```
pub fn parse(code: &str) -> Result<Vec<Cirru>, CirruError> {
  let tokens = lex(code)?;
  let tokens = resolve_indentations(&tokens);
  // println!("{:?}", tokens);
  let mut tree = build_exprs(&tokens)?;
  // println!("tree {:?}", tree);
  resolve_dollar(&mut tree);
  resolve_comma(&mut tree);
  Ok(tree)
}

/// Backward compatibility function that returns Result with String error
#[deprecated(since = "0.2.0", note = "Use parse() instead which provides better error information")]
pub fn parse_compat(code: &str) -> Result<Vec<Cirru>, String> {
  parse(code).map_err(|e| e.to_string())
}

/// Backward compatibility function for lex that returns tokens with String error
#[deprecated(since = "0.2.0", note = "Use lex() instead which provides better error information")]
pub fn lex_simple(code: &str) -> Result<CirruLexItemList, String> {
  lex(code).map_err(|e| e.to_string())
}

/// Parses a one-line Cirru expression into exactly one `Cirru` expression.
///
/// This is a convenience wrapper over `parse` that enforces there is exactly one
/// top-level expression.
pub fn parse_expr_one_liner(code: &str) -> Result<Cirru, CirruError> {
  let xs = parse(code)?;
  if xs.len() != 1 {
    return Err(CirruError::new(CirruErrorKind::WrongExprCount {
      expected: 1,
      got: xs.len(),
    }));
  }
  Ok(xs.into_iter().next().expect("len checked"))
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
