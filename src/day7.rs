use crate::intcode::{Computer, Rxout, Txin};
use std::cmp::max;
use std::collections::HashSet;
use std::iter::once;
use std::ops::Range;

#[aoc_generator(day7)]
fn gen(input: &str) -> Computer {
    input.into()
}

#[allow(dead_code)]
fn send_and_run(com: &mut Computer, txin: &Txin, rxout: &Rxout, sig: i64) -> i64 {
    txin.send(sig).unwrap();
    com.compute();
    rxout.iter().last().unwrap()
}

#[aoc(day7, part1)]
fn part1(com: &Computer) -> i64 {
    let mut max_sig = 0;

    iter(0_i64..5, |a, b, c, d, e| {
        let (mut coma, txina, rxouta) = com.init(once(a));
        let (mut comb, txinb, rxoutb) = com.init(once(b));
        let (mut comc, txinc, rxoutc) = com.init(once(c));
        let (mut comd, txind, rxoutd) = com.init(once(d));
        let (mut come, txine, rxoute) = com.init(once(e));

        let a_sig = send_and_run(&mut coma, &txina, &rxouta, 0);
        let b_sig = send_and_run(&mut comb, &txinb, &rxoutb, a_sig);
        let c_sig = send_and_run(&mut comc, &txinc, &rxoutc, b_sig);
        let d_sig = send_and_run(&mut comd, &txind, &rxoutd, c_sig);
        let e_sig = send_and_run(&mut come, &txine, &rxoute, d_sig);

        max_sig = max(e_sig, max_sig)
    });

    max_sig
}

#[aoc(day7, part2)]
fn part2(com: &Computer) -> i64 {
    let mut max_sig = 0;
    iter(5_i64..10, |a, b, c, d, e| {
        let coma = com.init(once(a));
        let comb = com.init(once(b));
        let comc = com.init(once(c));
        let comd = com.init(once(d));
        let come = com.init(once(e));

        let mut q = vec![coma, comb, comc, comd, come];
        let mut sig = 0;

        while !q.is_empty() {
            let (mut com, txin, rxout) = q.remove(0);

            txin.send(sig).unwrap();
            com.compute();
            sig = rxout.recv().unwrap();

            if !com.is_done {
                q.push((com, txin, rxout));
            }
        };

        max_sig = max(sig, max_sig);
    });

    max_sig
}

fn iter<F>(range: Range<i64>, mut f: F) where F: FnMut(i64, i64, i64, i64, i64) -> () {
    for a in range.clone() {
        for b in range.clone() {
            for c in range.clone() {
                for d in range.clone() {
                    for e in range.clone() {
                        if vec![a, b, c, d, e].iter().collect::<HashSet<_>>().len() != 5 {
                            continue;
                        }
                        f(a, b, c, d, e);
                    }
                }
            }
        }
    }
}