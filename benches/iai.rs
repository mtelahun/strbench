use iai::black_box;
use rand::Rng;
use strbench::{str_to_owned, str_to_string};

fn get_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter::<char, _>(rand::distributions::Standard)
        .take(len)
        .collect()
}

fn bench_to_owned_8() {
    let binding = get_random_string(8);
    let src = binding.as_str();
    let mut dst = String::new();
    str_to_owned(black_box(&src), &mut dst)
}

fn bench_to_owned_128() {
    let binding = get_random_string(128);
    let src = binding.as_str();
    let mut dst = String::new();
    str_to_owned(black_box(&src), &mut dst)
}

fn bench_to_string_8() {
    let binding = get_random_string(8);
    let src = binding.as_str();
    let mut dst = String::new();
    str_to_string(black_box(&src), &mut dst);
}

fn bench_to_string_128() {
    let binding = get_random_string(128);
    let src = binding.as_str();
    let mut dst = String::new();
    str_to_string(black_box(&src), &mut dst);
}

iai::main!(
    bench_to_owned_8,
    bench_to_owned_128,
    bench_to_string_8,
    bench_to_string_128
);
