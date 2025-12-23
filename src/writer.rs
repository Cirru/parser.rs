use crate::primes::Cirru;
use std::fmt;
use std::str;

#[derive(PartialEq, Clone, Copy, fmt::Debug)]
enum WriterNode {
  Nil,
  Leaf,
  SimpleExpr,
  BoxedExpr,
  Expr,
}

const CHAR_CLOSE: char = ')';
const CHAR_OPEN: char = '(';
const ALLOWED_CHARS: &str = "$-:<>[]{}*=+.,\\/!?~_@#&%^|;'";

fn is_a_digit(c: char) -> bool {
  let n = c as usize;
  // ascii table https://tool.oschina.net/commons?type=4
  (48..=57).contains(&n)
}

fn is_a_letter(c: char) -> bool {
  let n = c as usize;
  if (65..=90).contains(&n) {
    return true;
  }
  if (97..=122).contains(&n) {
    return true;
  }
  false
}

fn is_simple_expr(ys: &[Cirru]) -> bool {
  for y in ys {
    match y {
      Cirru::List(_) => return false,
      Cirru::Leaf(_) => (),
    }
  }
  true
}

fn is_boxed(ys: &[Cirru]) -> bool {
  for y in ys {
    if let Cirru::Leaf(_) = y {
      return false;
    }
  }
  true
}

fn is_simple_char(x: char) -> bool {
  is_a_letter(x) || is_a_digit(x)
}

fn is_char_allowed(x: char) -> bool {
  if is_simple_char(x) {
    return true;
  }
  ALLOWED_CHARS.find(x).is_some()
}

fn generate_leaf(s: &str) -> String {
  let mut all_allowed = true;
  for x in s.chars() {
    if !is_char_allowed(x) {
      all_allowed = false;
      break;
    }
  }
  if all_allowed {
    s.to_string()
  } else {
    let mut ret = String::with_capacity(s.len() + 2);
    ret.push('"');
    for c in s.chars() {
      match c {
        '\n' => ret.push_str("\\n"),
        '\t' => ret.push_str("\\t"),
        '\"' => ret.push_str("\\\""),
        '\\' => ret.push_str("\\\\"),
        '\'' => ret.push_str("\\'"),
        _ => ret.push(c),
      }
    }
    ret.push('"');
    ret
  }
}

fn generate_empty_expr() -> String {
  String::from("()")
}

fn generate_inline_expr(xs: &[Cirru]) -> String {
  let mut result = String::from(CHAR_OPEN);

  for (idx, x) in xs.iter().enumerate() {
    if idx > 0 {
      result.push(' ');
    }
    let piece = match x {
      Cirru::Leaf(s) => generate_leaf(s),
      Cirru::List(ys) => generate_inline_expr(ys),
    };
    result.push_str(&piece)
  }

  result.push(CHAR_CLOSE);
  result
}

/// by 2 spaces
fn push_spaces(buf: &mut String, n: usize) {
  for _ in 0..n {
    buf.push_str("  ");
  }
}

fn render_newline(n: usize) -> String {
  let mut ret = String::with_capacity(n * 2);
  ret.push('\n');
  push_spaces(&mut ret, n);
  ret
}

fn generate_statement_oneliner(xs: &[Cirru]) -> String {
  let mut ret = String::new();
  for (idx, cursor) in xs.iter().enumerate() {
    if idx > 0 {
      ret.push(' ');
    }
    match cursor {
      Cirru::Leaf(s) => ret.push_str(&generate_leaf(s)),
      Cirru::List(ys) => ret.push_str(&generate_inline_expr(ys)),
    }
  }
  ret
}

/// options for writer, `use_inline` for more compact format.
#[derive(Clone, Copy)]
pub struct CirruWriterOptions {
  pub use_inline: bool,
}

impl From<bool> for CirruWriterOptions {
  fn from(use_inline: bool) -> Self {
    CirruWriterOptions { use_inline }
  }
}

fn get_node_kind(cursor: &Cirru) -> WriterNode {
  match cursor {
    Cirru::Leaf(_) => WriterNode::Leaf,
    Cirru::List(xs) => {
      if xs.is_empty() {
        WriterNode::Leaf
      } else if is_simple_expr(xs) {
        WriterNode::SimpleExpr
      } else if is_boxed(xs) {
        WriterNode::BoxedExpr
      } else {
        WriterNode::Expr
      }
    }
  }
}

