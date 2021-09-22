use cirru_parser::Cirru;

#[test]
fn test_eq() {
  assert_eq!(
    Cirru::Leaf(String::from("a")),
    Cirru::Leaf(String::from("a"))
  );
  assert_ne!(
    Cirru::Leaf(String::from("b")),
    Cirru::Leaf(String::from("a"))
  );
  assert_ne!(
    Cirru::Leaf(String::from("a")),
    Cirru::List(vec![Cirru::Leaf(String::from("a"))]),
  );
  assert_eq!(
    Cirru::List(vec![Cirru::Leaf(String::from("a"))]),
    Cirru::List(vec![Cirru::Leaf(String::from("a"))]),
  );
}

#[test]
fn test_fmt() {
  let a = Cirru::Leaf(String::from("na"));
  assert_eq!(format!("{}", a), "na");

  let b = a.to_owned();
  let c = Cirru::List(vec![a, b]);
  assert_eq!(format!("{}", c), "(na na)")
}

#[test]
fn test_len() {
  assert_eq!(1, Cirru::Leaf(String::from("1")).len());
  assert_eq!(1, Cirru::List(vec![Cirru::Leaf(String::from("1"))]).len());
}
