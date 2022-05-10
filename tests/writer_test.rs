extern crate cirru_parser;

use cirru_parser::escape_cirru_leaf;

#[cfg(feature = "use-json")]
mod json_write_test {
  use super::*;

  use cirru_parser::{format, from_json_str, Cirru, CirruWriterOptions};
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
        println!("file err: {}", e);
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
      // "inline-mode",
      "inline-simple",
      "line",
      "nested-2",
      "parentheses",
      "quote",
      "spaces",
      "unfolding",
    ];
    for file in files {
      println!("testing file: {}", file);
      let json_str = fs::read_to_string(format!("./tests/writer_data/{}.json", file))?;
      let cirru_str = fs::read_to_string(format!("./tests/writer_cirru/{}.cirru", file))?;

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
          println!("{:?}", e);
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
      println!("testing file: {}", file);
      let json_str = fs::read_to_string(format!("./tests/writer_data/{}.json", file))?;
      let cirru_str = fs::read_to_string(format!("./tests/writer_cirru/{}.cirru", file))?;

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
          println!("file err: {:?}", e);
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
