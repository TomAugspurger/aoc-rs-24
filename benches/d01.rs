use std::fs;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_rs_24::d01;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    let data = d01::parse_input(&input);
    // let result = d01::find_distance(&data);
    c.bench_function("fib 20", |b| b.iter(|| d01::find_distance(black_box(&data))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
