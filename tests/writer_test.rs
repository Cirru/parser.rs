extern crate cirru_parser;

use cirru_parser::escape_cirru_leaf;

#[cfg(feature = "serde-json")]
mod json_write_test {
  use super::*;

  use cirru_parser::{Cirru, CirruWriterOptions, format, from_json_str};
  use std::fs;
  use std::io;

  #[test]
  fn write_demo() -> Result<(), String> {
    let writer_options = CirruWriterOptions { use_inline: false };

    match from_json_str(r#"[["a"], ["b"]]"#) {
      Ok(tree) => {
        if let Cirru::List(xs) = tree {
          assert_eq!("\na\n\nb\n", format(&xs, writer_options)?)
        } else {
          panic!("unexpected leaf here")
        }
      }
      Err(e) => {
        println!("file err: {e}");
        panic!("failed to load edn data from JSON");
      }
    };

    if let Cirru::List(xs) = from_json_str(r#"[["中文"], ["中文"]]"#).unwrap() {
      assert_eq!("\n\"中文\"\n\n\"中文\"\n", format(&xs, writer_options)?)
    } else {
      panic!("unexpected leaf here")
    }

    Ok(())
  }

  #[test]
  fn write_files() -> Result<(), io::Error> {
    let files = vec![
      "append-indent",
      "comma-indent",
      "cond-short",
      "cond",
      "demo",
      "double-nesting",
      "fold-vectors",
      "folding",
      // "html-inline",
      "html",
      "indent",
      "inline-let",
      "let",
      "match",
      // "inline-mode",
      "inline-simple",
      "line",
      "nested-2",
      "parentheses",
      "quote",
      "spaces",
      "unfolding",
      "list-match",
      "tag-match",
    ];
    for file in files {
      println!("testing file: {file}");
      let json_str = fs::read_to_string(format!("./tests/writer_data/{file}.json"))?;
      let cirru_str = fs::read_to_string(format!("./tests/writer_cirru/{file}.cirru"))?;

      let writer_options = CirruWriterOptions { use_inline: false };
      match from_json_str(&json_str) {
        Ok(tree) => {
          if let Cirru::List(xs) = tree {
            assert_eq!(cirru_str, format(&xs, writer_options).unwrap());
          } else {
            panic!("unexpected leaf here")
          }
        }
        Err(e) => {
          println!("{e:?}");
          panic!("failed to load edn data from json");
        }
      }
    }
    Ok(())
  }

  #[test]
  fn write_with_inline() -> Result<(), io::Error> {
    let files = vec!["html-inline", "inline-mode"];
    for file in files {
      println!("testing file: {file}");
      let json_str = fs::read_to_string(format!("./tests/writer_data/{file}.json"))?;
      let cirru_str = fs::read_to_string(format!("./tests/writer_cirru/{file}.cirru"))?;

      let writer_options = CirruWriterOptions { use_inline: true };
      match from_json_str(&json_str) {
        Ok(tree) => {
          if let Cirru::List(xs) = tree {
            assert_eq!(cirru_str, format(&xs, writer_options).unwrap());
          } else {
            panic!("unexpected literal here")
          }
        }
        Err(e) => {
          println!("file err: {e:?}");
          panic!("failed to load edn form data");
        }
      }
    }
    Ok(())
  }
}

#[test]
fn sibling_simple_exprs_in_struct_keep_separate_lines_with_use_inline() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format};
  // When a struct has multiple sibling pairs and the first pair is block-formatted
  // (not inlined), subsequent sibling SimpleExprs should also go on separate lines,
  // not be appended inline to the previous line.
  // Bug: with use_inline=true, `(:schema :dynamic)` was appended to the `:doc` line
  // producing `:doc |long doc string (:schema :dynamic)` — a 3-element list that
  // cirru_edn parsers reject as an invalid record field pair.
  let xs = vec![Cirru::List(vec![
    Cirru::leaf("%{}"),
    Cirru::leaf(":CodeEntry"),
    Cirru::List(vec![Cirru::leaf(":doc"), Cirru::leaf("|a long doc string")]),
    Cirru::List(vec![Cirru::leaf(":schema"), Cirru::leaf(":dynamic")]),
    Cirru::List(vec![Cirru::leaf(":code"), Cirru::leaf("stuff")]),
  ])];

  let rendered = format(&xs, CirruWriterOptions { use_inline: true })?;

  // No single line should contain both :doc and :schema tokens
  for line in rendered.lines() {
    assert!(
      !(line.contains(":doc") && line.contains(":schema")),
      ":doc and :schema should be on separate lines, but got line: {:?}",
      line
    );
  }

  // Round-trip: parse the rendered output back and compare to the original tree
  let reparsed = cirru_parser::parse(&rendered).expect("rendered output should be valid Cirru");
  assert_eq!(xs, reparsed, "round-trip should preserve structure");

  Ok(())
}

#[test]
fn leaves_escapeing() {
  assert_eq!("\"a\"", escape_cirru_leaf("a"));
  assert_eq!("\"a b\"", escape_cirru_leaf("a b"));
  assert_eq!("\"a!+-b\"", escape_cirru_leaf("a!+-b"));
  assert_eq!("\"a\\nb\"", escape_cirru_leaf("a\nb"));

  assert_eq!("\"中文\"", escape_cirru_leaf("中文"));
  assert_eq!("\"中文\\n\"", escape_cirru_leaf("中文\n"));
}

