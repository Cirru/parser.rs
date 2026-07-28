//! Structural focusing for Cirru tree previews.
//!
//! Provides `focus_cirru_preview`, which walks a Cirru tree along a path and
//! folds away irrelevant branches, keeping only the target path and nearby
//! siblings.  Hidden subtrees are replaced with a single `'FOLDED:...`
//! placeholder leaf carrying a short, machine-parseable description of what
//! was collapsed.

use crate::primes::Cirru;

/// Controls how a focused Cirru preview marks and folds its presentation tree.
///
/// The default values preserve the behavior of [`focus_cirru_preview`]. Use
/// the builder methods instead of relying on the private field layout so more
/// presentation options can be added without breaking callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CirruFocusOptions {
  focus_marker: String,
  folded_marker: String,
  root_prefix: usize,
  non_target_children: usize,
  target_children: usize,
  non_target_depth: usize,
  target_depth: usize,
}

impl Default for CirruFocusOptions {
  fn default() -> Self {
    Self {
      focus_marker: "'FOCUSED".to_string(),
      folded_marker: "'FOLDED".to_string(),
      root_prefix: 0,
      non_target_children: 2,
      target_children: 3,
      non_target_depth: 2,
      target_depth: 2,
    }
  }
}

impl CirruFocusOptions {
  /// Change the leaf used as the head of the focus wrapper.
  pub fn with_focus_marker(mut self, marker: impl Into<String>) -> Self {
    self.focus_marker = marker.into();
    self
  }

  /// Change the prefix used for folded placeholder leaves.
  pub fn with_folded_marker(mut self, marker: impl Into<String>) -> Self {
    self.folded_marker = marker.into();
    self
  }

  /// Preserve this many leading children of the root list verbatim when the
  /// focus target follows them. This is useful for definition signatures.
  pub fn with_root_prefix(mut self, count: usize) -> Self {
    self.root_prefix = count;
    self
  }

  /// Set the maximum visible children and depth for non-target subtrees.
  pub fn with_non_target_limits(mut self, children: usize, depth: usize) -> Self {
    self.non_target_children = children;
    self.non_target_depth = depth;
    self
  }

  /// Set the maximum visible children and depth for the focused subtree.
  pub fn with_target_limits(mut self, children: usize, depth: usize) -> Self {
    self.target_children = children;
    self.target_depth = depth;
    self
  }
}

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
fn folded_place(detail: &str, options: &CirruFocusOptions) -> Cirru {
  Cirru::leaf(format!("{}:{detail}", options.folded_marker))
}

/// Keep a small readable anchor when a sibling range is folded.
///
/// A bare leaf is already compact, while a list contributes its head
/// (operator/tag) and an explicit marker for its hidden body.
fn sibling_anchor(node: &Cirru, options: &CirruFocusOptions) -> Cirru {
  match node {
    Cirru::Leaf(_) => node.clone(),
    Cirru::List(xs) => {
      let Some(head) = xs.first() else {
        return node.clone();
      };
      let mut result = vec![head.clone()];
      if xs.len() > 1 {
        result.push(folded_place(&format!("inside:{}", xs.len() - 1), options));
      }
      Cirru::List(result)
    }
  }
}

fn focused_node(node: Cirru, options: &CirruFocusOptions) -> Cirru {
  Cirru::List(vec![Cirru::leaf(options.focus_marker.as_str()), node])
}

fn folded_detail<'a>(node: &'a Cirru, options: &CirruFocusOptions) -> Option<&'a str> {
  match node {
    Cirru::Leaf(value) => value
      .strip_prefix(options.folded_marker.as_str())
      .and_then(|value| value.strip_prefix(':')),
    Cirru::List(_) => None,
  }
}

