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
    c.bench_function("to_owned 8", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    c.bench_function("to_owned 16", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    c.bench_function("to_owned 32", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    c.bench_function("to_owned 64", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
    c.bench_function("to_owned 128", |b| b.iter(|| str_to_owned(black_box(&src), &mut dst)));
}

fn bench_to_string(c: &mut Criterion) {
    let binding = get_random_string(16);
    let src = binding.as_str();
    let mut dst = String::new();
    c.bench_function("to_string 8", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    c.bench_function("to_string 16", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    c.bench_function("to_string 32", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    c.bench_function("to_string 64", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
    c.bench_function("to_string 128", |b| b.iter(|| str_to_string(black_box(&src), &mut dst)));
}

criterion_group!(benches, bench_to_owned, bench_to_string);
criterion_main!(benches);
