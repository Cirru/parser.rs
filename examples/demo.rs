use cirru_parser::{CirruWriterOptions, format, parse};
// use std::collections::hash_map::DefaultHasher;
use std::fs;
// use std::hash::{Hash, Hasher};

// TODO currenly main only used to debugging, logs in tests are suppressed
fn main() {
  let _demo = String::from(
    r#"
def a 1

defn fib (n)
  if (<= n 2) 1
    +
      fib (dec n)
      fib (- n 2)
  "#,
  );

  let large_demo = "/Users/chenyong/repo/calcit-lang/editor/calcit.cirru";
  // let large_demo = "/Users/chen/repo/calcit-lang/respo-calcit-workflow/js-out/program-ir.cirru";
  // let large_demo = "/Users/chen/repo/calcit-lang/calcit_runner.rs/js-out/program-ir.cirru";
  let content = fs::read_to_string(large_demo).unwrap();

  match parse(&content) {
    Ok(result) => {
      let v = result;
      let writer_options = CirruWriterOptions { use_inline: false };
      let t = format(&v, writer_options).unwrap();
      println!("{}", t.len());
    }
    Err(e) => println!("{e:?}"),
  }

  // println!(
  //   "{} {}",
  //   Cirru::leaf("a") < Cirru::leaf("b"),
  //   Cirru::List(vec![Cirru::leaf("a")]) < Cirru::leaf("b")
  // );

  // let mut hasher = DefaultHasher::new();
  // Cirru::leaf("a").hash(&mut hasher);
  // println!("{:x}", hasher.finish());
}
