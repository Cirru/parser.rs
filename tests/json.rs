use serde_json::Value;

use cirru_parser::Cirru;

/// parse JSON `["a", ["b"]]` into Cirru,
/// only Arrays and Strings are accepted
pub fn from_json_value(x: Value) -> Cirru {
  match x {
    Value::Array(ys) => {
      let mut r: Vec<Cirru> = vec![];
      for y in ys {
        r.push(from_json_value(y));
      }
      Cirru::List(r)
    }
    Value::String(s) => Cirru::Leaf(s),
    _ => unreachable!("only string and array are expected"),
  }
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
  match x {
    Cirru::Leaf(s) => Value::String(s),
    Cirru::List(ys) => {
      let mut zs: Vec<Value> = vec![];
      for y in ys {
        zs.push(to_json_value(y));
      }
      Value::Array(zs)
    }
  }
}

/// generates JSON string from Cirru Data
pub fn to_json_str(x: Cirru) -> Result<String, String> {
  let v = to_json_value(x);
  match serde_json::to_string(&v) {
    Ok(r) => Ok(r),
    Err(e) => return Err(format!("error: {:?}", e)),
  }
}
