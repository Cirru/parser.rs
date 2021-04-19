#[macro_use]
extern crate lazy_static;

use cirru_parser::CirruNode::*;
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

  match parse_cirru(content) {
    Ok(v) => {
      let writer_options = CirruWriterOptions { use_inline: false };
      println!("{}", write_cirru(&v, writer_options));
    }
    Err(e) => println!("{:?}", e),
  }

  // println!(
  //   "{} {}",
  //   CirruLeaf(String::from("a")) < CirruLeaf(String::from("b")),
  //   CirruList(vec![CirruLeaf(String::from("a"))]) < CirruLeaf(String::from("b"))
  // );

  // let mut hasher = DefaultHasher::new();
  // CirruLeaf(String::from("a")).hash(&mut hasher);
  // println!("{:x}", hasher.finish());
}
