use crate::types::CirruNode;
use crate::types::CirruNode::*;
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
const ALLOWED_CHARS: &str = "-~_@#$&%!?^*=+|\\/<>[]{}.,:;'";

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

fn is_simple_expr(xs: CirruNode) -> bool {
  match xs {
    CirruList(ys) => {
      for y in ys {
        match y {
          CirruList(_) => return false,
          CirruLeaf(_) => (),
        }
      }
      true
    }
    CirruLeaf(_) => false,
  }
}

fn is_boxed(xs: CirruNode) -> bool {
  match xs {
    CirruList(ys) => {
      for y in ys {
        if let CirruLeaf(_) = y {
          return false;
        }
      }
      true
    }
    CirruLeaf(_) => false,
  }
}

fn is_simple_char(x: char) -> bool {
  is_a_digit(x) || is_a_letter(x)
}

fn is_char_allowed(x: char) -> bool {
  if is_simple_char(x) {
    return true;
  }
  ALLOWED_CHARS.find(x).is_some()
}

fn generate_leaf(s: String) -> String {
  let mut all_allowed = true;
  for x in s.chars() {
    if !is_char_allowed(x) {
      all_allowed = false;
      break;
    }
  }
  if all_allowed {
    s
  } else {
    format!(r#""{}""#, str::escape_debug(&s))
  }
}

fn generate_empty_expr() -> String {
  String::from("()")
}

fn generate_inline_expr(xs: Vec<CirruNode>) -> String {
  let mut result = String::from(CHAR_OPEN);

  for (idx, x) in xs.iter().enumerate() {
    if idx > 0 {
      result.push(' ');
    }
    let piece = match x {
      CirruLeaf(s) => generate_leaf(s.to_string()),
      CirruList(ys) => generate_inline_expr(ys.clone()),
    };
    result.push_str(&piece)
  }

  result.push_str(&CHAR_CLOSE.to_string());
  result
}

fn render_spaces(n: usize) -> String {
  let mut result = String::from("");
  for _ in 0..n {
    result.push_str("  ");
  }
  result
}

fn render_newline(n: usize) -> String {
  format!("\n{}", render_spaces(n))
}

/// options for writer, `use_inline` for more compact format.
#[derive(Clone, Copy)]
pub struct CirruWriterOptions {
  pub use_inline: bool,
}

fn get_node_kind(cursor: CirruNode) -> WriterNode {
  match cursor {
    CirruLeaf(_) => WriterNode::Leaf,
    CirruList(xs) => {
      if xs.is_empty() {
        WriterNode::Leaf
      } else if is_simple_expr(CirruList(xs.clone())) {
        WriterNode::SimpleExpr
      } else if is_boxed(CirruList(xs)) {
        WriterNode::BoxedExpr
      } else {
        WriterNode::Expr
      }
    }
  }
}

fn generate_tree(
  xs: Vec<CirruNode>,
  insist_head: bool,
  options: CirruWriterOptions,
  base_level: usize,
  in_tail: bool,
) -> String {
  let mut prev_kind = WriterNode::Nil;
  let mut level = base_level;
  let mut result = String::from("");

  for (idx, _cursor) in xs.iter().enumerate() {
    let cursor = _cursor.clone();
    let kind = get_node_kind(cursor.clone());
    let next_level = level + 1;
    let child_insist_head = (prev_kind == WriterNode::BoxedExpr) || (prev_kind == WriterNode::Expr);
    let at_tail = idx != 0 && !in_tail && prev_kind == WriterNode::Leaf && idx == xs.len() - 1;

    // println!("\nloop {:?} {:?}", prev_kind, kind);
    // println!("cursor {:?}", cursor);
    // println!("{}", str::escape_debug(&result));

    let child: String = match cursor {
      CirruLeaf(s) => generate_leaf(s),
      CirruList(ys) => {
        if at_tail {
          if ys.is_empty() {
            String::from("$")
          } else {
            format!("$ {}", generate_tree(ys, false, options, level, at_tail))
          }
        } else if idx == 0 && insist_head {
          generate_inline_expr(ys)
        } else if kind == WriterNode::Leaf {
          generate_empty_expr() // special since empty expr is treated as leaf
        } else if kind == WriterNode::SimpleExpr {
          if prev_kind == WriterNode::Leaf {
            generate_inline_expr(ys)
          } else if options.use_inline && prev_kind == WriterNode::SimpleExpr {
            format!(" {}", generate_inline_expr(ys))
          } else {
            format!(
              "{}{}",
              render_newline(next_level),
              &generate_tree(ys, child_insist_head, options, next_level, false,)
            )
          }
        } else if kind == WriterNode::Expr {
          let content = generate_tree(ys, child_insist_head, options, next_level, false);
          if content.starts_with('\n') {
            content
          } else {
            format!("{}{}", render_newline(next_level), content)
          }
        } else if kind == WriterNode::BoxedExpr {
          let content = generate_tree(ys, child_insist_head, options, next_level, false);
          if prev_kind == WriterNode::Nil
            || prev_kind == WriterNode::Leaf
            || prev_kind == WriterNode::SimpleExpr
          {
            content
          } else {
            format!("{}{}", render_newline(next_level), &content)
          }
        } else {
          unreachable!("Unpected condition")
        }
      }
    };

    let bended = kind == WriterNode::Leaf
      && (prev_kind == WriterNode::BoxedExpr || prev_kind == WriterNode::Expr);

    let chunk = if at_tail
      || (prev_kind == WriterNode::Leaf && kind == WriterNode::Leaf)
      || (prev_kind == WriterNode::Leaf && kind == WriterNode::SimpleExpr)
      || prev_kind == WriterNode::SimpleExpr && kind == WriterNode::Leaf
    {
      format!(" {}", child)
    } else if bended {
      format!("{}, {}", render_newline(next_level), child)
    } else {
      child
    };

    result = format!("{}{}", result, chunk);

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
  result
}

fn generate_statements(ys: Vec<CirruNode>, options: CirruWriterOptions) -> String {
  let mut zs = String::from("");
  for y in ys {
    match y {
      CirruLeaf(_) => unreachable!("expected an exprs at top level"),
      CirruList(cs) => {
        zs.push_str(&format!(
          "\n{}\n",
          generate_tree(cs, true, options, 0, false)
        ));
      }
    }
  }
  zs
}

/// format Cirru code, use options to control `use_inline` option
pub fn write_cirru(xs: CirruNode, options: CirruWriterOptions) -> String {
  match xs {
    CirruLeaf(_) => unreachable!("expected vector of exprs"),
    CirruList(ys) => generate_statements(ys, options),
  }
}
