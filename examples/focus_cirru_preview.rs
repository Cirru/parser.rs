//! Small, repeatable examples for `focus_cirru_preview`.
//!
//! Run with:
//!
//! ```text
//! cargo run --example focus_cirru_preview
//! ```

use cirru_parser::{Cirru, CirruOneLinerParseExt, CirruWriterOptions, focus_cirru_preview, format};

fn path_exists(node: &Cirru, path: &[usize]) -> bool {
  let mut current = node;
  for &index in path {
    match current {
      Cirru::List(children) => match children.get(index) {
        Some(child) => current = child,
        None => return false,
      },
      Cirru::Leaf(_) => return false,
    }
  }
  true
}

fn print_block(label: &str, content: &str) {
  println!("  {label}:");
  for line in content.lines() {
    println!("    {line}");
  }
}

fn show(code: &str, paths: &[&[usize]]) -> Result<(), String> {
  let tree = code.parse_expr_one_liner().map_err(|e| e.to_string())?;
  let options = CirruWriterOptions { use_inline: false };
  let source = format(std::slice::from_ref(&tree), options).map_err(|e| e.to_string())?;
  print_block("source", source.trim());

  for path in paths {
    let focused = focus_cirru_preview(&tree, path);
    let path_text = path.iter().map(usize::to_string).collect::<Vec<_>>().join(".");
    println!("  path @{path_text} (focus target)");
    if path_exists(&tree, path) {
      let result = format(std::slice::from_ref(&focused), options).map_err(|e| e.to_string())?;
      print_block("result", result.trim());
    } else {
      println!("    result: no content");
    }
  }
  println!();
  Ok(())
}

fn main() -> Result<(), String> {
  // The paths deliberately look random, but are fixed so the example is
  // useful in documentation and its output stays reproducible.
  let cases: &[(&str, &[&[usize]])] = &[
    (
      "pipeline (with (retry 3) (timeout 5000) (trace request-id) (metrics latency) (circuit-breaker payments)) (compose (map normalize-user) (map enrich-permissions) (filter active) (filter verified) (sort-by created-at) (group-by organization) (reduce merge-user empty) (dedupe user-id) (partition region) (annotate source)) (load-records source (where tenant-id) (select id profile settings) (include contacts) (include preferences)) (join (load-permissions acl) (on user-id)) (cache (key request-id) (ttl 300) (namespace users) (refresh background)) (emit report (format json) (compress gzip))",
      &[&[1, 3, 0], &[2, 8, 1], &[3, 3, 1], &[5, 3, 1]],
    ),
    (
      "match request (route (method request) (path request) (host request) (version request) (scheme https) (region us-east)) (get (cache key) (decode headers) (decode query) (authorize user roles) (load-resource db) (audit read) (respond 200 resource) (add-links resource) (trace response)) (post (validate schema body) (authorize user roles) (begin-transaction db) (save resource) (publish events) (commit-transaction db) (respond 201 resource)) (delete (authorize user roles) (load-resource db) (check-owner user resource) (delete-resource db) (audit delete) (respond 204)) (fallback (log unmatched-request) (respond 404 not-found))",
      &[&[2, 4, 1], &[3, 7, 1], &[4, 3, 1], &[6, 1, 1]],
    ),
    (
      "component dashboard (props (title text subtitle text) (filters (range start end timezone) (tags selected available) (owners selected) (status selected)) (actions (refresh source) (export csv columns) (share recipients) (save-view name))) (state (loading false) (query (page 1 size 50 sort created-at) (search text)) (data (summary totals) (series points) (rows items))) (layout (header (logo brand href) (menu primary secondary) (account user notifications)) (sidebar (navigation sections) (saved-views views)) (content (toolbar filters actions) (chart sales quarter points) (chart conversion funnel stages) (table rows columns pagination) (empty-state message action)) (footer links legal version))",
      &[&[2, 1, 1], &[3, 1, 1], &[4, 3, 1], &[4, 2, 1]],
    ),
    (
      "defn build-index (config options) (let (sources (config-sources config)) (entries (concat (map (fn (source) (read-all (source-path source) (source-pattern source) (source-ignore source))) sources))) (normalized (map (fn (entry) (normalize-entry entry config options)) entries)) (validated (filter (fn (entry) (and (valid-path entry) (has-frontmatter entry) (allowed-kind entry options))) normalized)) (grouped (group-by (project-key config) (kind-key) validated)) (sorted (map (fn (group) (sort-by (priority options) (updated-at) group)) grouped)) (index (map (fn (group) (build-index-entry group config)) sorted)) (write-index (output-path config) (metadata config options) index) (write-search-cache (cache-path config) (search-documents index)))",
      &[&[3, 1, 1], &[3, 2, 1, 1, 2], &[3, 4, 1], &[3, 7, 1]],
    ),
  ];

  for (code, paths) in cases {
    show(code, paths)?;
  }

  // A leaf is also a valid input; focusing an invalid coordinate folds the
  // available tree without panicking.
  let leaf = Cirru::leaf("terminal");
  println!("source: terminal");
  let path = &[7, 3];
  println!("  path @7.3");
  println!("    result: {}", if path_exists(&leaf, path) { "content" } else { "no content" });

  Ok(())
}
