//! Structural focusing for Cirru tree previews.
//!
//! Provides `focus_cirru_preview`, which walks a Cirru tree along a path and
//! folds away irrelevant branches, keeping only the target path and nearby
//! siblings.  Hidden subtrees are replaced with a single `'FOLDED:...`
//! placeholder leaf carrying a short, machine-parseable description of what
//! was collapsed.

use crate::primes::Cirru;

/// Max siblings to show on each side of the focus target.

/// Max children to show in non-target subtrees.
const FOLD_CHILDREN_NON_TARGET: usize = 2;

/// Max children to show in the focus-target subtree.
const FOLD_CHILDREN_TARGET: usize = 3;

/// Max depth for non-target subtrees before collapsing.
const FOLD_NON_TARGET_DEPTH: usize = 2;

/// Max depth for the focus-target subtree before collapsing.
const FOLD_TARGET_DEPTH: usize = 2;

/// Build a `'FOLDED:...` placeholder: a single bare Cirru symbol leaf.
///
/// Uses the `'` symbol prefix (not `|` string prefix): this is an
/// identifier-like marker, not free-form string data, so `'FOLDED:...`
/// matches Cirru's own convention for such tokens. Deliberately a single
/// leaf, not a `(FOLDED "...")` list: a list prints inconsistently
/// depending on its position among siblings (Cirru's pretty-writer
/// inlines-with-parens the first "simple" sibling on a line but drops
/// parens for later ones on their own line), which reads as
/// confusing/broken to a human even though it is valid Cirru. `detail` must
/// only use ASCII letters, digits, `-` and `:` so the leaf always prints
/// bare (unquoted, unescaped) — spaces or non-ASCII characters would force
/// the writer to wrap it in an escaped `"..."` string.
fn folded_place(detail: &str) -> Cirru {
  Cirru::leaf(format!("'FOLDED:{detail}"))
}

/// Keep a small readable anchor when a sibling range is folded.
///
/// A bare leaf is already compact, while a list contributes its head
/// (operator/tag) and an explicit marker for its hidden body.
fn sibling_anchor(node: &Cirru) -> Cirru {
  match node {
    Cirru::Leaf(_) => node.clone(),
    Cirru::List(xs) => {
      let Some(head) = xs.first() else {
        return node.clone();
      };
      let mut result = vec![head.clone()];
      if xs.len() > 1 {
        result.push(folded_place(&format!("inside:{}", xs.len() - 1)));
      }
      Cirru::List(result)
    }
  }
}

fn focused_node(node: Cirru) -> Cirru {
  Cirru::List(vec![Cirru::leaf("'FOCUSED"), node])
}

fn folded_detail(node: &Cirru) -> Option<&str> {
  match node {
    Cirru::Leaf(value) => value.strip_prefix("'FOLDED:"),
    Cirru::List(_) => None,
  }
}

fn merge_folded_runs(nodes: Vec<Cirru>) -> Vec<Cirru> {
  let mut result = Vec::with_capacity(nodes.len());
  let mut index = 0;
  while index < nodes.len() {
    let Some(detail) = folded_detail(&nodes[index]) else {
      result.push(nodes[index].clone());
      index += 1;
      continue;
    };

    let mut count = 1;
    while index + count < nodes.len() && folded_detail(&nodes[index + count]).is_some() {
      count += 1;
    }
    if count == 1 {
      result.push(nodes[index].clone());
    } else {
      let same_detail = (1..count).all(|offset| folded_detail(&nodes[index + offset]) == Some(detail));
      let kind = if same_detail { detail } else { "merged" };
      result.push(folded_place(&format!("{kind}:{count}")));
    }
    index += count;
  }
  result
}

