extern crate cirru_parser;

use cirru_parser::CirruLexItem;
use cirru_parser::{lex, resolve_indentations};

#[test]
fn handle_indentation() -> Result<(), String> {
  assert_eq!(
    resolve_indentations(&lex("a\nb")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str("b".into()),
      CirruLexItem::Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n  b\nc")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Open,
      CirruLexItem::Str("b".into()),
      CirruLexItem::Close,
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str("c".into()),
      CirruLexItem::Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n  b c\nd")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Open,
      CirruLexItem::Str("b".into()),
      CirruLexItem::Str("c".into()),
      CirruLexItem::Close,
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str("d".into()),
      CirruLexItem::Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n    b c\n    d e\n  f")?),
    vec![
      CirruLexItem::Open,
      CirruLexItem::Str("a".into()),
      CirruLexItem::Open,
      CirruLexItem::Open,
      CirruLexItem::Str("b".into()),
      CirruLexItem::Str("c".into()),
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str("d".into()),
      CirruLexItem::Str("e".into()),
      CirruLexItem::Close,
      CirruLexItem::Close,
      CirruLexItem::Open,
      CirruLexItem::Str("f".into()),
      CirruLexItem::Close,
      CirruLexItem::Close,
    ]
  );
  Ok(())
}
