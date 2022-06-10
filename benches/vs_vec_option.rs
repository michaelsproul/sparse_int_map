use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sparse_int_map::{DynamicMap, IntMap};
use std::collections::HashMap;
use vec_map::VecMap;

fn bench_insert_int_map(c: &mut Criterion) {
    c.bench_function("int_map_insert_200k", |b| {
        b.iter(|| {
            let n = black_box(200_000);
            let mut map = IntMap::with_capacity(n);
            for i in 0..n {
                map.insert(i, i);
            }
        })
    });
}

fn bench_insert_vec_map(c: &mut Criterion) {
    c.bench_function("vec_map_insert_200k", |b| {
        b.iter(|| {
            let n = black_box(200_000);
            let mut map = VecMap::with_capacity(n);
            for i in 0..n {
                map.insert(i, i);
            }
        })
    });
}

fn bench_insert_dynamic_int_map(c: &mut Criterion) {
    c.bench_function("dynamic_int_map_insert_200k", |b| {
        b.iter(|| {
            let n = black_box(200_000);
            let mut map = DynamicMap::new_int_map(n);
            for i in 0..n {
                map.insert(i, i);
            }
        })
    });
}

fn bench_insert_dynamic_vec_map(c: &mut Criterion) {
    c.bench_function("dynamic_vec_map_insert_200k", |b| {
        b.iter(|| {
            let n = black_box(200_000);
            let mut map = DynamicMap::new_vec_map(n);
            for i in 0..n {
                map.insert(i, i);
            }
        })
    });
}

fn bench_insert_hash_map(c: &mut Criterion) {
    c.bench_function("hash_map_insert_200k", |b| {
        b.iter(|| {
            let n = black_box(200_000);
            let mut map = HashMap::with_capacity(n);
            for i in 0..n {
                map.insert(i, i);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_insert_int_map,
    bench_insert_vec_map,
    bench_insert_dynamic_int_map,
    bench_insert_dynamic_vec_map,
    bench_insert_hash_map,
);
criterion_main!(benches);
