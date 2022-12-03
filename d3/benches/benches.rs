use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d3::*;

pub fn p1_benchmark(c: &mut Criterion) {
    c.bench_function("p1_btreeset", |b| b.iter(|| p1_btreeset(black_box(REAL))));
    c.bench_function("p1_hashset_defaulthash", |b| {
        b.iter(|| p1_hashset_defaulthash(black_box(REAL)))
    });
    c.bench_function("p1_hashset_identityhash", |b| {
        b.iter(|| p1_hashset_identityhash(black_box(REAL)))
    });
    c.bench_function("p1_hashset_identityhash_manual", |b| {
        b.iter(|| p1_hashset_identityhash_manual(black_box(REAL)))
    });

    c.bench_function("p1_array_filter_byteset", |b| {
        b.iter(|| p1_array_filter_byteset(black_box(REAL)))
    });

    c.bench_function("p1_array_filter_byteset_unsafe", |b| {
        b.iter(|| p1_array_filter_byteset_unsafe(black_box(REAL)))
    });
}

pub fn p2_benchmark(c: &mut Criterion) {
    c.bench_function("p2_btreeset", |b| b.iter(|| p2_btreeset(black_box(REAL))));
    c.bench_function("p2_hashset_defaulthash", |b| {
        b.iter(|| p2_hashset_defaulthash(black_box(REAL)))
    });
    c.bench_function("p2_hashset_identityhash", |b| {
        b.iter(|| p2_hashset_identityhash(black_box(REAL)))
    });
    c.bench_function("p2_hashset_identityhash_manual", |b| {
        b.iter(|| p2_hashset_identityhash_manual(black_box(REAL)))
    });

    c.bench_function("p2_array_filter_byteset", |b| {
        b.iter(|| p2_array_filter_byteset(black_box(REAL)))
    });
    c.bench_function("p2_array_filter_byteset_2", |b| {
        b.iter(|| p2_array_filter_byteset_2(black_box(REAL)))
    });
    c.bench_function("p2_array_filter_byteset_unsafe", |b| {
        b.iter(|| p2_array_filter_byteset_unsafe(black_box(REAL)))
    });
}

criterion_group!(benches, p1_benchmark, p2_benchmark);
criterion_main!(benches);