fn merge_folded_runs(nodes: Vec<Cirru>, options: &CirruFocusOptions) -> Vec<Cirru> {
  let mut result = Vec::with_capacity(nodes.len());
  let mut index = 0;
  while index < nodes.len() {
    let Some(detail) = folded_detail(&nodes[index], options) else {
      result.push(nodes[index].clone());
      index += 1;
      continue;
    };

    let mut count = 1;
    while index + count < nodes.len() && folded_detail(&nodes[index + count], options).is_some() {
      count += 1;
    }
    if count == 1 {
      result.push(nodes[index].clone());
    } else {
      let same_detail = (1..count).all(|offset| folded_detail(&nodes[index + offset], options) == Some(detail));
      let kind = if same_detail { detail } else { "merged" };
      result.push(folded_place(&format!("{kind}:{count}"), options));
    }
    index += count;
  }
  result
}

/// Structurally focus on a path within a Cirru tree, folding away irrelevant branches.
///
/// Walks the tree via `path`, keeping only the list head and target at every
/// nesting level. Other siblings are replaced with a single placeholder.
/// Non-target subtrees and the focus path use the limits in
/// [`CirruFocusOptions::default`].
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
  focus_cirru_preview_with_options(node, path, &CirruFocusOptions::default())
}

/// Structurally focus on a path using custom presentation options.
///
/// `root_prefix` is applied only to the outermost list. For example, a value
/// of 3 keeps a definition's head, name, and argument list intact while the
/// body is focused. Markers affect only the returned presentation tree.
///
/// ```rust
/// use cirru_parser::{Cirru, CirruFocusOptions, focus_cirru_preview_with_options};
///
/// let tree = Cirru::List(vec![
///   Cirru::leaf("defn"),
///   Cirru::leaf("render"),
///   Cirru::List(vec![Cirru::leaf("value")]),
///   Cirru::List(vec![Cirru::leaf("println"), Cirru::leaf("value")]),
/// ]);
/// let options = CirruFocusOptions::default()
///   .with_focus_marker("CURSOR")
///   .with_root_prefix(3);
/// let preview = focus_cirru_preview_with_options(&tree, &[3, 1], &options);
/// let Cirru::List(items) = preview else { panic!("preview should be a list") };
/// assert_eq!(items[1], Cirru::leaf("render"));
/// ```
pub fn focus_cirru_preview_with_options(node: &Cirru, path: &[usize], options: &CirruFocusOptions) -> Cirru {
  focus_cirru_preview_impl(node, path, 0, options)
}

