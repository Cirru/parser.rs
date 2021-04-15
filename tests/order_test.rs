use cirru_parser::CirruNode::*;
use cirru_parser::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn nodes_order() {
  assert_eq!(CirruLeaf(String::from("a")), CirruLeaf(String::from("a")));

  assert_eq!(
    CirruList(vec![CirruLeaf(String::from("a"))]),
    CirruList(vec![CirruLeaf(String::from("a"))])
  );

  assert_eq!(
    true,
    CirruLeaf(String::from("a")) < CirruLeaf(String::from("b"))
  );

  assert_eq!(
    true,
    CirruList(vec![CirruLeaf(String::from("a"))])
      < CirruList(vec![
        CirruLeaf(String::from("a")),
        CirruLeaf(String::from("a"))
      ])
  );

  assert_eq!(
    true,
    CirruList(vec![CirruLeaf(String::from("b"))])
      > CirruList(vec![
        CirruLeaf(String::from("a")),
        CirruLeaf(String::from("a"))
      ])
  );

  assert_eq!(
    false,
    CirruList(vec![CirruLeaf(String::from("a"))]) < CirruLeaf(String::from("b"))
  );
}

#[test]
fn try_hash() {
  let mut hasher = DefaultHasher::new();
  CirruLeaf(String::from("a")).hash(&mut hasher);
  let a = hasher.finish();
  println!("{:x}", a);

  let mut hasher = DefaultHasher::new();
  CirruLeaf(String::from("a")).hash(&mut hasher);
  let b = hasher.finish();
  println!("{:x}", b);

  let mut hasher = DefaultHasher::new();
  CirruList(vec![CirruLeaf(String::from("a"))]).hash(&mut hasher);
  let c = hasher.finish();
  println!("{:x}", c);

  let mut hasher = DefaultHasher::new();
  CirruList(vec![CirruLeaf(String::from("a"))]).hash(&mut hasher);
  let d = hasher.finish();
  println!("{:x}", d);

  assert_eq!(a, b);
  assert_ne!(a, c);
  assert_eq!(c, d);
}
