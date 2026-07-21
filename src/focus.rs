//! Structural focusing for Cirru tree previews.
//!
//! Provides `focus_cirru_preview`, which walks a Cirru tree along a path and
//! folds away irrelevant branches, keeping only the target path and nearby
//! siblings.  Hidden subtrees are replaced with a single `'folded:...`
//! placeholder leaf carrying a short, machine-parseable description of what
//! was collapsed.

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

/// Build a `'folded:...` placeholder: a single bare Cirru symbol leaf.
///
/// Uses the `'` symbol prefix (not `|` string prefix): this is an
/// identifier-like marker, not free-form string data, so `'folded:...`
/// matches Cirru's own convention for such tokens. Deliberately a single
/// leaf, not a `(folded "...")` list: a list prints inconsistently
/// depending on its position among siblings (Cirru's pretty-writer
/// inlines-with-parens the first "simple" sibling on a line but drops
/// parens for later ones on their own line), which reads as
/// confusing/broken to a human even though it is valid Cirru. `detail` must
/// only use ASCII letters, digits, `-` and `:` so the leaf always prints
/// bare (unquoted, unescaped) — spaces or non-ASCII characters would force
/// the writer to wrap it in an escaped `"..."` string.
fn folded_place(detail: &str) -> Cirru {
  Cirru::leaf(format!("'folded:{detail}"))
}

/// Structurally focus on a path within a Cirru tree, folding away irrelevant branches.
///
/// Walks the tree via `path`, keeping at most `FOLD_SIBLINGS_HALF` siblings
/// on each side of the target at every nesting level.  Non-target subtrees
/// are capped at `FOLD_NON_TARGET_DEPTH`; the focus path is capped at
/// `FOLD_TARGET_DEPTH`.  Hidden branches are replaced with a single
/// `'folded:...` placeholder leaf.
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
        result.push(folded_place(&format!("before:{}", start - 1)));
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
        result.push(folded_place(&format!("after:{remaining}")));
      } else {
        for i in end..xs.len() {
          result.push(fold_children_impl(&xs[i], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
        }
      }

      Cirru::List(result)
    }
  }
}

/// Recursively limit a subtree for display: caps depth and child count.
///
/// Two rules avoid over-folding:
/// - Only `Cirru::List` nodes are ever collapsed by depth — a `Cirru::Leaf`
///   is already atomic (adds no nested complexity) and is always shown as-is,
///   regardless of depth.
/// - The head of a list (index 0) — the operator/tag/field-name in Cirru's
///   Lisp-style prefix notation — is always kept visible and does not count
///   against `max_children`; only the remaining elements can be folded.
fn fold_children_impl(node: &Cirru, max_children: usize, depth: usize, max_depth: usize) -> Cirru {
  match node {
    Cirru::Leaf(_) => node.clone(),
    Cirru::List(xs) => {
      if depth >= max_depth {
        return folded_place("max-depth");
      }
      let Some((head, rest)) = xs.split_first() else {
        return node.clone();
      };
      let mut result = vec![fold_children_impl(head, max_children, depth + 1, max_depth)];
      let hidden = rest.len().saturating_sub(max_children);
      // Don't bother folding 1–2 nodes — just show them
      if hidden <= 2 {
        for c in rest {
          result.push(fold_children_impl(c, max_children, depth + 1, max_depth));
        }
        return Cirru::List(result);
      }
      for c in rest.iter().take(max_children) {
        result.push(fold_children_impl(c, max_children, depth + 1, max_depth));
      }
      result.push(folded_place(&format!("inside:{hidden}")));
      Cirru::List(result)
    }
  }
}
