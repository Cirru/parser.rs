extern crate cirru_parser;

mod json_test {

  use cirru_parser::{parse, Cirru};

  #[cfg(feature = "serde-json")]
  use cirru_parser::from_json_str;

  #[test]
  fn parse_demo() {
    assert_eq!(parse("a").map(Cirru::List), Ok(Cirru::List(vec!(vec!["a"].into()))));

    assert_eq!(parse("a b c").map(Cirru::List), Ok(Cirru::List(vec!(vec!["a", "b", "c"].into()))));

    assert_eq!(
      parse("a\nb").map(Cirru::List),
      Ok(Cirru::List(vec!(vec!["a"].into(), vec!["b"].into())))
    );

    assert_eq!(parse("a\rb").map(Cirru::List), Ok(Cirru::List(vec!(vec!["a\rb"].into()))));

    assert_eq!(
      parse("a (b) c").map(Cirru::List),
      Ok(Cirru::List(vec![vec![Cirru::leaf("a"), vec!["b"].into(), "c".into()].into()]))
    );

    assert_eq!(
      parse("a (b)\n  c").map(Cirru::List),
      Ok(Cirru::List(vec!(vec![Cirru::leaf("a"), vec!["b"].into(), vec!["c"].into()].into())))
    );

    assert_eq!(parse("").map(Cirru::List), Ok((vec![] as Vec<Cirru>).into()));
  }

  #[cfg(feature = "serde-json")]
  use std::io;

  #[cfg(feature = "serde-json")]
  #[test]
  fn parse_files() -> Result<(), io::Error> {
    use std::fs;

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
      "list-match",
    ];
    for file in files {
      println!("testing file: {}", file);
      let json_str = fs::read_to_string(format!("./tests/data/{}.json", file))?;
      let cirru_str = fs::read_to_string(format!("./tests/cirru/{}.cirru", file))?;

      assert_eq!(parse(&cirru_str).map(Cirru::List), from_json_str(&json_str));
    }
    Ok(())
  }
}
