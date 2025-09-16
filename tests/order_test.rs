extern crate cirru_parser;

use cirru_parser::Cirru;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn nodes_order() {
  assert_eq!(Cirru::leaf("a"), Cirru::leaf("a"));

  assert_eq!(Cirru::List(vec![Cirru::leaf("a")]), Cirru::List(vec![Cirru::leaf("a")]));

  assert!(Cirru::leaf("a") < Cirru::leaf("b"));

  assert!(Cirru::List(vec![Cirru::leaf("a")]) < Cirru::List(vec![Cirru::leaf("a"), Cirru::leaf("a")]));

  assert!(Cirru::List(vec![Cirru::leaf("b")]) > Cirru::List(vec![Cirru::leaf("a"), Cirru::leaf("a")]));

  assert!(Cirru::List(vec![Cirru::leaf("a")]) >= Cirru::leaf("b"));
}

#[test]
fn try_hash() {
  let mut hasher = DefaultHasher::new();
  Cirru::leaf("a").hash(&mut hasher);
  let a = hasher.finish();
  println!("{a:x}");

  let mut hasher = DefaultHasher::new();
  Cirru::leaf("a").hash(&mut hasher);
  let b = hasher.finish();
  println!("{b:x}");

  let mut hasher = DefaultHasher::new();
  Cirru::List(vec![Cirru::leaf("a")]).hash(&mut hasher);
  let c = hasher.finish();
  println!("{c:x}");

  let mut hasher = DefaultHasher::new();
  Cirru::List(vec![Cirru::leaf("a")]).hash(&mut hasher);
  let d = hasher.finish();
  println!("{d:x}");

  assert_eq!(a, b);
  assert_ne!(a, c);
  assert_eq!(c, d);
}
