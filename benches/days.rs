use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("days");

    group.bench_function("01", |b| b.iter(|| advent_of_code_2024::day01::solve()));
    group.bench_function("02", |b| b.iter(|| advent_of_code_2024::day02::solve()));
    group.bench_function("03", |b| b.iter(|| advent_of_code_2024::day03::solve()));
    group.bench_function("04", |b| b.iter(|| advent_of_code_2024::day04::solve()));
    group.bench_function("05", |b| b.iter(|| advent_of_code_2024::day05::solve()));
    group.bench_function("06", |b| b.iter(|| advent_of_code_2024::day06::solve()));
    group.bench_function("07", |b| b.iter(|| advent_of_code_2024::day07::solve()));
    group.bench_function("08", |b| b.iter(|| advent_of_code_2024::day08::solve()));
    group.bench_function("09", |b| b.iter(|| advent_of_code_2024::day09::solve()));
    group.bench_function("10", |b| b.iter(|| advent_of_code_2024::day10::solve()));
    group.bench_function("11", |b| b.iter(|| advent_of_code_2024::day11::solve()));
    group.bench_function("12", |b| b.iter(|| advent_of_code_2024::day12::solve()));
    group.bench_function("13", |b| b.iter(|| advent_of_code_2024::day13::solve()));
    group.bench_function("14", |b| b.iter(|| advent_of_code_2024::day14::solve()));
    group.bench_function("15", |b| b.iter(|| advent_of_code_2024::day15::solve()));
    group.bench_function("16", |b| b.iter(|| advent_of_code_2024::day16::solve()));
    group.bench_function("17", |b| b.iter(|| advent_of_code_2024::day17::solve()));
    group.bench_function("18", |b| b.iter(|| advent_of_code_2024::day18::solve()));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
