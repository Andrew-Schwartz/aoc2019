use crate::intcode::Computer;
use std::iter::once;

#[aoc_generator(day9)]
fn gen(input: &str) -> Computer {
    input.into()
}

#[aoc(day9, part1)]
fn part1(com: &Computer) -> String {
    let (mut com, _, rxout) = com.init(once(1));
    com.compute();
    rxout.iter()
        .map(|n| format!("{}, ", n))
        .collect()
}

#[aoc(day9, part2)]
fn part2(com: &Computer) -> String {
    let (mut com, _, rxout) = com.init(once(2));
    com.compute();
    rxout.iter()
        .map(|n| format!("{}, ", n))
        .collect()
}