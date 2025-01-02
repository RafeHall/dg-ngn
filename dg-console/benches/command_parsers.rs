use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dg_console::command;

// fn parser_recursive_descent(s: &str) -> command_parser_combinator::Command {
//     black_box(command_parser_combinator::parse_command(black_box(s))).unwrap().1
// }

fn parser_combinator(s: &str) -> command::Command {
    black_box(command::parse_command(black_box(s))).unwrap()
}

fn benchmark_parsers(c: &mut Criterion) {
    let commands = ["function true false 16.666 5 \"frametime\"", "really_long_function_name_thingy_should_be_easy_to_parse_i_think", "array [true, true, true, true, false, false, false, false]"];
    let mut group = c.benchmark_group("Parsers");
    for (i, command) in commands.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("Parser Combinator", i), command, |b, i| b.iter(|| parser_combinator(i)));
    }
}

criterion_group!(benches, benchmark_parsers);
criterion_main!(benches);