#[test]
fn leaf_single_quote_without_quotes() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format};

  let xs = vec![Cirru::List(vec![Cirru::leaf("foo'bar")])];
  let rendered = format(&xs, CirruWriterOptions::from(false))?;

  assert_eq!("\nfoo'bar\n", rendered);
  Ok(())
}

#[test]
fn leaf_single_quote_with_spaces_requires_quotes() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format};

  let xs = vec![Cirru::List(vec![Cirru::leaf("foo bar'baz")])];
  let rendered = format(&xs, CirruWriterOptions::from(false))?;

  assert_eq!("\n\"foo bar'baz\"\n", rendered);
  Ok(())
}

#[test]
fn test_writer_options_from_bool() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format};

  // Directly construct test data, not dependent on JSON
  // Create a structure with multiple nested lists so that inline mode has obvious differences
  let xs = vec![Cirru::List(vec![
    Cirru::leaf("a"),
    Cirru::List(vec![Cirru::leaf("c"), Cirru::leaf("b")]),
    Cirru::List(vec![Cirru::leaf("d"), Cirru::leaf("e")]),
    Cirru::List(vec![Cirru::leaf("g"), Cirru::leaf("h")]),
  ])];

  // 测试从 bool 转换为 CirruWriterOptions 并用于 format
  let inline_result = format(&xs, true.into())?;
  let non_inline_result = format(&xs, false.into())?;

  // 验证 inline 和 non-inline 模式产生不同的输出
  assert_ne!(inline_result, non_inline_result);
  assert!(non_inline_result.contains("\n"));

  // 测试使用 From::from 方法
  let inline_from_result = format(&xs, CirruWriterOptions::from(true))?;
  let non_inline_from_result = format(&xs, CirruWriterOptions::from(false))?;

  // 验证结果一致性
  assert_eq!(inline_result, inline_from_result);
  assert_eq!(non_inline_result, non_inline_from_result);

  Ok(())
}

#[test]
fn format_let_with_nested_second_element() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format};

  let xs = vec![Cirru::List(vec![
    Cirru::leaf("let"),
    Cirru::List(vec![
      Cirru::List(vec![Cirru::leaf("a"), Cirru::leaf("1")]),
      Cirru::List(vec![Cirru::leaf("b"), Cirru::leaf("2")]),
    ]),
    Cirru::List(vec![Cirru::leaf("+"), Cirru::leaf("a"), Cirru::leaf("b")]),
  ])];

  let rendered = format(&xs, CirruWriterOptions::from(false))?;

  assert_eq!("\nlet\n    a 1\n    b 2\n  + a b\n", rendered);
  Ok(())
}

#[test]
fn format_match_without_bending_later_clauses() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format};

  let xs = vec![Cirru::List(vec![
    Cirru::leaf("match"),
    Cirru::leaf("x"),
    Cirru::List(vec![Cirru::leaf(":dyn"), Cirru::leaf("1")]),
    Cirru::List(vec![Cirru::List(vec![Cirru::leaf(":dyn"), Cirru::leaf("x")]), Cirru::leaf("2")]),
    Cirru::List(vec![
      Cirru::List(vec![Cirru::leaf(":dyn"), Cirru::leaf("x"), Cirru::leaf("y")]),
      Cirru::leaf("3"),
    ]),
    Cirru::List(vec![
      Cirru::List(vec![Cirru::leaf(":dyn"), Cirru::leaf("x"), Cirru::leaf("y"), Cirru::leaf("z")]),
      Cirru::leaf("4"),
    ]),
  ])];

  let rendered = format(&xs, CirruWriterOptions::from(false))?;

  assert_eq!(
    "\nmatch x\n  :dyn 1\n  (:dyn x) 2\n  (:dyn x y) 3\n  (:dyn x y z) 4\n",
    rendered
  );
  Ok(())
}

#[cfg(feature = "serde-json")]
#[test]
fn test_dollar_sign_spacing() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format, from_json_str};

  // Test case from user: tag-match with nested structures
  let json_str = r#"[
    [
      "tag-match",
      "self",
      [
        [
          ":plugin",
          "node",
          "cursor",
          "state"
        ],
        [
          "d!",
          "cursor",
          [
            "assoc",
            "state",
            ":show?",
            "false"
          ]
        ]
      ]
    ]
  ]"#;

  let writer_options = CirruWriterOptions { use_inline: false };

  match from_json_str(json_str) {
    Ok(tree) => {
      if let Cirru::List(xs) = tree {
        let result = format(&xs, writer_options)?;
        println!("Formatted result:\n{}", result);

        // Check that there's no "$ \n" pattern (dollar sign followed by space and newline)
        assert!(!result.contains("$ \n"), "Found unexpected '$ \\n' pattern in output");

        // The result should contain "$" followed directly by newline
        // when there are nested structures after the dollar sign
        Ok(())
      } else {
        panic!("unexpected leaf here")
      }
    }
    Err(e) => {
      println!("parse error: {e}");
      Err(format!("failed to parse JSON: {e}"))
    }
  }
}
