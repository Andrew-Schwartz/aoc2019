use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pt(i32, i32);

#[aoc(day3, part1)]
fn part1(input: &str) -> i32 {
    let mut vec: Vec<&str> = input.split("\n").collect();
    let (w1, w2) = (vec.remove(0), vec.remove(0));

    let (w1, w2) = (hash_map(w1), hash_map(w2));
    w1.iter()
        .filter(|(pt, _)| w2.contains_key(pt))
        .map(|(&pt, _)| pt.0.abs() + pt.1.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> i32 {
    let mut vec: Vec<&str> = input.split("\n").collect();
    let (w1, w2) = (vec.remove(0), vec.remove(0));

    let (w1, w2) = (hash_map(w1), hash_map(w2));
    w1.iter()
        .filter_map(|(pt, i)| w2.get(pt).map(|i2| i + i2))
        .min()
        .unwrap()
}

fn hash_map(wire: &str) -> HashMap<Pt, i32> {
    let mut map = HashMap::new();
    let mut curr_pt = Pt(0, 0);
    let mut count = 0;
    for st in wire.split(",") {
        let (dir, len) = st.split_at(1);
        match dir {
            "D" => {
                for _ in 0..len.parse().unwrap() {
                    curr_pt.1 -= 1;
                    count += 1;
                    map.insert(curr_pt, count);
                }
            }
            "L" => {
                for _ in 0..len.parse().unwrap() {
                    curr_pt.0 -= 1;
                    count += 1;
                    map.insert(curr_pt, count);
                }
            }
            "R" => {
                for _ in 0..len.parse().unwrap() {
                    curr_pt.0 += 1;
                    count += 1;
                    map.insert(curr_pt, count);
                }
            }
            "U" => {
                for _ in 0..len.parse().unwrap() {
                    curr_pt.1 += 1;
                    count += 1;
                    map.insert(curr_pt, count);
                }
            }
            _ => unreachable!(),
        };
    }
    map
}