use cirru_parser::Cirru;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn nodes_order() {
  assert_eq!(
    Cirru::Leaf(String::from("a")),
    Cirru::Leaf(String::from("a"))
  );

  assert_eq!(
    Cirru::List(vec![Cirru::Leaf(String::from("a"))]),
    Cirru::List(vec![Cirru::Leaf(String::from("a"))])
  );

  assert_eq!(
    true,
    Cirru::Leaf(String::from("a")) < Cirru::Leaf(String::from("b"))
  );

  assert_eq!(
    true,
    Cirru::List(vec![Cirru::Leaf(String::from("a"))])
      < Cirru::List(vec![
        Cirru::Leaf(String::from("a")),
        Cirru::Leaf(String::from("a"))
      ])
  );

  assert_eq!(
    true,
    Cirru::List(vec![Cirru::Leaf(String::from("b"))])
      > Cirru::List(vec![
        Cirru::Leaf(String::from("a")),
        Cirru::Leaf(String::from("a"))
      ])
  );

  assert_eq!(
    false,
    Cirru::List(vec![Cirru::Leaf(String::from("a"))]) < Cirru::Leaf(String::from("b"))
  );
}

#[test]
fn try_hash() {
  let mut hasher = DefaultHasher::new();
  Cirru::Leaf(String::from("a")).hash(&mut hasher);
  let a = hasher.finish();
  println!("{:x}", a);

  let mut hasher = DefaultHasher::new();
  Cirru::Leaf(String::from("a")).hash(&mut hasher);
  let b = hasher.finish();
  println!("{:x}", b);

  let mut hasher = DefaultHasher::new();
  Cirru::List(vec![Cirru::Leaf(String::from("a"))]).hash(&mut hasher);
  let c = hasher.finish();
  println!("{:x}", c);

  let mut hasher = DefaultHasher::new();
  Cirru::List(vec![Cirru::Leaf(String::from("a"))]).hash(&mut hasher);
  let d = hasher.finish();
  println!("{:x}", d);

  assert_eq!(a, b);
  assert_ne!(a, c);
  assert_eq!(c, d);
}
