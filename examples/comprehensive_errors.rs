/*! Comprehensive error demonstration with various syntax errors */

use cirru_parser::{parse, print_error};

fn test_case(name: &str, code: &str) {
  println!("╔═══════════════════════════════════════════════════════╗");
  println!("║ {name:<53} ║");
  println!("╚═══════════════════════════════════════════════════════╝");
  println!("\n📝 Code:\n{code}\n");

  match parse(code) {
    Ok(result) => {
      println!("✅ Parse succeeded!");
      println!("\n📊 Result: {result:?}\n");
    }
    Err(e) => {
      println!("❌ Parse failed!\n");
      print_error(&e, Some(code));
      println!();
    }
  }
  println!("\n{}\n", "─".repeat(80));
}

fn main() {
  println!("\n🔍 Cirru Parser - Comprehensive Error Demonstration\n");
  println!("{}\n", "═".repeat(80));

  // Test 1: Success case with warning
  test_case(
    "Test 1: Unicode escape warning (non-fatal)",
    r#"defn greet (name)
  print "Hello \u{1F600} World"
  return name"#,
  );

  // Test 2: Odd indentation (3 spaces)
  test_case(
    "Test 2: Invalid indentation (odd spaces)",
    r#"defn calculate
   add 1 2
  multiply 3 4"#,
  );

  // Test 3: Unclosed string at EOF
  test_case(
    "Test 3: Unclosed string literal",
    r#"defn main
  let x 10
  print "Hello world"#,
  );

  // Test 4: Newline in string
  test_case(
    "Test 4: Newline in string literal",
    r#"defn show-message
  print "This is
a multiline message""#,
  );

  // Test 5: Unexpected closing paren at line start
  test_case(
    "Test 5: Unexpected ) at line start",
    r#"defn calculate (x y)
  let result (add x y
)
  return result"#,
  );

  // Test 6: Invalid escape sequence
  test_case(
    "Test 6: Invalid escape sequence",
    r#"defn format-string
  let escaped "text\xHH\yBad"
  print escaped"#,
  );

  // Test 7: Multiple odd indentations
  test_case(
    "Test 7: Multiple indentation errors",
    r#"defn complex
  if true
     do-something
       nested-call"#,
  );

  // Test 8: Deep nesting with error
  test_case(
    "Test 8: Deep nesting with unclosed string",
    r#"defn process-data (items)
  map items $ fn (item)
    if (valid? item)
      let result (transform item)
        print "Processing: item
      result"#,
  );

  // Test 9: Mixed parentheses and indentation error
  test_case(
    "Test 9: Parentheses with odd indentation",
    r#"defn handler
  (lambda (x)
     add x 1)"#,
  );

  // Test 10: Invalid escape at string end
  test_case("Test 10: Invalid escape at end", r#"print "Hello\""#);

  // Test 11: Unicode warning with other code
  test_case(
    "Test 11: Valid code with Unicode warning",
    r#"defn unicode-test
  let emoji "🎄"
  let escaped "\u{1F384}"
  print emoji
  print escaped"#,
  );

  // Test 12: Empty odd-indented line
  test_case(
    "Test 12: Single space indentation",
    r#"defn test
 x"#,
  );

  // Test 13: Multiple nested errors
  test_case(
    "Test 13: Nested parentheses error",
    r#"defn nested
  ((( incomplete"#,
  );

  // Test 14: Valid complex structure
  test_case(
    "Test 14: Valid complex nested structure (success)",
    r#"defn fibonacci (n)
  if (<= n 1)
    n
    + (fibonacci (- n 1))
      fibonacci (- n 2)"#,
  );

  // Test 15: String with various escapes
  test_case(
    "Test 15: String with valid escapes",
    r#"defn escape-test
  print "Tab:\t Newline:\n Quote:\" Backslash:\\ Single:\'""#,
  );

  // Test 16: Mixed error - string and indentation
  test_case(
    "Test 16: Multiple error types",
    r#"defn mixed-errors
   print "unclosed
  add 1 2"#,
  );

  // Test 17: Very long line with error in middle
  test_case(
    "Test 17: Error in long line",
    r#"defn long-function (param1 param2 param3 param4 param5)
  let very-long-variable-name (calculate param1 param2 param3 "string with \z error" param5)"#,
  );

  println!("\n╔═══════════════════════════════════════════════════════╗");
  println!("║ 🎉 Error Demonstration Complete!                     ║");
  println!("╚═══════════════════════════════════════════════════════╝\n");
}
