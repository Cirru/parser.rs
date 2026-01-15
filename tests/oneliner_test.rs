use cirru_parser::{Cirru, format_expr_one_liner, parse_expr_one_liner};

#[test]
fn test_format_expr_one_liner() -> Result<(), String> {
  let tree = Cirru::List(vec![
    Cirru::Leaf("defn".into()),
    Cirru::Leaf("main".into()),
    Cirru::List(vec![]),
    Cirru::List(vec![Cirru::Leaf("println".into()), Cirru::Leaf("Hello, world!".into())]),
  ]);

  let one_liner = format_expr_one_liner(&tree)?;
  // Tail expression should use $ syntax
  assert_eq!(one_liner, "defn main () $ println \"Hello, world!\"");

  let parsed = parse_expr_one_liner(&one_liner)?;
  assert_eq!(parsed, tree);
  Ok(())
}

#[test]
fn test_multiple_statements_is_error() {
  assert!(parse_expr_one_liner("a\nb").is_err());
}

#[test]
fn test_complex_nesting() -> Result<(), String> {
  let tree = Cirru::List(vec![
    Cirru::Leaf("a".into()),
    Cirru::List(vec![Cirru::Leaf("b".into()), Cirru::List(vec![Cirru::Leaf("c".into())])]),
    Cirru::Leaf("d".into()),
  ]);

  let one_liner = format_expr_one_liner(&tree)?;
  assert_eq!(one_liner, "a (b (c)) d");

  let parsed = parse_expr_one_liner(&one_liner)?;
  assert_eq!(parsed, tree);

  Ok(())
}

#[test]
fn test_tail_expression() -> Result<(), String> {
  // Test tail expression with $ syntax
  let tree = Cirru::List(vec![
    Cirru::Leaf("defn".into()),
    Cirru::Leaf("main".into()),
    Cirru::List(vec![]),
    Cirru::List(vec![Cirru::Leaf("println".into()), Cirru::Leaf("Hello".into())]),
  ]);

  let one_liner = format_expr_one_liner(&tree)?;
  assert_eq!(one_liner, "defn main () $ println Hello");

  let parsed = parse_expr_one_liner(&one_liner)?;
  assert_eq!(parsed, tree);

  Ok(())
}

#[test]
fn test_nested_tail_expression() -> Result<(), String> {
  // Test nested tail expression
  let tree = Cirru::List(vec![
    Cirru::Leaf("if".into()),
    Cirru::Leaf("condition".into()),
    Cirru::List(vec![Cirru::Leaf("do".into()), Cirru::List(vec![Cirru::Leaf("action".into())])]),
  ]);

  let one_liner = format_expr_one_liner(&tree)?;
  assert_eq!(one_liner, "if condition $ do $ action");

  let parsed = parse_expr_one_liner(&one_liner)?;
  assert_eq!(parsed, tree);

  Ok(())
}

#[test]
fn test_empty_tail_expression() -> Result<(), String> {
  // Test empty list as tail expression
  let tree = Cirru::List(vec![Cirru::Leaf("a".into()), Cirru::Leaf("b".into()), Cirru::List(vec![])]);

  let one_liner = format_expr_one_liner(&tree)?;
  assert_eq!(one_liner, "a b $");

  let parsed = parse_expr_one_liner(&one_liner)?;
  assert_eq!(parsed, tree);

  Ok(())
}
