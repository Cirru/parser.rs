/*! Demonstrates the improved error handling in Cirru parser */

use cirru_parser::{parse, print_error};

fn main() {
  println!("=== Cirru Parser Error Handling Demo ===\n");

  // Example 1: Valid code with warnings (Unicode escape)
  println!("Example 1: Valid code with Unicode escape warning");
  let code1 = r#"defn main
  print "Hello \u0048orld""#;
  println!("Code:\n{code1}\n");

  match parse(code1) {
    Ok(result) => {
      println!("✓ Parsed successfully!");
      println!("Result: {result:?}\n");
    }
    Err(e) => {
      print_error(&e, Some(code1));
    }
  }

  println!("\n{}\n", "=".repeat(60));

  // Example 2: Odd indentation error
  println!("Example 2: Invalid indentation (odd spaces)");
  let code2 = r#"defn main
   print "hello"
  add 1 2"#;
  println!("Code:\n{code2}\n");

  match parse(code2) {
    Ok(result) => {
      println!("Parsed: {result:?}");
    }
    Err(e) => {
      print_error(&e, Some(code2));
    }
  }

  println!("\n{}\n", "=".repeat(60));

  // Example 3: Unclosed string
  println!("Example 3: Unclosed string literal");
  let code3 = r#"defn main
  print "hello world
  add 1 2"#;
  println!("Code:\n{code3}\n");

  match parse(code3) {
    Ok(result) => {
      println!("Parsed: {result:?}");
    }
    Err(e) => {
      print_error(&e, Some(code3));
    }
  }

  println!("\n{}\n", "=".repeat(60));

  // Example 4: Unexpected closing parenthesis
  println!("Example 4: Unexpected closing parenthesis at line start");
  let code4 = r#"defn main
  (add 1 2
)
  print "done""#;
  println!("Code:\n{code4}\n");

  match parse(code4) {
    Ok(result) => {
      println!("Parsed: {result:?}");
    }
    Err(e) => {
      print_error(&e, Some(code4));
    }
  }

  println!("\n{}\n", "=".repeat(60));

  // Example 5: Invalid escape sequence
  println!("Example 5: Invalid escape sequence");
  let code5 = r#"defn main
  print "hello\xworld""#;
  println!("Code:\n{code5}\n");

  match parse(code5) {
    Ok(result) => {
      println!("Parsed: {result:?}");
    }
    Err(e) => {
      print_error(&e, Some(code5));
    }
  }

  println!("\n{}\n", "=".repeat(60));

  // Example 6: Newline in string
  println!("Example 6: Unexpected newline in string");
  let code6 = r#"defn main
  print "hello
world""#;
  println!("Code:\n{code6}\n");

  match parse(code6) {
    Ok(result) => {
      println!("Parsed: {result:?}");
    }
    Err(e) => {
      print_error(&e, Some(code6));
    }
  }
}
