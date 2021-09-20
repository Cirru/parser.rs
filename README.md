## Cirru Parser in Rust

### Usages

Found on [crate](https://crates.io/crates/cirru_parser) ![](https://img.shields.io/crates/v/cirru_parser?style=flat-square) .

[Rust Docs](https://docs.rs/crate/cirru_parser/).

```bash
cargo install cirru_parser
```

```rs
use cirru_parser::{parse};

parse("defn f (x)\n  x"); // returns Result<Vec<Cirru>, String>
```

use writer:

```rs
use cirru_parser::{format, CirruWriterOptions, escape_cirru_leaf}

let writer_options = CirruWriterOptions { use_inline: false };
format(tree, writer_options); // tree is Vec<Cirru>

escape_cirru_leaf("a b");
```

### License

MIT
