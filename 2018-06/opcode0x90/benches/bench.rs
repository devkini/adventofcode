use std::fs::File;

use aoc::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_part1(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        let f = File::open("input.txt").expect("input.txt not found!");
        let input = get_input(f).unwrap();
        b.iter(|| part1(&input))
    });
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("part2", |b| {
        let f = File::open("input.txt").expect("input.txt not found!");
        let input = get_input(f).unwrap();
        b.iter(|| part2(&input, 10000))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
