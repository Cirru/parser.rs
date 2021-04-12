use cirru_parser::*;

// TODO currenly main only used to debugging, logs in tests are suppressed
fn main() {
  println!("Start");
  parse(String::from("a"));
  println!("End");
}
