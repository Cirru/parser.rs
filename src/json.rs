use serde_json::Value;

use crate::Cirru;

/// parse JSON `["a", ["b"]]` into Cirru,
/// only Arrays and Strings are accepted
pub fn from_json_value(x: Value) -> Cirru {
  serde_json::from_value(x).unwrap()
}

/// parse JSON string `r#"["a", ["b"]]"#` into Cirru,
/// only Arrays and Strings are accepted
pub fn from_json_str(s: &str) -> Result<Cirru, String> {
  let v: serde_json::Result<Value> = serde_json::from_str(s);
  match v {
    Ok(json) => Ok(from_json_value(json)),
    Err(e) => Err(format!("error: {:?}", e)),
  }
}

/// generates JSON from Cirru Data
pub fn to_json_value(x: Cirru) -> Value {
  serde_json::to_value(x).unwrap()
}

/// generates JSON string from Cirru Data
pub fn to_json_str(x: Cirru) -> Result<String, String> {
  let v = to_json_value(x);
  match serde_json::to_string(&v) {
    Ok(r) => Ok(r),
    Err(e) => return Err(format!("error: {:?}", e)),
  }
}
