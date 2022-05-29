extern crate cirru_parser;

use cirru_parser::{parse, Cirru};

#[test]
fn format_tests() -> Result<(), String> {
  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![Cirru::leaf("a"), Cirru::leaf("b"),])]).to_lisp()?,
    String::from("\n(a b)\n")
  );

  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![
      "a".into(),
      "b".into(),
      Cirru::List(vec![Cirru::leaf("c"), Cirru::leaf("d"),])
    ])])
    .to_lisp()?,
    String::from("\n(a b (c d))\n")
  );

  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![
      "a".into(),
      "b".into(),
      Cirru::List(vec!["c".into(), "d".into(), Cirru::List(vec![Cirru::leaf("e"), Cirru::leaf("f"),])])
    ])])
    .to_lisp()?,
    String::from("\n(a b\n  (c d (e f)))\n")
  );

  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![Cirru::leaf("a"), Cirru::leaf("|b"),])]).to_lisp()?,
    String::from("\n(a \"b\")\n")
  );

  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![Cirru::leaf("a"), Cirru::leaf("|b c"),])]).to_lisp()?,
    String::from("\n(a \"b c\")\n")
  );

  Ok(())
}

const COMMENT_1: &str = r#"
(a b
  ;; d
  )
"#;
const COMMENT_2: &str = r#"
(a b
  ;; d
  (e f))
"#;

#[test]
fn comment_tests() -> Result<(), String> {
  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![
      Cirru::leaf("a"),
      Cirru::leaf("b"),
      Cirru::List(vec![Cirru::leaf(";"), Cirru::leaf("d"),])
    ])])
    .to_lisp()?,
    String::from(COMMENT_1)
  );

  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![
      Cirru::leaf("a"),
      Cirru::leaf("b"),
      Cirru::List(vec![Cirru::leaf(";"), Cirru::leaf("d"),]),
      Cirru::List(vec![Cirru::leaf("e"), Cirru::leaf("f"),])
    ])])
    .to_lisp()?,
    String::from(COMMENT_2)
  );

  Ok(())
}

const SIMPLE_CODE: &str = r#"
(func $get_16 (result i32) (i32.const 16))
"#;

const BIGGER_CODE_CIRRU: &str = r#"
func $sum_struct_create
  param $sum_struct_ptr i32
  param $var$a i32
  param $var$b i32
  ;; "c// sum_struct_ptr->a = a;"

  i32.store
    get_local $sum_struct_ptr
    get_local $var$a

  ;; "c// sum_struct_ptr->b = b;"
  i32.store offset=4
    get_local $sum_struct_ptr
    get_local $var$b
"#;

const BIGGER_CODE: &str = r#"
(func $sum_struct_create (param $sum_struct_ptr i32) (param $var$a i32) (param $var$b i32)
  ;; "c// sum_struct_ptr->a = a;"
  (i32.store (get_local $sum_struct_ptr) (get_local $var$a))
  ;; "c// sum_struct_ptr->b = b;"
  (i32.store offset=4 (get_local $sum_struct_ptr) (get_local $var$b)))
"#;

#[test]
fn format_wast_tests() -> Result<(), String> {
  assert_eq!(
    Cirru::List(vec![Cirru::List(vec![
      Cirru::leaf("func"),
      Cirru::leaf("$get_16"),
      vec![Cirru::leaf("result"), "i32".into()].into(),
      Cirru::List(vec![Cirru::leaf("i32.const"), Cirru::leaf("16"),]),
    ])])
    .to_lisp()?,
    String::from(SIMPLE_CODE)
  );

  assert_eq!(Cirru::List(parse(BIGGER_CODE_CIRRU)?).to_lisp()?, String::from(BIGGER_CODE));

  Ok(())
}
