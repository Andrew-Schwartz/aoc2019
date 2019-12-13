use crate::intcode::Computer;
use std::iter::once;

#[aoc_generator(day5)]
fn gen(input: &str) -> Vec<i64> {
    Computer::parse_mem(input)
}

#[aoc(day5, part1)]
fn part1(mem: &Vec<i64>) -> String {
    let mut com = Computer::init(mem, once(1));

    com.compute();

    com.recv_all()
        .map(|n| format!("{}, ", n))
        .collect()
}

#[aoc(day5, part2)]
fn part2(mem: &Vec<i64>) ->  String {
    let mut com = Computer::init(mem, once(5));
    com.compute();
    com.recv_all()
        .map(|n| format!("{}, ", n))
        .collect()
}
