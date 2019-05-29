#![feature(test)]

extern crate test;

use simd_examples::*;

fn million_random_ints() -> Vec<u64> {
    let mut v = Vec::with_capacity(1_000_000);
    for _ in 0..v.capacity() {
        v.push(rand::random());
    }
    v
}

#[bench]
fn bench_portable(b: &mut test::Bencher) {
    let ints = million_random_ints();
    b.iter(|| sum_portable(&ints));
}

#[bench]
fn bench_static(b: &mut test::Bencher) {
    let ints = million_random_ints();
    b.iter(|| sum_avx2_static(&ints));
}

#[bench]
fn bench_dynamic(b: &mut test::Bencher) {
    let ints = million_random_ints();
    b.iter(|| sum_avx2_dynamic(&ints));
}
