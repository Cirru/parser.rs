extern crate cirru_parser;

use cirru_parser::lex;
use cirru_parser::CirruLexItem;

#[test]
fn lexer() -> Result<(), String> {
  assert_eq!(
    lex("a")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("a"))
    ],
  );
  assert_eq!(
    lex("a b")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Str(String::from("b"))
    ],
  );
  assert_eq!(
    lex("(a)")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Close,
    ],
  );
  assert_eq!(
    lex("(a b)")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Str(String::from("b")),
      CirruLexItem::Close,
    ],
  );
  assert_eq!(
    lex("(a  b)  ")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Str(String::from("b")),
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
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Indent(1),
      CirruLexItem::Str(String::from("b")),
    ],
  );
  assert_eq!(
    lex("a\n  b\nc")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Indent(1),
      CirruLexItem::Str(String::from("b")),
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("c")),
    ],
  );
  Ok(())
}

#[test]
fn lex_strings() -> Result<(), String> {
  assert_eq!(
    lex("\"a\"")?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("a"))
    ],
  );

  assert_eq!(
    lex(r#""a\\""#)?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("a\\"))
    ],
  );

  assert_eq!(
    lex(r#""a\n""#)?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from("a\n"))
    ],
  );

  Ok(())
}

#[test]
fn escape_chars() -> Result<(), String> {
  assert_eq!(
    lex(r#""\u{6c49}""#)?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from(r#"\u{6c49}"#))
    ]
  );

  assert_eq!(
    lex(r#""\'""#)?,
    vec![
      CirruLexItem::Indent(0),
      CirruLexItem::Str(String::from(r#"'"#))
    ]
  );
  Ok(())
}
