use std::fs;
use std::io;

use cirru_parser::{from_json_str, write_cirru, CirruWriterOptions};

#[test]
fn write_demo() {
  let writer_options = CirruWriterOptions { use_inline: false };

  match from_json_str(r#"[["a"], ["b"]]"#) {
    Ok(tree) => assert_eq!("\na\n\nb\n", write_cirru(&tree, writer_options)),
    Err(e) => {
      println!("file err: {}", e);
      panic!("failed to load edn data from JSON");
    }
  };
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
        assert_eq!(cirru_str, write_cirru(&tree, writer_options));
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
        assert_eq!(cirru_str, write_cirru(&tree, writer_options));
      }
      Err(e) => {
        println!("file err: {:?}", e);
        panic!("failed to load edn form data");
      }
    }
  }
  Ok(())
}
