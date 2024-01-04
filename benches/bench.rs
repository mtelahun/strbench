use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rand::Rng;
use strbench::{str_to_owned, str_to_string};

fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(len)
        .collect()
}

fn bench_to_owned(c: &mut Criterion) {
    let binding = get_random_string(8);
    let src = binding.as_str();
    let mut dst = String::new();
    let mut group = c.benchmark_group("to_owned");
    group.bench_function("8", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    let binding = get_random_string(16);
    let src = binding.as_str();
    group.bench_function("16", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    let binding = get_random_string(32);
    let src = binding.as_str();
    group.bench_function("32", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    let binding = get_random_string(64);
    let src = binding.as_str();
    group.bench_function("64", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    let binding = get_random_string(128);
    let src = binding.as_str();
    group.bench_function("128", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    group.finish();
}

fn bench_to_string(c: &mut Criterion) {
    let binding = get_random_string(8);
    let src = binding.as_str();
    let mut dst = String::new();
    let mut group = c.benchmark_group("to_string");
    group.bench_function("8", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    let binding = get_random_string(16);
    let src = binding.as_str();
    group.bench_function("16", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    let binding = get_random_string(32);
    let src = binding.as_str();
    group.bench_function("32", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    let binding = get_random_string(64);
    let src = binding.as_str();
    group.bench_function("64", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    let binding = get_random_string(128);
    let src = binding.as_str();
    group.bench_function("128", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    group.finish();
}

criterion_group!(benches, bench_to_owned, bench_to_string);
criterion_main!(benches);