fn focus_cirru_preview_impl(node: &Cirru, path: &[usize], depth: usize, options: &CirruFocusOptions) -> Cirru {
  match node {
    Cirru::Leaf(_) => {
      if path.is_empty() {
        focused_node(node.clone(), options)
      } else {
        node.clone()
      }
    }
    Cirru::List(xs) => {
      if path.is_empty() {
        return focused_node(
          fold_children_impl(node, options.target_children, 0, options.target_depth, options),
          options,
        );
      }

      let target_idx = path[0];
      if target_idx >= xs.len() {
        return fold_children_impl(node, options.non_target_children, 0, options.non_target_depth, options);
      }

      let rest_path = &path[1..];

      let mut result: Vec<Cirru> = Vec::new();

      // In Lisp-style prefix notation the head (position 0) carries
      // semantic meaning (operator / record tag / special form) — always
      // keep it visible even when siblings before the target are folded.
      let root_prefix = if depth == 0 { options.root_prefix.min(target_idx) } else { 0 };
      if root_prefix > 0 {
        result.extend(xs.iter().take(root_prefix).cloned());
        let hidden = target_idx - root_prefix;
        if hidden > 0 {
          result.push(sibling_anchor(&xs[root_prefix], options));
          if hidden > 1 {
            result.push(folded_place(&format!("before:{}", hidden - 1), options));
          }
        }
      } else {
        if target_idx > 0 {
          result.push(fold_children_impl(
            &xs[0],
            options.non_target_children,
            0,
            options.non_target_depth,
            options,
          ));
        }
        if target_idx > 1 {
          let hidden = target_idx - 1;
          result.push(sibling_anchor(&xs[1], options));
          if hidden > 1 {
            result.push(folded_place(&format!("before:{}", hidden - 1), options));
          }
        }
      }

      result.push(focus_cirru_preview_impl(&xs[target_idx], rest_path, depth + 1, options));

      let remaining = xs.len() - target_idx - 1;
      if remaining > 0 {
        result.push(sibling_anchor(&xs[target_idx + 1], options));
        if remaining > 1 {
          result.push(folded_place(&format!("after:{}", remaining - 1), options));
        }
      }

      Cirru::List(merge_folded_runs(result, options))
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
fn fold_children_impl(node: &Cirru, max_children: usize, depth: usize, max_depth: usize, options: &CirruFocusOptions) -> Cirru {
  match node {
    Cirru::Leaf(_) => node.clone(),
    Cirru::List(xs) => {
      if depth >= max_depth {
        return folded_place("max-depth", options);
      }
      let Some((head, rest)) = xs.split_first() else {
        return node.clone();
      };
      let mut result = vec![fold_children_impl(head, max_children, depth + 1, max_depth, options)];
      let hidden = rest.len().saturating_sub(max_children);
      // Don't bother folding 1–2 nodes — just show them
      if hidden <= 2 {
        for c in rest {
          result.push(fold_children_impl(c, max_children, depth + 1, max_depth, options));
        }
        return Cirru::List(merge_folded_runs(result, options));
      }
      for c in rest.iter().take(max_children) {
        result.push(fold_children_impl(c, max_children, depth + 1, max_depth, options));
      }
      result.push(folded_place(&format!("inside:{hidden}"), options));
      Cirru::List(merge_folded_runs(result, options))
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
      fold_children_impl(&tree, 3, 0, 1, &CirruFocusOptions::default()),
      Cirru::List(vec![leaf("root"), leaf("'FOLDED:max-depth:2")])
    );
  }

  #[test]
  fn out_of_range_path_does_not_mark_a_focus_node() {
    let tree = Cirru::List(vec![leaf("root"), leaf("child")]);

    assert_eq!(focus_cirru_preview(&tree, &[4]), Cirru::List(vec![leaf("root"), leaf("child")]));
  }

  #[test]
  fn custom_options_preserve_root_signature_and_change_markers() {
    let args = Cirru::List(vec![leaf("value"), leaf("options")]);
    let tree = Cirru::List(vec![
      leaf("defn"),
      leaf("render"),
      args.clone(),
      Cirru::List(vec![leaf("let"), leaf("target"), leaf("tail")]),
      leaf("metadata"),
    ]);
    let options = CirruFocusOptions::default()
      .with_focus_marker("CURSOR")
      .with_folded_marker("FOLDED")
      .with_root_prefix(3);
    let focused = focus_cirru_preview_with_options(&tree, &[3, 1], &options);
    let Cirru::List(items) = focused else {
      panic!("focused definition should remain a list")
    };
    assert_eq!(&items[..3], &[leaf("defn"), leaf("render"), args]);
    assert_eq!(
      items[3],
      Cirru::List(vec![leaf("let"), Cirru::List(vec![leaf("CURSOR"), leaf("target")]), leaf("tail"),])
    );
    assert_eq!(items[4], leaf("metadata"));
  }

  #[test]
  fn custom_options_tune_target_and_non_target_fold_limits() {
    let tree = Cirru::List(vec![leaf("root"), leaf("a"), leaf("b"), leaf("c"), leaf("d")]);
    let target_options = CirruFocusOptions::default().with_target_limits(1, 2);
    assert_eq!(
      focus_cirru_preview_with_options(&tree, &[], &target_options),
      Cirru::List(vec![
        leaf("'FOCUSED"),
        Cirru::List(vec![leaf("root"), leaf("a"), leaf("'FOLDED:inside:3")]),
      ])
    );

    let non_target_options = CirruFocusOptions::default().with_non_target_limits(0, 2);
    assert_eq!(
      focus_cirru_preview_with_options(&tree, &[9], &non_target_options),
      Cirru::List(vec![leaf("root"), leaf("'FOLDED:inside:4")])
    );
  }
}
