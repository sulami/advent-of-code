use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::random;
use std::iter::repeat_with;

fn digits_string(n: u64) -> u32 {
    n.to_string().len() as u32
}

fn digits_match(n: u64) -> u32 {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1_000..=9_999 => 4,
        10_000..=99_999 => 5,
        100_000..=999_999 => 6,
        1_000_000..=9_999_999 => 7,
        10_000_000..=99_999_999 => 8,
        100_000_000..=999_999_999 => 9,
        1_000_000_000..=9_999_999_999 => 10,
        10_000_000_000..=99_999_999_999 => 11,
        100_000_000_000..=999_999_999_999 => 12,
        1_000_000_000_000..=9_999_999_999_999 => 13,
        10_000_000_000_000..=99_999_999_999_999 => 14,
        100_000_000_000_000..=999_999_999_999_999 => 15,
        1_000_000_000_000_000..=9_999_999_999_999_999 => 16,
        10_000_000_000_000_000..=99_999_999_999_999_999 => 17,
        100_000_000_000_000_000..=999_999_999_999_999_999 => 18,
        1_000_000_000_000_000_000..=9_999_999_999_999_999_999 => 19,
        10_000_000_000_000_000_000..=u64::MAX => 20,
    }
}

fn digits_log(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn digits_match_log(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => n.ilog10() + 1,
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("digits");

    for zero in ["zero", "non-zero"] {
        let input = match zero {
            "zero" => 0,
            "non-zero" => random::<u64>().saturating_add(1),
            _ => unreachable!(),
        };
        group.bench_function(BenchmarkId::new("match", zero), |b| {
            b.iter(|| digits_match(black_box(input)))
        });
        group.bench_function(BenchmarkId::new("log", zero), |b| {
            b.iter(|| digits_log(black_box(input)))
        });
        group.bench_function(BenchmarkId::new("match_log", zero), |b| {
            b.iter(|| digits_match_log(black_box(input)))
        });
        // This is quite slow, omit to zoom in graphs.
        // group.bench_function(BenchmarkId::new("string", zero), |b| {
        //     b.iter(|| digits_string(black_box(input)))
        // });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
