use Id::*;
use crate::intcode::Computer;
use std::iter::empty;
use itertools::{Itertools, Chunk};
use std::collections::HashSet;

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
            _ => unreachable!(),
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
fn gen(input: &str) -> Computer {
    input.into()
}

#[aoc(day13, part1)]
fn part1(com: &Computer) -> i64 {
    let (mut com, _, rxout) = com.init(empty());

    com.compute();

    rxout.iter()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| if chunk.nth(2).unwrap() == 2 { 1 } else { 0 })
        .sum()
}

#[aoc(day13, part2)]
fn part2(com: &Computer) -> i64 {
    let (mut com, txin, rxout) = com.init(empty());

    com.modify_mem(0, 2);

    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut blocks = HashSet::new();
    let mut score = 0;

    loop {
        com.compute();

        rxout.try_iter()
            .chunks(3)
            .into_iter()
            .map(|chunk| Tile::from(chunk))
            .for_each(|Tile { x, y, id }| match id {
                Block => { blocks.insert((x, y)); }
                Paddle => paddle_x = x,
                Ball => ball_x = x,
                Score(sc) => score = sc,
                Empty => { blocks.remove(&(x, y)); }
                _ => {}
            });

        txin.send(match paddle_x {
            x if x < ball_x => 1,
            x if x > ball_x => -1,
            _ => 0,
        }).unwrap();

        if blocks.is_empty() {
            break score;
        }
    }
}