extern crate cirru_parser;

use std::vec;

use cirru_parser::Cirru;

#[test]
fn test_eq() {
  assert_eq!(Cirru::leaf("a"), Cirru::leaf("a"));
  assert_eq!(Cirru::leaf("a"), "a".into());
  assert_ne!(Cirru::leaf("b"), Cirru::leaf("a"));
  assert_ne!(Cirru::leaf("a"), Cirru::List(vec![Cirru::leaf("a")]),);
  assert_eq!(Cirru::List(vec![Cirru::leaf("a")]), Cirru::List(vec![Cirru::leaf("a")]),);
  assert_eq!(Cirru::List(vec![Cirru::leaf("a")]), (vec!["a"]).into());
}

#[test]
fn test_fmt() {
  let a = Cirru::leaf("na");
  assert_eq!(format!("{}", a), "na");

  let b = a.to_owned();
  let c = Cirru::List(vec![a, b]);
  assert_eq!(format!("{}", c), "(na na)")
}

#[test]
fn test_len() {
  assert_eq!(1, Cirru::leaf("1").len());
  assert_eq!(1, Cirru::List(vec![Cirru::leaf("1")]).len());
}
