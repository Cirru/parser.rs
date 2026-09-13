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
    Cirru::List(vec![Cirru::List(vec![Cirru::leaf(":dyn")]), Cirru::leaf("1")]),
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

  assert_eq!("\nmatch x\n  (:dyn) 1\n  (:dyn x) 2\n  (:dyn x y) 3\n  (:dyn x y z) 4\n", rendered);
  Ok(())
}

#[test]
fn format_tail_fold_boundaries() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format, parse};

  let one_fold = vec![Cirru::List(vec![
    Cirru::leaf("a"),
    Cirru::List(vec![Cirru::leaf("b"), Cirru::leaf("c")]),
  ])];
  let two_folds = vec![Cirru::List(vec![
    Cirru::leaf("a"),
    Cirru::List(vec![Cirru::leaf("b"), Cirru::List(vec![Cirru::leaf("c"), Cirru::leaf("d")])]),
  ])];
  let three_folds = vec![Cirru::List(vec![
    Cirru::leaf("a"),
    Cirru::List(vec![
      Cirru::leaf("b"),
      Cirru::List(vec![Cirru::leaf("c"), Cirru::List(vec![Cirru::leaf("d"), Cirru::leaf("e")])]),
    ]),
  ])];

  let one_rendered = format(&one_fold, CirruWriterOptions::from(true))?;
  let two_rendered = format(&two_folds, CirruWriterOptions::from(true))?;
  let three_rendered = format(&three_folds, CirruWriterOptions::from(true))?;

  assert_eq!("\na $ b c\n", one_rendered);
  assert_eq!("\na $ b $ c d\n", two_rendered);
  assert_eq!("\na $ b $ c (d e)\n", three_rendered);
  assert_eq!(one_fold, parse(&one_rendered).expect("one-fold output should remain valid Cirru"));
  assert_eq!(two_folds, parse(&two_rendered).expect("two-fold output should remain valid Cirru"));
  assert_eq!(
    three_folds,
    parse(&three_rendered).expect("three-fold output should remain valid Cirru")
  );
  Ok(())
}

#[test]
fn format_simple_expression_leaf_count_boundaries() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format, parse};

  let simple_expr = |size: usize| Cirru::List((0..size).map(|idx| Cirru::leaf(format!("x{idx}"))).collect());
  let seven = vec![Cirru::List(vec![Cirru::leaf("{}"), simple_expr(7), Cirru::leaf("tail")])];
  let eight = vec![Cirru::List(vec![Cirru::leaf("{}"), simple_expr(8), Cirru::leaf("tail")])];
  let nine = vec![Cirru::List(vec![Cirru::leaf("{}"), simple_expr(9), Cirru::leaf("tail")])];

  let seven_rendered = format(&seven, CirruWriterOptions::from(true))?;
  let eight_rendered = format(&eight, CirruWriterOptions::from(true))?;
  let nine_rendered = format(&nine, CirruWriterOptions::from(true))?;

  assert_eq!("\n{} (x0 x1 x2 x3 x4 x5 x6) tail\n", seven_rendered);
  assert_eq!("\n{} (x0 x1 x2 x3 x4 x5 x6 x7) tail\n", eight_rendered);
  assert_eq!("\n{}\n  x0 x1 x2 x3 x4 x5 x6 x7 x8\n  , tail\n", nine_rendered);
  assert_eq!(seven, parse(&seven_rendered).expect("seven-leaf output should remain valid Cirru"));
  assert_eq!(eight, parse(&eight_rendered).expect("eight-leaf output should remain valid Cirru"));
  assert_eq!(nine, parse(&nine_rendered).expect("nine-leaf output should remain valid Cirru"));
  Ok(())
}

