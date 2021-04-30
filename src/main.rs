#[macro_use]
extern crate lazy_static;

use cirru_parser::Cirru;
use cirru_parser::*;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

// TODO currenly main only used to debugging, logs in tests are suppressed
fn main() {
  let demo = String::from(
    r#"
def a 1

defn fib (n)
  if (<= n 2) 1
    +
      fib (dec n)
      fib (- n 2)
  "#,
  );

  let large_demo = "/Users/chen/repo/calcit-lang/runner.rs/src/cirru/calcit-core.cirru";
  let content = fs::read_to_string(large_demo).unwrap();

  match parse(&content) {
    Ok(v) => {
      let writer_options = CirruWriterOptions { use_inline: false };
      println!("{}", format(&v, writer_options).unwrap());
    }
    Err(e) => println!("{:?}", e),
  }

  // println!(
  //   "{} {}",
  //   Cirru::Leaf(String::from("a")) < Cirru::Leaf(String::from("b")),
  //   Cirru::List(vec![Cirru::Leaf(String::from("a"))]) < Cirru::Leaf(String::from("b"))
  // );

  // let mut hasher = DefaultHasher::new();
  // Cirru::Leaf(String::from("a")).hash(&mut hasher);
  // println!("{:x}", hasher.finish());
}
