use criterion::{criterion_group, criterion_main, Criterion};
use day_2::*;

fn criterion_benchmark_part_1(c: &mut Criterion) {
    let input = include_str!("../data/input.txt");

    let mut group = c.benchmark_group("part1");
    group.bench_with_input("part1", input, |b, i| b.iter(|| part1::solution(i)));

    group.finish();
}

fn criterion_benchmark_part_2(c: &mut Criterion) {
    let input = include_str!("../data/input.txt");

    let mut group = c.benchmark_group("part2");
    group.bench_with_input("part2", input, |b, i| b.iter(|| part2::solution(i)));

    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark_part_1,
    criterion_benchmark_part_2
);
criterion_main!(benches);
