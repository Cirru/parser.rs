use cirru_parser::{parse, print_error};

fn main() {
  println!("=== Testing Unicode Escape Error ===\n");

  let code = r#"defn greet
  print "Hello \u{6c49} world""#;

  println!("Code:\n{code}\n");

  match parse(code) {
    Ok(tree) => {
      println!("✓ Parsed successfully: {tree:?}");
    }
    Err(e) => {
      println!("❌ Parse failed as expected!\n");
      print_error(&e, Some(code));
    }
  }
}
