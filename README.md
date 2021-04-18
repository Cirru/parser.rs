## Cirru Parser in Rust

### Usages

Found on [crate](https://crates.io/crates/cirru_parser).

[Rust Docs](https://docs.rs/crate/cirru_parser/).

```bash
cargo install cirru_parser
```

```rs
use cirru_parser::{parse};

parse_cirru("defn f (x)\n  x");
```

use writer:

```rs
use cirru_parser::{write_cirru, CirruWriterOptions}

let writer_options = CirruWriterOptions { use_inline: false };
write_cirru(tree, writer_options)
```

### License

MIT
