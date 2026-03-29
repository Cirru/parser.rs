// 测试新的 feature 结构：
// - serde feature: 提供 Serialize/Deserialize 实现
// - serde-json feature: 提供 JSON 转换功能

use cirru_parser::Cirru;
use serde::{Deserialize, Serialize};

fn test_basic_functionality() {
  let data = Cirru::leaf("test");
  println!("Basic Cirru functionality works: {data}");
}

fn test_serde_available() {
  let data = Cirru::leaf("test");
  assert_serde_traits(&data);
  println!("Serde available - Serialize/Deserialize trait bounds are satisfied");
}

fn assert_serde_traits<T>(_value: &T)
where
  T: Serialize + for<'de> Deserialize<'de>,
{
}

#[cfg(feature = "serde-json")]
fn test_json_available() {
  use cirru_parser::{from_json_str, to_json_str};

  let json_str = r#"["a", ["b", "c"]]"#;
  let cirru = from_json_str(json_str).unwrap();
  let back_to_json = to_json_str(cirru).unwrap();
  println!("JSON functions available: {back_to_json}");
}

#[cfg(not(feature = "serde-json"))]
fn test_json_available() {
  println!("JSON functions not available");
}

fn main() {
  println!("Testing new feature structure...");
  test_basic_functionality();
  test_serde_available();
  test_json_available();
}
