use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

pub fn simd(c: &mut Criterion) {
    use dg_math::vector::simd::vec4::Vec4;

    fn setup() -> (Vec4, Vec4) {
        (
            Vec4::new(
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            ),
            Vec4::new(
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            ),
        )
    }

    c.bench_function("dot (simd)", |b| {
        b.iter_batched(
            setup,
            |(a, b)| black_box(a.dot(b)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("add (simd)", |b| {
        b.iter_batched(
            setup,
            |(a, b)| black_box(a.add(b)),
            BatchSize::SmallInput,
        )
    });
}

pub fn non_simd(c: &mut Criterion) {
    use dg_math::vector::Vec4;
    
    fn setup() -> (Vec4, Vec4) {
        (
            Vec4::new(
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            ),
            Vec4::new(
                rand::random(),
                rand::random(),
                rand::random(),
                rand::random(),
            ),
        )
    }

    c.bench_function("dot", |b| {
        b.iter_batched(
            setup,
            |(a, b)| black_box(a.dot(b)),
            BatchSize::SmallInput,
        )
    });

    c.bench_function("add", |b| {
        b.iter_batched(
            setup,
            |(a, b)| black_box(a.add(b)),
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, non_simd, simd);
criterion_main!(benches);
