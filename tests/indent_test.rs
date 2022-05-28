extern crate cirru_parser;

use cirru_parser::CirruLexItem;
use cirru_parser::{lex, resolve_indentations};

#[test]
fn handle_indentation() -> Result<(), String> {
  use CirruLexItem::*;
  assert_eq!(
    resolve_indentations(&lex("a\nb")?),
    vec![Open, "a".into(), Close, Open, "b".into(), Close,]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n  b\nc")?),
    vec![Open, "a".into(), Open, "b".into(), Close, Close, Open, "c".into(), Close,]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n  b c\nd")?),
    vec![
      Open,
      "a".into(),
      Open,
      "b".into(),
      "c".into(),
      Close,
      Close,
      Open,
      "d".into(),
      Close,
    ]
  );
  assert_eq!(
    resolve_indentations(&lex("a\n    b c\n    d e\n  f")?),
    vec![
      Open,
      "a".into(),
      Open,
      Open,
      "b".into(),
      "c".into(),
      Close,
      Open,
      "d".into(),
      "e".into(),
      Close,
      Close,
      Open,
      "f".into(),
      Close,
      Close,
    ]
  );
  Ok(())
}
