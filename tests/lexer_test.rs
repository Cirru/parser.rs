use cirru_parser::lex;
use cirru_parser::CirruLexItem::*;

#[test]
fn lexer() -> Result<(), String> {
  assert_eq!(
    lex("a")?,
    vec![LexItemIndent(0), LexItemString(String::from("a"))],
  );
  assert_eq!(
    lex("a b")?,
    vec![
      LexItemIndent(0),
      LexItemString(String::from("a")),
      LexItemString(String::from("b"))
    ],
  );
  assert_eq!(
    lex("(a)")?,
    vec![
      LexItemIndent(0),
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemClose,
    ],
  );
  assert_eq!(
    lex("(a b)")?,
    vec![
      LexItemIndent(0),
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemString(String::from("b")),
      LexItemClose,
    ],
  );
  assert_eq!(
    lex("(a  b)  ")?,
    vec![
      LexItemIndent(0),
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemString(String::from("b")),
      LexItemClose,
    ],
  );
  Ok(())
}

#[test]
fn lexer_with_indent() -> Result<(), String> {
  assert_eq!(
    lex("a\n  b")?,
    vec![
      LexItemIndent(0),
      LexItemString(String::from("a")),
      LexItemIndent(1),
      LexItemString(String::from("b")),
    ],
  );
  assert_eq!(
    lex("a\n  b\nc")?,
    vec![
      LexItemIndent(0),
      LexItemString(String::from("a")),
      LexItemIndent(1),
      LexItemString(String::from("b")),
      LexItemIndent(0),
      LexItemString(String::from("c")),
    ],
  );
  Ok(())
}

#[test]
fn lex_strings() -> Result<(), String> {
  assert_eq!(
    lex("\"a\"")?,
    vec![LexItemIndent(0), LexItemString(String::from("a"))],
  );

  assert_eq!(
    lex(r#""a\\""#)?,
    vec![LexItemIndent(0), LexItemString(String::from("a\\"))],
  );

  assert_eq!(
    lex(r#""a\n""#)?,
    vec![LexItemIndent(0), LexItemString(String::from("a\n"))],
  );

  Ok(())
}
