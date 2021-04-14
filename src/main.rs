use cirru_parser::*;

// TODO currenly main only used to debugging, logs in tests are suppressed
fn main() {
  let demo = r#"
def a 1

defn fib (n)
  if (<= n 2) 1
    +
      fib (dec n)
      fib (- n 2)
  "#;

  match parse(String::from(demo)) {
    Ok(v) => {
      println!("{:?}", v);
      let writer_options = CirruWriterOptions { use_inline: false };
      println!("{}", write_cirru(v, writer_options));
    }
    Err(e) => println!("{:?}", e),
  }
}
