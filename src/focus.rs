//! Structural focusing for Cirru tree previews.
//!
//! Provides `focus_cirru_preview`, which walks a Cirru tree along a path and
//! folds away irrelevant branches, keeping only the target path and nearby
//! siblings.  Hidden subtrees are replaced with `(folded "|…")` placeholder
//! nodes that carry a human-readable description of what was collapsed.

use crate::primes::Cirru;

/// Max siblings to show on each side of the focus target.
const FOLD_SIBLINGS_HALF: usize = 1;

/// Max children to show in non-target subtrees.
const FOLD_CHILDREN_NON_TARGET: usize = 2;

/// Max children to show in the focus-target subtree.
const FOLD_CHILDREN_TARGET: usize = 4;

/// Max depth for non-target subtrees before collapsing.
const FOLD_NON_TARGET_DEPTH: usize = 2;

/// Max depth for the focus-target subtree before collapsing.
const FOLD_TARGET_DEPTH: usize = 3;

/// Build a `|folded …` placeholder leaf node (compact, rendered as one token).
fn folded_place(label: &str) -> Cirru {
  Cirru::leaf(format!("|folded {label}"))
}

/// Structurally focus on a path within a Cirru tree, folding away irrelevant branches.
///
/// Walks the tree via `path`, keeping at most `FOLD_SIBLINGS_HALF` siblings
/// on each side of the target at every nesting level.  Non-target subtrees
/// are capped at `FOLD_NON_TARGET_DEPTH`; the focus path is capped at
/// `FOLD_TARGET_DEPTH`.  Hidden branches are replaced with `|folded …` leaf tokens.
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

      // In Lisp-style prefix notation the head (position 0) carries
      // semantic meaning (operator / record tag / special form) — always
      // keep it visible even when siblings before the target are folded.
      if start > 2 {
        result.push(fold_children_impl(&xs[0], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
        result.push(folded_place(&format!("|…+{} nodes before", start - 1)));
      } else {
        // 1–2 nodes: too few to justify a fold, show them directly
        for i in 0..start {
          result.push(fold_children_impl(&xs[i], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
        }
      }

      for i in start..end {
        if i == target_idx {
          result.push(focus_cirru_preview_impl(&xs[i], rest_path, depth + 1));
        } else {
          result.push(fold_children_impl(&xs[i], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
        }
      }

      let remaining = xs.len() - end;
      if remaining > 2 {
        result.push(folded_place(&format!("|…+{} nodes after", remaining)));
      } else {
        for i in end..xs.len() {
          result.push(fold_children_impl(&xs[i], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
        }
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
      let hidden = xs.len().saturating_sub(max_children);
      // Don't bother folding 1–2 nodes — just show them
      if hidden <= 2 {
        let children: Vec<Cirru> = xs.iter().map(|c| fold_children_impl(c, max_children, depth + 1, max_depth)).collect();
        return Cirru::List(children);
      }
      let mut result: Vec<Cirru> = Vec::new();
      for c in xs.iter().take(max_children) {
        result.push(fold_children_impl(c, max_children, depth + 1, max_depth));
      }
      result.push(folded_place(&format!("|…+{} nodes inside", hidden)));
      Cirru::List(result)
    }
  }
}
