
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pangram::*;

fn set_operation_benchmark(c: &mut Criterion) {
    let sentence = "The quick brown fox jumps over the lazy dog";
    c.bench_function("set_operation", |b| b.iter(|| is_pangram_set_operation(black_box(sentence))));
}

fn array_based_benchmark(c: &mut Criterion) {
    let sentence = "The quick brown fox jumps over the lazy dog";
    c.bench_function("array_based", |b| b.iter(|| is_pangram_array_based(black_box(sentence))));
}

criterion_group!(benches, set_operation_benchmark, array_based_benchmark);
criterion_main!(benches);

