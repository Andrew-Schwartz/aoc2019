use crate::intcode::Computer;
use std::sync::mpsc::channel;

#[aoc_generator(day5)]
fn gen(input: &str) -> Computer {
    input.into()
}

#[aoc(day5, part1)]
fn part1(input: &Computer) -> String {
    let (txin, rxin) = channel();
    let (txout, rxout) = channel();

    txin.send(1).unwrap();

    let mut com = input.clone_rx_input(rxin);
    com.tx_output(txout);

    com.compute();
    rxout.iter()
        .map(|n| format!("{}, ", n))
        .collect()
}

#[aoc(day5, part2)]
fn part2(input: &Computer) ->  String {
    let (txin, rxin) = channel();
    let (txout, rxout) = channel();

    txin.send(5).unwrap();

    let mut com = input.clone_rx_input(rxin);
    com.tx_output(txout);

    com.compute();
    rxout.iter()
        .map(|n| format!("{}, ", n))
        .collect()
}
