use std::fs;
use std::io;

use serde_json::json;

mod json;

use cirru_parser::{parse, Cirru};

use json::{from_json_str, from_json_value};

#[test]
fn parse_demo() {
  assert_eq!(parse("a").map(Cirru::List), from_json_str(r#"[["a"]]"#));

  assert_eq!(
    parse("a b c").map(Cirru::List),
    from_json_str(r#"[["a", "b", "c"]]"#)
  );

  assert_eq!(
    parse("a\nb").map(Cirru::List),
    from_json_str(r#"[["a"], ["b"]]"#)
  );

  assert_eq!(
    parse("a (b) c").map(Cirru::List),
    Ok(from_json_value(json!([["a", ["b"], "c"]])))
  );

  assert_eq!(
    parse("a (b)\n  c").map(Cirru::List),
    Ok(from_json_value(json!([["a", ["b"], ["c"]]])))
  );

  assert_eq!(parse("").map(Cirru::List), from_json_str(r#"[]"#));
}

#[test]
fn parse_files() -> Result<(), io::Error> {
  let files = vec![
    "comma",
    "demo",
    "folding",
    "html",
    "indent-twice",
    "indent",
    "let",
    "line",
    "paren-indent",
    "paren-indent2", // same result as parent-indent
    "parentheses",
    "quote",
    "spaces",
    "unfolding",
  ];
  for file in files {
    println!("testing file: {}", file);
    let json_str = fs::read_to_string(format!("./tests/data/{}.json", file))?;
    let cirru_str = fs::read_to_string(format!("./tests/cirru/{}.cirru", file))?;
    assert_eq!(parse(&cirru_str).map(Cirru::List), from_json_str(&json_str));
  }
  Ok(())
}
