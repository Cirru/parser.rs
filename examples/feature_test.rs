// 验证新的 feature 结构：serde 默认启用，serde-json 可选

use cirru_parser::Cirru;
use serde::{Deserialize, Serialize};

fn main() {
  println!("=== Cirru Parser Feature Test ===");

  // 基础功能（总是可用）
  let data = Cirru::leaf("hello");
  println!("✓ Basic Cirru functionality: {data}");

  // Serde trait 支持（默认启用）
  assert_serde_traits(&data);
  println!("✓ Serde Serialize/Deserialize trait bounds are available");

  // JSON 转换工具（需要 serde-json feature）
  #[cfg(feature = "serde-json")]
  {
    use cirru_parser::{from_json_str, to_json_str};

    let json_str = r#"["hello", ["world", "!"]]"#;
    let cirru = from_json_str(json_str).unwrap();
    let back_to_json = to_json_str(cirru).unwrap();
    println!("✓ JSON conversion utilities available: {back_to_json}");
  }

  #[cfg(not(feature = "serde-json"))]
  {
    println!("○ JSON conversion utilities not available (use --features serde-json to enable)");
  }

  println!("=== Test completed ===");
}

fn assert_serde_traits<T>(_value: &T)
where
  T: Serialize + for<'de> Deserialize<'de>,
{
}
