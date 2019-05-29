#![feature(test)]

extern crate test;

use simd_examples::*;

// Manually chosen to make AVX2 look good :)
const INPUT_LEN: usize = 10_000;

fn random_ints() -> Vec<u64> {
    let mut v = Vec::with_capacity(INPUT_LEN);
    for _ in 0..INPUT_LEN {
        v.push(rand::random());
    }
    v
}

#[bench]
fn bench_portable(b: &mut test::Bencher) {
    let ints = random_ints();
    b.iter(|| sum_portable(&ints));
}

#[bench]
fn bench_static(b: &mut test::Bencher) {
    let ints = random_ints();
    b.iter(|| sum_avx2_static(&ints));
}

#[bench]
fn bench_dynamic(b: &mut test::Bencher) {
    let ints = random_ints();
    b.iter(|| sum_avx2_dynamic(&ints));
}
