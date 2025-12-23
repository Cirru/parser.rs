use cirru_parser::{Cirru, format_expr_oneliner, parse_expr_oneliner};

#[test]
fn test_format_expr_oneliner() -> Result<(), String> {
  let tree = Cirru::List(vec![
    Cirru::Leaf("defn".into()),
    Cirru::Leaf("main".into()),
    Cirru::List(vec![]),
    Cirru::List(vec![Cirru::Leaf("println".into()), Cirru::Leaf("Hello, world!".into())]),
  ]);

  let oneliner = format_expr_oneliner(&tree)?;
  assert_eq!(oneliner, "defn main () (println \"Hello, world!\")");

  let parsed = parse_expr_oneliner(&oneliner)?;
  assert_eq!(parsed, tree);
  Ok(())
}

#[test]
fn test_multiple_statements_is_error() {
  assert!(parse_expr_oneliner("a\nb").is_err());
}

#[test]
fn test_complex_nesting() -> Result<(), String> {
  let tree = Cirru::List(vec![
    Cirru::Leaf("a".into()),
    Cirru::List(vec![Cirru::Leaf("b".into()), Cirru::List(vec![Cirru::Leaf("c".into())])]),
    Cirru::Leaf("d".into()),
  ]);

  let oneliner = format_expr_oneliner(&tree)?;
  assert_eq!(oneliner, "a (b (c)) d");

  let parsed = parse_expr_oneliner(&oneliner)?;
  assert_eq!(parsed, tree);

  Ok(())
}
