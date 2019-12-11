use crate::intcode::Computer;
use std::sync::mpsc::channel;

#[aoc_generator(day9)]
fn gen(input: &str) -> Computer {
    input.into()
}

#[aoc(day9, part1)]
fn part1(com: &Computer) -> String {
    let (txin, rxin) = channel();
    let (txout, rxout) = channel();

    txin.send(1).unwrap();

    let mut com = com.clone_rx_input(rxin);
    com.tx_output(txout);

    com.compute();
    rxout.iter()
        .map(|n| format!("{}, ", n))
        .collect()
}

#[aoc(day9, part2)]
fn part2(com: &Computer) -> String {
    let (txin, rxin) = channel();
    let (txout, rxout) = channel();

    txin.send(2).unwrap();

    let mut com = com.clone_rx_input(rxin);
    com.tx_output(txout);

    com.compute();
    rxout.iter()
        .map(|n| format!("{}, ", n))
        .collect()
}