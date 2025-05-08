use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;

use cirru_parser::parse;

fn criterion_benchmark(c: &mut Criterion) {
  let large_demo = "/Users/chenyong/repo/calcit-lang/editor/compact.cirru";
  let content = fs::read_to_string(large_demo).unwrap();

  c.bench_function("parse", |b| {
    b.iter(|| {
      let _ = parse(&content);
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