fn generate_tree(
  xs: &[Cirru],
  insist_head: bool,
  options: CirruWriterOptions,
  base_level: usize,
  in_tail: bool,
) -> Result<String, String> {
  let mut prev_kind = WriterNode::Nil;
  let mut level = base_level;
  let mut result = String::from("");

  for (idx, cursor) in xs.iter().enumerate() {
    let kind = get_node_kind(cursor);
    let next_level = level + 1;
    let child_insist_head = (prev_kind == WriterNode::BoxedExpr) || (prev_kind == WriterNode::Expr);
    let at_tail = idx != 0 && !in_tail && prev_kind == WriterNode::Leaf && idx == xs.len() - 1;

    // println!("\nloop {:?} {:?}", prev_kind, kind);
    // println!("cursor {:?} {} {}", cursor, idx, insist_head);
    // println!("{:?}", result);

    let child: String = match cursor {
      Cirru::Leaf(s) => generate_leaf(s),
      Cirru::List(ys) => {
        if at_tail {
          if ys.is_empty() {
            String::from("$")
          } else {
            let mut ret = String::from("$ ");
            ret.push_str(&generate_tree(ys, false, options, level, at_tail)?);
            ret
          }
        } else if idx == 0 && insist_head {
          generate_inline_expr(ys)
        } else if kind == WriterNode::Leaf {
          if idx == 0 {
            let mut ret = render_newline(level);
            ret.push_str(&generate_empty_expr());
            ret
          } else {
            generate_empty_expr() // special since empty expr is treated as leaf
          }
        } else if kind == WriterNode::SimpleExpr {
          if prev_kind == WriterNode::Leaf {
            generate_inline_expr(ys)
          } else if options.use_inline && prev_kind == WriterNode::SimpleExpr {
            let mut ret = String::from(" ");
            ret.push_str(&generate_inline_expr(ys));
            ret
          } else {
            let mut ret = render_newline(next_level);
            ret.push_str(&generate_tree(ys, child_insist_head, options, next_level, false)?);
            ret
          }
        } else if kind == WriterNode::Expr {
          let content = generate_tree(ys, child_insist_head, options, next_level, false)?;
          if content.starts_with('\n') {
            content
          } else {
            let mut ret = render_newline(next_level);
            ret.push_str(&content);
            ret
          }
        } else if kind == WriterNode::BoxedExpr {
          let content = generate_tree(ys, child_insist_head, options, next_level, false)?;
          if prev_kind == WriterNode::Nil || prev_kind == WriterNode::Leaf || prev_kind == WriterNode::SimpleExpr {
            content
          } else {
            let mut ret = render_newline(next_level);
            ret.push_str(&content);
            ret
          }
        } else {
          return Err(String::from("Unpected condition"));
        }
      }
    };

    let bended = kind == WriterNode::Leaf && (prev_kind == WriterNode::BoxedExpr || prev_kind == WriterNode::Expr);

    let chunk = if at_tail
      || (prev_kind == WriterNode::Leaf && kind == WriterNode::Leaf)
      || (prev_kind == WriterNode::Leaf && kind == WriterNode::SimpleExpr)
      || prev_kind == WriterNode::SimpleExpr && kind == WriterNode::Leaf
    {
      let mut ret = String::from(" ");
      ret.push_str(&child);
      ret
    } else if bended {
      let mut ret = render_newline(next_level);
      ret.push_str(", ");
      ret.push_str(&child);
      ret
    } else {
      child
    };

    result.push_str(&chunk);

    // update writer states

    if kind == WriterNode::SimpleExpr {
      if idx == 0 && insist_head {
        prev_kind = WriterNode::SimpleExpr;
      } else if options.use_inline {
        if prev_kind == WriterNode::Leaf || prev_kind == WriterNode::SimpleExpr {
          prev_kind = WriterNode::SimpleExpr;
        } else {
          prev_kind = WriterNode::Expr;
        }
      } else if prev_kind == WriterNode::Leaf {
        prev_kind = WriterNode::SimpleExpr;
      } else {
        prev_kind = WriterNode::Expr;
      }
    } else {
      prev_kind = kind;
    }

    if bended {
      level += 1;
    }

    // console.log("chunk", JSON.stringify(chunk));
    // console.log("And result", JSON.stringify(result));
  }
  Ok(result)
}

fn generate_statements(ys: &[Cirru], options: CirruWriterOptions) -> Result<String, String> {
  let mut zs = String::from("");
  for y in ys {
    match y {
      Cirru::Leaf(_) => return Err(String::from("expected an exprs at top level")),
      Cirru::List(cs) => {
        zs.push('\n');
        zs.push_str(&generate_tree(cs, true, options, 0, false)?);
        zs.push('\n');
      }
    }
  }
  Ok(zs)
}

/// format Cirru code, use options to control `use_inline` option
pub fn format(xs: &[Cirru], options: CirruWriterOptions) -> Result<String, String> {
  generate_statements(xs, options)
}

/// Format a single Cirru expression as a single line without newlines or indentation.
///
/// Note: the top-level expression (a `Cirru::List`) is rendered without wrapping parentheses,
/// while nested expressions are still rendered with parentheses.
pub fn format_expr_oneliner(expr: &Cirru) -> Result<String, String> {
  match expr {
    Cirru::Leaf(_) => Err(String::from("format_expr_oneliner expects an expr (list)")),
    Cirru::List(cs) => Ok(generate_statement_oneliner(cs)),
  }
}
