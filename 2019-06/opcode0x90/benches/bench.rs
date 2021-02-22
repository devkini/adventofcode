use std::{
    fs::File,
    io::{BufReader, Read},
};

use aoc::*;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_part1(c: &mut Criterion) {
    c.bench_function("part1", |b| {
        let f = File::open("input.txt").expect("input.txt not found!");
        let mut buf = String::new();
        BufReader::new(f).read_to_string(&mut buf).unwrap();
        let input = parse_input(buf.as_str());
        b.iter(|| part1(&input))
    });
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("part2", |b| {
        let f = File::open("input.txt").expect("input.txt not found!");
        let mut buf = String::new();
        BufReader::new(f).read_to_string(&mut buf).unwrap();
        let input = parse_input(buf.as_str());
        b.iter(|| part2(&input))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
