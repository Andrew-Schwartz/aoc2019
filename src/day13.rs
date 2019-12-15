use Id::*;
use crate::intcode::Computer;
use std::iter::empty;
use itertools::{Itertools, Chunk};
use std::collections::HashSet;
use std::hint::unreachable_unchecked;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Id {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(i64),
}

impl<N: Into<i64>> From<N> for Id {
    fn from(n: N) -> Self {
        match n.into() {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => unsafe { unreachable_unchecked() },
//            _ => unreachable!(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Tile {
    x: i64,
    y: i64,
    id: Id,
}

impl<I> From<Chunk<'_, I>> for Tile
    where I: Itertools<Item=i64> {
    fn from(chunk: Chunk<I>) -> Self {
        let vec: Vec<i64> = chunk.collect();
        let x = vec[0];
        let y = vec[1];
        match (x, y) {
            (-1, 0) => Tile {
                x,
                y,
                id: Score(vec[2]),
            },
            (_x, _) => Tile {
                x,
                y,
                id: vec[2].into(),
            }
        }
    }
}

#[aoc_generator(day13)]
fn gen(input: &str) -> Vec<i64> {
    Computer::parse_mem(input)
}

//#[aoc(day13, part1)]
//fn part1(mem: &Vec<i64>) -> i64 {
//    let mut com = Computer::init(mem, empty());
//    com.compute();
//    let borrow_checker = com.recv_all()
//        .chunks(3)
//        .into_iter()
//        .map(|mut chunk| if chunk.nth(2).unwrap() == 2 { 1 } else { 0 })
//        .sum();
//    borrow_checker
//}

#[aoc(day13, part2)]
fn part2(mem: &Vec<i64>) -> i64 {
    let mut com = Computer::init(mem, empty());

    com.mem[0] = 2;

    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut blocks = HashSet::new();
    let mut score = 0;

    loop {
        com.compute();

        com.recv_all()
            .chunks(3)
            .into_iter()
            .map(|chunk| Tile::from(chunk))
            .for_each(|Tile { x, y, id }| match id {
                Block => { blocks.insert((x, y)); }
                Paddle => paddle_x = x,
                Ball => ball_x = x,
                Score(sc) => {
                    println!("{}", sc - score);
                    score = sc
                }
                Empty => { blocks.remove(&(x, y)); }
                _ => {}
            });

        com.send((ball_x - paddle_x).signum());

        if blocks.is_empty() {
            break score;
        }
    }
}