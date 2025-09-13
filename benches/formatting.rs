use criterion::{Criterion, criterion_group, criterion_main};
use std::fs;

use cirru_parser::{CirruWriterOptions, format, parse};

fn criterion_benchmark(c: &mut Criterion) {
  let large_demo = "/Users/chenyong/repo/calcit-lang/editor/compact.cirru";
  let content = fs::read_to_string(large_demo).unwrap();

  let data = parse(&content).expect("parsed");

  c.bench_function("format", |b| {
    b.iter(|| {
      format(&data, CirruWriterOptions { use_inline: true }).expect("formatted");
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
