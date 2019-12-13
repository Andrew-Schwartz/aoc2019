use crate::intcode::Computer;
use std::cmp::max;
use std::collections::HashSet;
use std::iter::once;
use std::ops::Range;

#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<i64> {
    Computer::parse_mem(input)
}

#[aoc(day7, part1)]
fn part1(mem: &Vec<i64>) -> i64 {
    let mut max_sig = 0;

    iter(0_i64..5, |a, b, c, d, e| {
        let mut coma = Computer::init(mem, once(a));
        let mut comb = Computer::init(mem, once(b));
        let mut comc = Computer::init(mem, once(c));
        let mut comd = Computer::init(mem, once(d));
        let mut come = Computer::init(mem, once(e));

        coma.send_all(once(0));
        coma.compute();
        let a_sig = coma.recv().unwrap();

        comb.send_all(once(a_sig));
        comb.compute();
        let b_sig = comb.recv().unwrap();

        comc.send_all(once(b_sig));
        comc.compute();
        let c_sig = comc.recv().unwrap();

        comd.send_all(once(c_sig));
        comd.compute();
        let d_sig = comd.recv().unwrap();

        come.send_all(once(d_sig));
        come.compute();
        let e_sig = come.recv().unwrap();

        max_sig = max(e_sig, max_sig)
    });

    max_sig
}

#[aoc(day7, part2)]
fn part2(mem: &Vec<i64>) -> i64 {
    let mut max_sig = 0;

    iter(5_i64..10, |a, b, c, d, e| {
        let coma = Computer::init(mem, once(a));
        let comb = Computer::init(mem, once(b));
        let comc = Computer::init(mem, once(c));
        let comd = Computer::init(mem, once(d));
        let come = Computer::init(mem, once(e));

        let mut q = vec![coma, comb, comc, comd, come];
        let mut sig = 0;

        while !q.is_empty() {
            let mut com = q.remove(0);

            com.send_all(once(sig));
            com.compute();
            sig = com.recv().unwrap();

            if !com.is_done {
                q.push(com);
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