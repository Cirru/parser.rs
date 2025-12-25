/*! Test to demonstrate escape_debug in error messages */

use cirru_parser::{parse, print_error};

fn main() {
    println!("🔍 Testing escape_debug in error messages\n");
    println!("{}\n", "═".repeat(70));

    // Test 1: Newline in string
    println!("Test 1: Newline character in string");
    let code1 = "defn test\n  print \"hello\nworld\"";
    println!("Code: {code1:?}\n");
    if let Err(e) = parse(code1) {
        print_error(&e, Some(code1));
    }
    println!("\n{}\n", "─".repeat(70));

    // Test 2: Tab indentation error
    println!("Test 2: Tab character causing indentation issue");
    let code2 = "defn test\n\tabc";  // Tab character
    println!("Code: {code2:?}\n");
    if let Err(e) = parse(code2) {
        print_error(&e, Some(code2));
    }
    println!("\n{}\n", "─".repeat(70));

    // Test 3: Multiple spaces (odd indentation)
    println!("Test 3: Odd number of spaces (visible with escape_debug)");
    let code3 = "defn test\n   abc\n  def";  // 3 spaces
    println!("Code: {code3:?}\n");
    if let Err(e) = parse(code3) {
        print_error(&e, Some(code3));
    }
    println!("\n{}\n", "─".repeat(70));

    // Test 4: Mixed whitespace
    println!("Test 4: Carriage return in string");
    let code4 = "print \"hello\rworld\"";
    println!("Code: {code4:?}\n");
    match parse(code4) {
        Ok(_) => println!("✅ Parsed successfully (\\r is valid in identifier)"),
        Err(e) => print_error(&e, Some(code4)),
    }
    println!("\n{}\n", "─".repeat(70));

    // Test 5: Long line with special chars
    println!("Test 5: Long line with special characters and error");
    let code5 = "defn process-long-text (data)\n  let result \"This is a very long string with \\t tabs and \\n newlines \\x error\"";
    println!("Code: {code5:?}\n");
    if let Err(e) = parse(code5) {
        print_error(&e, Some(code5));
    }
    println!("\n{}\n", "─".repeat(70));

    // Test 6: Unicode with newline
    println!("Test 6: Unicode escape with newline nearby");
    let code6 = "print \"emoji \\u{1F600}\ntext\"";
    println!("Code: {code6:?}\n");
    if let Err(e) = parse(code6) {
        print_error(&e, Some(code6));
    }
    println!("\n{}\n", "─".repeat(70));

    // Test 7: Indentation with spaces visible
    println!("Test 7: Clear space visualization");
    let code7 = "defn main\n  line1\n    line2\n     line3";  // 5 spaces on line3
    println!("Code with explicit spaces: 'defn main\\n  line1\\n    line2\\n     line3'\n");
    if let Err(e) = parse(code7) {
        print_error(&e, Some(code7));
    }

    println!("\n{}\n", "═".repeat(70));
    println!("✨ Now you can clearly see:");
    println!("  • \\n for newlines");
    println!("  • \\t for tabs");
    println!("  • Spaces are preserved and visible");
    println!("  • Special escape sequences are shown");
    println!("  • Context window is ~60 chars (increased from 40)");
}
