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

/// Format a Cirru leaf token: returns the bare string if all characters are allowed
/// in Cirru without quoting; otherwise wraps in double quotes with escape sequences.
/// This mirrors the exact quoting behaviour used by the Cirru formatter when emitting
/// leaf nodes, so callers outside this crate get consistent output.
pub fn generate_leaf(s: &str) -> String {
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

fn generate_statement_one_liner(xs: &[Cirru]) -> String {
  let mut ret = String::new();
  let len = xs.len();
  for (idx, cursor) in xs.iter().enumerate() {
    if idx > 0 {
      ret.push(' ');
    }
    let at_tail = idx == len - 1 && idx > 0;
    match cursor {
      Cirru::Leaf(s) => ret.push_str(&generate_leaf(s)),
      Cirru::List(ys) => {
        if at_tail {
          // Use $ syntax for tail expressions
          if ys.is_empty() {
            ret.push('$');
          } else {
            ret.push_str("$ ");
            ret.push_str(&generate_statement_one_liner(ys));
          }
        } else {
          ret.push_str(&generate_inline_expr(ys));
        }
      }
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
    let child_insist_head = (prev_kind == WriterNode::BoxedExpr) || (prev_kind == WriterNode::Expr) || idx > 1;
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
            let content = generate_tree(ys, false, options, level, at_tail)?;
            if content.starts_with('\n') {
              // If content starts with newline, don't add space after $
              let mut ret = String::from("$");
              ret.push_str(&content);
              ret
            } else {
              // Otherwise, add space after $
              let mut ret = String::from("$ ");
              ret.push_str(&content);
              ret
            }
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
          if child_insist_head {
            // special case for boxed expr when it insists head, it has both indentation and brackets
            let mut ret = render_newline(next_level);
            ret.push_str(&content);
            ret
          } else if prev_kind == WriterNode::Nil || prev_kind == WriterNode::Leaf || prev_kind == WriterNode::SimpleExpr {
            content
          } else {
            let mut ret = render_newline(next_level);
            ret.push_str(&content);
            ret
          }
        } else {
          return Err(String::from("Unexpected condition"));
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
pub fn format_expr_one_liner(expr: &Cirru) -> Result<String, String> {
  match expr {
    Cirru::Leaf(_) => Err(String::from("format_expr_one_liner expects an expr (list)")),
    Cirru::List(cs) => Ok(generate_statement_one_liner(cs)),
  }
}

/// Extension trait for method-style one-liner formatting.
pub trait CirruOneLinerExt {
  fn format_one_liner(&self) -> Result<String, String>;
}

impl CirruOneLinerExt for Cirru {
  fn format_one_liner(&self) -> Result<String, String> {
    format_expr_one_liner(self)
  }
}

// ── structural folding for error display ──────────────────────────────────────

/// Max siblings to show on each side of the error target.
const FOLD_SIBLINGS_HALF: usize = 1;

/// Max children to show in non-target subtrees.
const FOLD_CHILDREN_NON_TARGET: usize = 2;

/// Max children to show in the error-target subtree.
const FOLD_CHILDREN_TARGET: usize = 4;

/// Max depth for non-target subtrees before collapsing to `|…`.
const FOLD_NON_TARGET_DEPTH: usize = 2;

/// Max depth for the error-target subtree before collapsing to `|…`.
const FOLD_TARGET_DEPTH: usize = 3;

/// Build a `(folded "|…description")` placeholder node.
fn folded_place(label: &str) -> Cirru {
  Cirru::List(vec![Cirru::leaf("folded"), Cirru::leaf(label)])
}

/// Structurally focus on a path within a Cirru tree, folding away irrelevant branches.
///
/// Walks the tree via `path`, keeping at most `FOLD_SIBLINGS_HALF` siblings
/// on each side of the target at every nesting level.  Non-target subtrees
/// are capped at `FOLD_NON_TARGET_DEPTH`; the focus path is capped at
/// `FOLD_TARGET_DEPTH`.  Hidden branches are replaced with `(folded "|…")` nodes.
///
/// This is a general-purpose display utility — not tied to errors.
///
/// # Example
///
/// ```rust
/// use cirru_parser::Cirru;
/// use cirru_parser::focus_cirru_preview;
///
/// let tree = Cirru::List(vec![
///   Cirru::leaf("a"),
///   Cirru::List(vec![
///     Cirru::leaf("b"),
///     Cirru::leaf("c"),
///     Cirru::leaf("d"),
///   ]),
///   Cirru::leaf("e"),
/// ]);
/// let focused = focus_cirru_preview(&tree, &[1, 2]);
/// // Shows sibling "a", target "(b c d)" with child "d" highlighted,
/// // sibling "e", hiding rest.
/// ```
pub fn focus_cirru_preview(node: &Cirru, path: &[usize]) -> Cirru {
  focus_cirru_preview_impl(node, path, 0)
}

fn focus_cirru_preview_impl(node: &Cirru, path: &[usize], depth: usize) -> Cirru {
  match node {
    Cirru::Leaf(_) => node.clone(),
    Cirru::List(xs) => {
      if path.is_empty() {
        return fold_children_impl(node, FOLD_CHILDREN_TARGET, 0, FOLD_TARGET_DEPTH);
      }

      let target_idx = path[0];
      if target_idx >= xs.len() {
        return fold_children_impl(node, FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH);
      }

      let start = target_idx.saturating_sub(FOLD_SIBLINGS_HALF);
      let end = (target_idx + FOLD_SIBLINGS_HALF + 1).min(xs.len());
      let rest_path = &path[1..];

      let mut result: Vec<Cirru> = Vec::new();

      if start > 0 {
        result.push(folded_place(&format!("|…+{} nodes before", start)));
      }

      for i in start..end {
        if i == target_idx {
          result.push(focus_cirru_preview_impl(&xs[i], rest_path, depth + 1));
        } else {
          result.push(fold_children_impl(&xs[i], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
        }
      }

      let remaining = xs.len() - end;
      if remaining > 0 {
        result.push(folded_place(&format!("|…+{} nodes after", remaining)));
      }

      Cirru::List(result)
    }
  }
}

fn fold_children_impl(node: &Cirru, max_children: usize, depth: usize, max_depth: usize) -> Cirru {
  if depth >= max_depth {
    return folded_place("|… max depth");
  }
  match node {
    Cirru::Leaf(_) => node.clone(),
    Cirru::List(xs) => {
      if xs.len() <= max_children {
        let children: Vec<Cirru> = xs.iter().map(|c| fold_children_impl(c, max_children, depth + 1, max_depth)).collect();
        return Cirru::List(children);
      }
      let mut result: Vec<Cirru> = Vec::new();
      for c in xs.iter().take(max_children) {
        result.push(fold_children_impl(c, max_children, depth + 1, max_depth));
      }
      result.push(folded_place(&format!("|…+{} nodes inside", xs.len() - max_children)));
      Cirru::List(result)
    }
  }
}
