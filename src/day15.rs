use crate::intcode::Computer;
use std::io::empty;
use std::collections::HashMap;

#[aoc_generator(day15)]
fn gen(input: &str) -> Vec<i64> {
    Computer::parse_mem(input)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pt(i32, i32);

#[aoc(day15, part1)]
fn part1(mem: &Vec<i64>) -> u32 {
    let mut com = Computer::init(mem, empty());

    let mut board = HashMap::new();
    let mut droid = Pt(0, 0);


}