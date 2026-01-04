/*! Simple example showing how to use the improved error handling */

use cirru_parser::{parse, print_error};

fn main() {
  // Example 1: Successful parse
  println!("=== Example 1: Successful parse ===\n");
  let code1 = r#"defn greet (name)
  print "Hello world""#;

  match parse(code1) {
    Ok(result) => {
      println!("✓ Parse succeeded!");
      println!("\nResult: {result:?}\n");
    }
    Err(e) => {
      print_error(&e, Some(code1));
    }
  }

  // Example 2: Parse error with detailed context
  println!("\n=== Example 2: Parse error with context ===\n");
  let code2 = r#"defn calculate
   add 1 2
  multiply 3 4"#;

  println!("Code:\n{code2}\n");
  match parse(code2) {
    Ok(result) => {
      println!("Parsed: {result:?}");
    }
    Err(e) => {
      eprintln!("❌ Parse failed!\n");
      print_error(&e, Some(code2));
    }
  }

  // Example 3: Simple parse
  println!("\n\n=== Example 3: Simple parse ===\n");

  let code3 = "defn main\n  print \"simple\"";
  match parse(code3) {
    Ok(tree) => println!("✓ Parsed: {tree:?}"),
    Err(e) => eprintln!("❌ Error: {e}"),
  }
}
