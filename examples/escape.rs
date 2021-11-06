use cirru_parser::{lex, parse, Cirru};

pub fn main() {
  println!("{:?}", parse("a \"b\\u{87DF}\""));
  println!("{:?}", parse("a \"b\\u{87DF}\" c d e f g f"));
}
