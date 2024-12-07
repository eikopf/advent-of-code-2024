use aoc_2024::day06::count_distinct_patrol_positions as part_1;
use aoc_2024::day06::count_possible_loops as part_2;
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../input/day06.txt");

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

criterion_group!(day06, part_1_benchmark, part_2_benchmark);
criterion_main!(day06);
