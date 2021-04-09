mod types;

pub fn parse(content: String) -> i32 {
  println!("good");
  // let mut buf = String::new();
  // std::io::stdin().read_line(&mut buf).expect("TODO");

  // println!("a{}", &mut buf)

  println!("{}", content);

  return 1;
}

#[cfg(test)]
mod test_parser {
  use super::parse;

  #[test]
  fn parse_a() {
    assert_eq!(parse(String::from("demo")), 1);
  }
}
