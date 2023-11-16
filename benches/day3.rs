use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate adv_rs;
use adv_rs::y2022::day3::Day3;
use adv_rs::day::Day;

fn pt1_bench(c: &mut Criterion) {
    let input = include_str!("../src/y2022/input3");
    c.bench_function("2022/3/1", |b| b.iter(|| Day3::new_with_input(black_box(input)).part1()));
}

fn pt2_bench(c: &mut Criterion) {
    let mut day = Day3::new();
    c.bench_function("2022/3/2", |b| b.iter(|| day.part2()));
}

criterion_group!(day3, pt1_bench, pt2_bench);
criterion_main!(day3);