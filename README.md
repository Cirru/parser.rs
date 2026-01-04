# Cirru Parser

[![Crates.io](https://img.shields.io/crates/v/cirru_parser.svg)](https://crates.io/crates/cirru_parser)
[![Docs.rs](https://docs.rs/cirru_parser/badge.svg)](https://docs.rs/cirru_parser)

This crate provides a parser for the [Cirru](http://cirru.org/) text syntax, an indentation-based syntax that can be used as a replacement for S-Expressions. It's designed to be simple, clean, and easy to read.

For example, this Cirru code:

```cirru
defn fib (x)
  if (<= x 2) 1
    +
      fib $ dec x
      fib $ - x 2
```

is parsed into a tree structure that represents the nested expressions:

```edn
[ ["defn" "fib" [ "x" ]
    [ "if" [ "<=" "x" "2" ] "1"
      [ "+" [ "fib" ["dec" "x"] ] [ "fib" ["-" "x" "2"] ] ]
    ]
] ]
```

## ✨ Improved Error Handling

The parser provides **detailed error messages** with:

- Exact line and column numbers
- Code snippet preview with visual pointer (`^`)
- Context description (e.g., "in string literal", "at line start")
- Escaped special characters in snippets for clarity (shows `\n`, `\t`, etc.)

Example error output:

```
Error: Invalid indentation (odd number: 3)
  at line 2, column 4
  context: checking indentation
  near (escaped): ...defn calculate\n   add 1 2...
```

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cirru_parser = "0.2"
```

### Parsing

The `parse` function returns a `Result` with the parsed tree:

```rust
use cirru_parser::{parse, Cirru};

fn main() {
  let code = "defn main\n  println \"Hello, world!\"";

  match parse(code) {
    Ok(tree) => {
      println!("Parsed: {:?}", tree);

      let expected = vec![
        Cirru::List(vec![
          Cirru::Leaf("defn".into()),
          Cirru::Leaf("main".into()),
          Cirru::List(vec![
            Cirru::Leaf("println".into()),
            Cirru::Leaf("Hello, world!".into()),
          ]),
        ]),
      ];

      assert_eq!(tree, expected);
    }
    Err(e) => {
      eprintln!("Parse error: {}", e);
      // For detailed error display:
      // use cirru_parser::print_error;
      // print_error(&e, Some(code));
    }
  }
}
```

For cleaner error output with context:

```rust
use cirru_parser::{parse, print_error};

fn main() {
  let code = "defn calculate\n   add 1 2";  // Odd indentation (3 spaces)
  
  match parse(code) {
    Ok(tree) => println!("Success: {:?}", tree),
    Err(e) => {
      print_error(&e, Some(code));
      // Output:
      // Error: Invalid indentation (odd number: 3)
      //   at line 2, column 4
      //   context: checking indentation
      //   near (escaped): ...defn calculate\n   add 1 2...
    }
  }
}
```

### Formatting

This crate also provides a `format` function to convert a `Cirru` tree back into a string. You can control the output format with `CirruWriterOptions`.

```rust
use cirru_parser::{parse, format, CirruWriterOptions};

let code = "a (b c)";
let tree = parse(code).unwrap();

let options = CirruWriterOptions { use_inline: true };
let formatted_code = format(&tree, options).unwrap();

assert_eq!(formatted_code, "a (b c)");
```

### Escaping

When creating Cirru code programmatically, you might need to escape strings to ensure they are treated as single leaves, especially if they contain spaces or special characters.

```rust
use cirru_parser::escape_cirru_leaf;

let escaped = escape_cirru_leaf("a b");
assert_eq!(escaped, "\"a b\"");
```

## Features

This crate provides the following features:

**Default features:**

- **serde**: The `Cirru` type implements `Serialize` and `Deserialize` traits by default, allowing integration with any serde-compatible serialization format (bincode, MessagePack, etc.).

**Optional features:**

- **serde-json**: Provides JSON conversion utilities (`from_json_str`, `to_json_str`, etc.) for converting between Cirru structures and JSON.

To use JSON conversion features, add them to your `Cargo.toml`:

```toml
[dependencies]
cirru_parser = { version = "0.2", features = ["serde-json"] }
```

### Examples

```rust
use cirru_parser::Cirru;

// Basic usage (always available)
let data = Cirru::leaf("hello");

// Serde serialization (always available)
use serde_json;
let json = serde_json::to_string(&data).unwrap();

// JSON conversion (requires "serde-json" feature)
#[cfg(feature = "serde-json")]
{
    use cirru_parser::from_json_str;
    let cirru = from_json_str(r#"["a", ["b", "c"]]"#).unwrap();
}
```

## Development

Contributions are welcome! Here's how to get started:

- **Run tests**: `cargo test`
- **Run benchmarks**: `cargo bench`
- **Check for issues**: `cargo clippy`

## License

This project is licensed under the MIT License.
