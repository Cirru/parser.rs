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

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cirru_parser = "0.1.33"
```

### Parsing

To parse a string of Cirru code, use the `parse` function. It returns a `Result<Vec<Cirru>, String>`, where `Cirru` is an enum representing either a leaf (string) or a list of `Cirru` expressions.

```rust
use cirru_parser::{parse, Cirru};

fn main() -> Result<(), String> {
  let code = "defn main\n  println \"Hello, world!\"";
  let tree = parse(code)?;

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
  Ok(())
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
cirru_parser = { version = "0.1.33", features = ["serde-json"] }
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
