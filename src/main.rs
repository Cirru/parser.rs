use cirru_parser::*;
use serde_json::{Result, Value};

// TODO currenly main only used to debugging, logs in tests are suppressed
fn main() {
  println!("Start");
  parse(String::from("a"));
  println!("End");

  let demo = r#"
      ["TODO", ["c"]]
      "#;
  // println!("{:?}", demo);
  let v: Result<Value> = serde_json::from_str(demo);
  match v {
    Ok(a) => println!("a: {:?}", a),
    Err(e) => println!("Err {:?}", e),
  }
}
