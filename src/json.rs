use serde_json::Value;

use crate::types::CirruNode;
use crate::types::CirruNode::*;

/// parse JSON `["a", ["b"]]` into Cirru,
/// only Arrays and Strings are accepted
pub fn from_json_value(x: Value) -> CirruNode {
  match x {
    Value::Array(ys) => {
      let mut r: Vec<CirruNode> = vec![];
      for y in ys {
        r.push(from_json_value(y));
      }
      CirruList(r)
    }
    Value::String(s) => CirruLeaf(s),
    _ => unreachable!("only string and array are expected"),
  }
}

/// parse JSON string `r#"["a", ["b"]]"#` into Cirru,
/// only Arrays and Strings are accepted
pub fn from_json_str(s: &str) -> Result<CirruNode, String> {
  let v: serde_json::Result<Value> = serde_json::from_str(s);
  match v {
    Ok(json) => Ok(from_json_value(json)),
    Err(e) => Err(format!("error: {:?}", e)),
  }
}

/// generates JSON from Cirru Data
pub fn to_json_value(x: CirruNode) -> Value {
  match x {
    CirruLeaf(s) => Value::String(s),
    CirruList(ys) => {
      let mut zs: Vec<Value> = vec![];
      for y in ys {
        zs.push(to_json_value(y));
      }
      Value::Array(zs)
    }
  }
}

/// generates JSON string from Cirru Data
pub fn to_json_str(x: CirruNode) -> String {
  let v = to_json_value(x);
  match serde_json::to_string(&v) {
    Ok(r) => r,
    Err(e) => unreachable!(format!("error: {:?}", e)),
  }
}
