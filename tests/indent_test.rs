use cirru_parser::CirruLexItem;
use cirru_parser::{lex, resolve_indentations};

#[test]
fn handle_indentation() -> Result<(), String> {
  assert_eq!(
    resolve_indentations(&lex("a\nb")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("b")),
      CirruLexItem::Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n  b\nc")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("b")),
      CirruLexItem::Close,
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("c")),
      CirruLexItem::Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n  b c\nd")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("b")),
      CirruLexItem::Str(String::from("c")),
      CirruLexItem::Close,
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("d")),
      CirruLexItem::Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n    b c\n    d e\n  f")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("a")),
      CirruLexItem::Open,
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("b")),
      CirruLexItem::Str(String::from("c")),
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("d")),
      CirruLexItem::Str(String::from("e")),
      CirruLexItem::Close,
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str(String::from("f")),
      CirruLexItem::Close,
      CirruLexItem::Close,
    ]
  );
  Ok(())
}
