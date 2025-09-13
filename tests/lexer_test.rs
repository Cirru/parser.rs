extern crate cirru_parser;

use cirru_parser::CirruLexItem;
use cirru_parser::lex;

#[test]
fn lexer() -> Result<(), String> {
  assert_eq!(lex("a")?, vec![0.into(), "a".into()]);
  assert_eq!(lex("a b")?, vec![0.into(), "a".into(), "b".into()],);
  assert_eq!(lex("(a)")?, vec![0.into(), CirruLexItem::Open, "a".into(), CirruLexItem::Close,],);
  assert_eq!(
    lex("(a b)")?,
    vec![0.into(), CirruLexItem::Open, "a".into(), "b".into(), CirruLexItem::Close,],
  );
  assert_eq!(
    lex("(a  b)  ")?,
    vec![0.into(), CirruLexItem::Open, "a".into(), "b".into(), CirruLexItem::Close,],
  );
  Ok(())
}

#[test]
fn lexer_with_indent() -> Result<(), String> {
  assert_eq!(lex("a\n  b")?, vec![0.into(), "a".into(), 1.into(), "b".into(),],);
  assert_eq!(
    lex("a\n  b\nc")?,
    vec![0.into(), "a".into(), 1.into(), "b".into(), 0.into(), "c".into(),],
  );
  Ok(())
}

#[test]
fn lex_strings() -> Result<(), String> {
  assert_eq!(lex("\"a\"")?, vec![0.into(), "a".into()],);

  assert_eq!(lex(r#""a\\""#)?, vec![0.into(), "a\\".into()],);

  assert_eq!(lex(r#""a\n""#)?, vec![0.into(), "a\n".into()],);

  Ok(())
}

#[test]
fn escape_chars() -> Result<(), String> {
  assert_eq!(lex(r#""\u{6c49}""#)?, vec![0.into(), r#"\u{6c49}"#.into()]);

  assert_eq!(lex(r#""\'""#)?, vec![0.into(), r#"'"#.into()]);
  Ok(())
}
