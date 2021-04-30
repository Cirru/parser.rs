## Cirru Parser in Rust

### Usages

Found on [crate](https://crates.io/crates/cirru_parser).

[Rust Docs](https://docs.rs/crate/cirru_parser/).

```bash
cargo install cirru_parser
```

```rs
use cirru_parser::{parse};

parse("defn f (x)\n  x");
```

use writer:

```rs
use cirru_parser::{format, CirruWriterOptions, escape_cirru_leaf}

let writer_options = CirruWriterOptions { use_inline: false };
format(tree, writer_options);

escape_cirru_leaf("a b");
```

### License

MIT