#[test]
fn format_simple_expression_leaf_length_boundaries() -> Result<(), String> {
  use cirru_parser::{Cirru, CirruWriterOptions, format, parse};

  let expr_with_value = |value: &str| {
    vec![Cirru::List(vec![
      Cirru::leaf("{}"),
      Cirru::List(vec![Cirru::leaf("label"), Cirru::leaf(value)]),
      Cirru::leaf("tail"),
    ])]
  };
  let fifteen = expr_with_value("123456789012345");
  let sixteen = expr_with_value("1234567890123456");
  let seventeen = expr_with_value("12345678901234567");
  let unicode_sixteen = expr_with_value("甲乙丙丁戊己庚辛壬癸子丑寅卯辰巳");

  let fifteen_rendered = format(&fifteen, CirruWriterOptions::from(true))?;
  let sixteen_rendered = format(&sixteen, CirruWriterOptions::from(true))?;
  let seventeen_rendered = format(&seventeen, CirruWriterOptions::from(true))?;
  let unicode_rendered = format(&unicode_sixteen, CirruWriterOptions::from(true))?;

  assert_eq!("\n{} (label 123456789012345) tail\n", fifteen_rendered);
  assert_eq!("\n{} (label 1234567890123456) tail\n", sixteen_rendered);
  assert_eq!("\n{}\n  label 12345678901234567\n  , tail\n", seventeen_rendered);
  assert_eq!("\n{} (label \"甲乙丙丁戊己庚辛壬癸子丑寅卯辰巳\") tail\n", unicode_rendered);
  assert_eq!(fifteen, parse(&fifteen_rendered).expect("15-char output should remain valid Cirru"));
  assert_eq!(sixteen, parse(&sixteen_rendered).expect("16-char output should remain valid Cirru"));
  assert_eq!(
    seventeen,
    parse(&seventeen_rendered).expect("17-char output should remain valid Cirru")
  );
  assert_eq!(
    unicode_sixteen,
    parse(&unicode_rendered).expect("16-char Unicode output should remain valid Cirru")
  );
  Ok(())
}

#[test]
fn format_calcit_fixtures_are_short_canonical_and_lossless() -> Result<(), String> {
  use cirru_parser::{CirruWriterOptions, format, parse};

  // Canonicalized excerpts from calcit/test-set.cirru,
  // calcit/type-fail/js-nullish-dereference-strict.cirru, and
  // calcit/type-fail/whole-dynamic-schema-strict.cirru.
  let fixtures = [
    ("calcit-test-set", include_str!("writer_cases/calcit-test-set.cirru")),
    (
      "calcit-js-nullish-dereference",
      include_str!("writer_cases/calcit-js-nullish-dereference.cirru"),
    ),
    (
      "calcit-whole-dynamic-schema",
      include_str!("writer_cases/calcit-whole-dynamic-schema.cirru"),
    ),
  ];

  for (name, source) in fixtures {
    assert!(source.lines().count() <= 30, "{name} should stay within 30 lines");
    let tree = parse(source).unwrap_or_else(|error| panic!("{name} should parse: {error}"));
    let rendered = format(&tree, CirruWriterOptions::from(true))?;
    assert_eq!(source, rendered, "{name} should already use the canonical writer layout");
    assert_eq!(
      tree,
      parse(&rendered).unwrap_or_else(|error| panic!("formatted {name} should parse: {error}"))
    );
  }
  Ok(())
}

#[test]
fn format_wasi_wait_fixture_with_new_layout_rules() -> Result<(), String> {
  use cirru_parser::{CirruWriterOptions, format, parse};

  let source = include_str!("writer_cases/wasi-wait.cirru");
  let tree = parse(source).expect("wasi-wait fixture should parse");
  let rendered = format(&tree, CirruWriterOptions::from(true))?;

  assert_eq!(source, rendered, "wasi-wait fixture should already use the canonical writer layout");
  assert!(rendered.contains(":code $ quote $ defn wait-ms (milliseconds)"));
  assert!(rendered.contains(":edges $ #{} $ :: :call 'calcit.core/wait-ms 'calcit.core/&wait-ms"));
  assert!(rendered.contains("and (round? milliseconds) (>= milliseconds 0) (<= milliseconds 4294967295)"));
  assert_eq!(tree, parse(&rendered).expect("formatted fixture should preserve its tree"));
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
