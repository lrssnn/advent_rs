use criterion::{black_box, criterion_group, criterion_main, Criterion};
extern crate adv_rs;
use adv_rs::y2022::day6::Day6;

fn pt1_bench(c: &mut Criterion) {
    let input: Vec<char> = include_str!("../src/y2022/input6").chars().collect();
    c.bench_function("part 1", |b| b.iter(|| Day6::find_packet_marker(black_box(&input))));
}

fn pt2_bench(c: &mut Criterion) {
    let input: Vec<char> = include_str!("../src/y2022/input6").chars().collect();
    c.bench_function("part 2", |b| b.iter(|| Day6::find_message_marker(black_box(&input))));
}

criterion_group!(benches, pt1_bench, pt2_bench);
criterion_main!(benches);