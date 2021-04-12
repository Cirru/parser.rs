use cirru_parser::parse;
use cirru_parser::CirruNode::*;

#[test]
fn parse_a() {
  assert_eq!(
    parse(String::from("a")),
    CirruList(vec![CirruList(vec![CirruLeaf(String::from("a"))])])
  );
}
