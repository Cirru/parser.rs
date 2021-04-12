use cirru_parser::CirruNode::{CirruLeaf, CirruList};

#[test]
fn test_eq() {
  assert_eq!(CirruLeaf(String::from("a")), CirruLeaf(String::from("a")));
  assert_ne!(CirruLeaf(String::from("b")), CirruLeaf(String::from("a")));
  assert_ne!(
    CirruLeaf(String::from("a")),
    CirruList(vec![CirruLeaf(String::from("a"))]),
  );
  assert_eq!(
    CirruList(vec![CirruLeaf(String::from("a"))]),
    CirruList(vec![CirruLeaf(String::from("a"))]),
  );
}

#[test]
fn test_fmt() {
  let a = CirruLeaf(String::from("na"));
  assert_eq!(format!("{}", a), "na");

  let b = a.clone();
  let c = CirruList(vec![a, b]);
  assert_eq!(format!("{}", c), "(na, na)")
}

#[test]
fn test_len() {
  assert_eq!(1, CirruLeaf(String::from("1")).len());
  assert_eq!(1, CirruList(vec![CirruLeaf(String::from("1"))]).len());
}
