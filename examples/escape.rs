use cirru_parser::parse;

pub fn main() {
  println!("{:?}", parse("a \"b\\u{87DF}\""));
  println!("{:?}", parse("a \"b\\u{87DF}\" c d e f g f"));
}
