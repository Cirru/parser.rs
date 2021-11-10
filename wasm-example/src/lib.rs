extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use cirru_parser;

#[wasm_bindgen]
pub fn cirru_to_lisp(code: String) -> String {
  cirru_parser::cirru_to_lisp(&code)
}
