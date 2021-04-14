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
    Ok(v) => println!("{:?}", v),
    Err(e) => println!("{:?}", e),
  }
}
