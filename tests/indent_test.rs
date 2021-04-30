use cirru_parser::CirruLexItem::*;
use cirru_parser::{lex, resolve_indentations};

#[test]
fn handle_indentation() -> Result<(), String> {
  assert_eq!(
    resolve_indentations(lex("a\nb")?),
    vec![
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemClose,
      LexItemOpen,
      LexItemString(String::from("b")),
      LexItemClose,
    ]
  );
  assert_eq!(
    resolve_indentations(lex("a\n  b\nc")?),
    vec![
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemOpen,
      LexItemString(String::from("b")),
      LexItemClose,
      LexItemClose,
      LexItemOpen,
      LexItemString(String::from("c")),
      LexItemClose,
    ]
  );
  assert_eq!(
    resolve_indentations(lex("a\n  b c\nd")?),
    vec![
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemOpen,
      LexItemString(String::from("b")),
      LexItemString(String::from("c")),
      LexItemClose,
      LexItemClose,
      LexItemOpen,
      LexItemString(String::from("d")),
      LexItemClose,
    ]
  );
  assert_eq!(
    resolve_indentations(lex("a\n    b c\n    d e\n  f")?),
    vec![
      LexItemOpen,
      LexItemString(String::from("a")),
      LexItemOpen,
      LexItemOpen,
      LexItemString(String::from("b")),
      LexItemString(String::from("c")),
      LexItemClose,
      LexItemOpen,
      LexItemString(String::from("d")),
      LexItemString(String::from("e")),
      LexItemClose,
      LexItemClose,
      LexItemOpen,
      LexItemString(String::from("f")),
      LexItemClose,
      LexItemClose,
    ]
  );
  Ok(())
}
