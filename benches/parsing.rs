use criterion::{Criterion, criterion_group, criterion_main};
use std::fs;
use std::path::Path;

use cirru_parser::parse;

fn criterion_benchmark(c: &mut Criterion) {
  // benchmark for the large file
  let large_demo = "/Users/chenyong/repo/calcit-lang/editor/calcit.cirru";
  if let Ok(content) = fs::read_to_string(large_demo) {
    c.bench_function("parse large file", |b| {
      b.iter(|| {
        let _ = parse(&content).expect("parsed");
      })
    });
  } else {
    println!("Failed to read large demo file, skipping benchmark.");
  }

  // benchmarks for smaller test files
  let paths: Vec<String> = fs::read_dir(Path::new("tests/cirru"))
    .unwrap()
    .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
    .filter(|p| p.ends_with(".cirru"))
    .collect();

  for file_path in paths {
    let content = fs::read_to_string(&file_path).unwrap();
    let name = Path::new(&file_path).file_stem().unwrap().to_str().unwrap();

    c.bench_function(&format!("parse {name}"), |b| {
      b.iter(|| {
        let _ = parse(&content).expect("parsed");
      })
    });
  }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
