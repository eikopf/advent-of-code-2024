use aoc_2024::day07::total_calibration_result as part_1;
use aoc_2024::day07::total_calibration_result_with_concatenation as part_2;

use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/day07.txt");

pub fn part_1_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| {
        b.iter(|| {
            part_1(INPUT);
        })
    });
}

pub fn part_2_benchmark(c: &mut Criterion) {
    c.bench_function("part 2", |b| {
        b.iter(|| {
            part_2(INPUT);
        })
    });
}

criterion_group!(day07, part_1_benchmark, part_2_benchmark,);

criterion_main!(day07);
