use serde_json::Value;

use cirru_parser::from_json_str;
use cirru_parser::parse;
use cirru_parser::CirruNode::*;

#[test]
fn parse_demo() {
  assert_eq!(parse(String::from("a")), from_json_str(r#"[["a"]]"#),);

  assert_eq!(
    parse(String::from("a b c")),
    from_json_str(r#"[["a", "b", "c"]]"#)
  );

  assert_eq!(
    parse(String::from("a (b) c")),
    from_json_str(r#"[["a", ["b"], "c"]]"#)
  );

  assert_eq!(
    parse(String::from("a (b)\n  c")),
    from_json_str(r#"[["a", ["b"], ["c"]]]"#)
  );
}
