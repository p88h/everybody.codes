use criterion::{criterion_group, criterion_main, Criterion};

fn read_input(event: i32, quest: i32, part: i32) -> String {
    let path = format!("input/everybody_codes_e{event}_q{quest:02}_p{part}.txt");
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn criterion_benchmark(c: &mut Criterion) {    
    let solutions = everybody_codes::e2025();
    for sol in solutions {
        let input1 = read_input(sol.event, sol.quest, 1);
        let input2 = read_input(sol.event, sol.quest, 2);
        let input3: String = read_input(sol.event, sol.quest, 3);
        let name = format!("{}.{}", sol.event, sol.quest);
        let mut group = c.benchmark_group(&name);
        group.bench_function("part1", |b| {
            b.iter(|| (sol.part1)(&input1))
        });
        group.bench_function("part2", |b| {
            b.iter(|| (sol.part2)(&input2))
        });
        group.bench_function("part3", |b| {
            b.iter(|| (sol.part3)(&input3))
        });
    }     
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);