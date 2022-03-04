extern crate cirru_parser;

use cirru_parser::lex;
use cirru_parser::CirruLexItem;

#[test]
fn lexer() -> Result<(), String> {
  assert_eq!(lex("a")?, vec![CirruLexItem::Indent(0), CirruLexItem::Str("a".into())],);
  assert_eq!(
    lex("a b")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str("a".into()),
      CirruLexItem::Str("b".into())
    ],
  );
  assert_eq!(
    lex("(a)")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Close,
    ],
  );
  assert_eq!(
    lex("(a b)")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Str("b".into()),
      CirruLexItem::Close,
    ],
  );
  assert_eq!(
    lex("(a  b)  ")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Str("b".into()),
      CirruLexItem::Close,
    ],
  );
  Ok(())
}

#[test]
fn lexer_with_indent() -> Result<(), String> {
  assert_eq!(
    lex("a\n  b")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str("a".into()),
      CirruLexItem::Indent(1),
      CirruLexItem::Str("b".into()),
    ],
  );
  assert_eq!(
    lex("a\n  b\nc")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str("a".into()),
      CirruLexItem::Indent(1),
      CirruLexItem::Str("b".into()),
      CirruLexItem::Indent(0),
      CirruLexItem::Str("c".into()),
    ],
  );
  Ok(())
}

#[test]
fn lex_strings() -> Result<(), String> {
  assert_eq!(lex("\"a\"")?, vec![CirruLexItem::Indent(0), CirruLexItem::Str("a".into())],);

  assert_eq!(lex(r#""a\\""#)?, vec![CirruLexItem::Indent(0), CirruLexItem::Str("a\\".into())],);

  assert_eq!(lex(r#""a\n""#)?, vec![CirruLexItem::Indent(0), CirruLexItem::Str("a\n".into())],);

  Ok(())
}

#[test]
fn escape_chars() -> Result<(), String> {
  assert_eq!(
    lex(r#""\u{6c49}""#)?,
    vec![CirruLexItem::Indent(0), CirruLexItem::Str(r#"\u{6c49}"#.into())]
  );

  assert_eq!(lex(r#""\'""#)?, vec![CirruLexItem::Indent(0), CirruLexItem::Str(r#"'"#.into())]);
  Ok(())
}
