use std::fs;
use std::io;

use serde_json::Value;

use cirru_parser::from_json_str;
use cirru_parser::parse;
use cirru_parser::CirruNode::*;

#[test]
fn parse_demo() {
  assert_eq!(parse(String::from("a")), from_json_str(r#"[["a"]]"#),);

  assert_eq!(
    parse(String::from("a b c")),
    from_json_str(r#"[["a", "b", "c"]]"#)
  );

  assert_eq!(
    parse(String::from("a\nb")),
    from_json_str(r#"[["a"], ["b"]]"#)
  );

  assert_eq!(
    parse(String::from("a (b) c")),
    from_json_str(r#"[["a", ["b"], "c"]]"#)
  );

  assert_eq!(
    parse(String::from("a (b)\n  c")),
    from_json_str(r#"[["a", ["b"], ["c"]]]"#)
  );

  assert_eq!(parse(String::from("")), from_json_str(r#"[]"#));
}

#[test]
fn parse_files() -> Result<(), io::Error> {
  let files = vec![
    // "comma",
    // "demo",
    // "folding",
    // "html",
    // "indent-twice",
    // "indent",
    "let", "line", // "paren-indent",
    // "paren-indent2",
    // "parentheses",
    "quote", "spaces",
    // "unfolding",
  ];
  for file in files {
    let json_str = fs::read_to_string(format!("./tests/data/{}.json", file))?;
    let cirru_str = fs::read_to_string(format!("./tests/cirru/{}.cirru", file))?;
    assert_eq!(parse(cirru_str), from_json_str(&json_str));
  }
  Ok(())
}
