// mutable to acc
pub fn push_to_list<T: Clone>(acc: Vec<T>, xss: Vec<Vec<T>>) -> Vec<T> {
  let mut result = acc;
  for xs in xss {
    for x in xs {
      result.push(x)
    }
  }
  result
}