/// Structurally focus on a path within a Cirru tree, folding away irrelevant branches.
///
/// Walks the tree via `path`, keeping only the list head and target at every
/// nesting level. Other siblings are replaced with a single placeholder.
/// Non-target subtrees are capped at `FOLD_NON_TARGET_DEPTH`; the focus path
/// is capped at `FOLD_TARGET_DEPTH`.
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
    Cirru::Leaf(_) => {
      if path.is_empty() {
        focused_node(node.clone())
      } else {
        node.clone()
      }
    }
    Cirru::List(xs) => {
      if path.is_empty() {
        return focused_node(fold_children_impl(node, FOLD_CHILDREN_TARGET, 0, FOLD_TARGET_DEPTH));
      }

      let target_idx = path[0];
      if target_idx >= xs.len() {
        return fold_children_impl(node, FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH);
      }

      let rest_path = &path[1..];

      let mut result: Vec<Cirru> = Vec::new();

      // In Lisp-style prefix notation the head (position 0) carries
      // semantic meaning (operator / record tag / special form) — always
      // keep it visible even when siblings before the target are folded.
      if target_idx > 0 {
        result.push(fold_children_impl(&xs[0], FOLD_CHILDREN_NON_TARGET, 0, FOLD_NON_TARGET_DEPTH));
      }
      if target_idx > 1 {
        let hidden = target_idx - 1;
        result.push(sibling_anchor(&xs[1]));
        if hidden > 1 {
          result.push(folded_place(&format!("before:{}", hidden - 1)));
        }
      }

      result.push(focus_cirru_preview_impl(&xs[target_idx], rest_path, depth + 1));

      let remaining = xs.len() - target_idx - 1;
      if remaining > 0 {
        result.push(sibling_anchor(&xs[target_idx + 1]));
        if remaining > 1 {
          result.push(folded_place(&format!("after:{}", remaining - 1)));
        }
      }

      Cirru::List(merge_folded_runs(result))
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
        return Cirru::List(merge_folded_runs(result));
      }
      for c in rest.iter().take(max_children) {
        result.push(fold_children_impl(c, max_children, depth + 1, max_depth));
      }
      result.push(folded_place(&format!("inside:{hidden}")));
      Cirru::List(merge_folded_runs(result))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn leaf(value: &str) -> Cirru {
    Cirru::leaf(value)
  }

  #[test]
  fn marks_a_focused_leaf_without_changing_its_value() {
    let tree = Cirru::List(vec![leaf("call"), leaf("first"), leaf("target"), leaf("last")]);

    assert_eq!(
      focus_cirru_preview(&tree, &[2]),
      Cirru::List(vec![
        leaf("call"),
        leaf("first"),
        Cirru::List(vec![leaf("'FOCUSED"), leaf("target")]),
        leaf("last"),
      ])
    );
  }

  #[test]
  fn keeps_anchors_and_fold_markers_for_sibling_ranges() {
    let tree = Cirru::List(vec![
      leaf("pipeline"),
      leaf("name"),
      Cirru::List(vec![leaf("config"), leaf("a"), leaf("b")]),
      leaf("target"),
      Cirru::List(vec![leaf("tail"), leaf("x"), leaf("y")]),
      leaf("last"),
    ]);

    assert_eq!(
      focus_cirru_preview(&tree, &[3]),
      Cirru::List(vec![
        leaf("pipeline"),
        leaf("name"),
        leaf("'FOLDED:before:1"),
        Cirru::List(vec![leaf("'FOCUSED"), leaf("target")]),
        Cirru::List(vec![leaf("tail"), leaf("'FOLDED:inside:2")]),
        leaf("'FOLDED:after:1"),
      ])
    );
  }

  #[test]
  fn merges_adjacent_folded_placeholders_with_a_count() {
    let tree = Cirru::List(vec![leaf("root"), Cirru::List(vec![leaf("a")]), Cirru::List(vec![leaf("b")])]);

    assert_eq!(
      fold_children_impl(&tree, 3, 0, 1),
      Cirru::List(vec![leaf("root"), leaf("'FOLDED:max-depth:2")])
    );
  }

  #[test]
  fn out_of_range_path_does_not_mark_a_focus_node() {
    let tree = Cirru::List(vec![leaf("root"), leaf("child")]);

    assert_eq!(focus_cirru_preview(&tree, &[4]), Cirru::List(vec![leaf("root"), leaf("child")]));
  }
}
