use std::collections::HashMap;

type StarMap = HashMap<String, Vec<String>>;

#[aoc_generator(day6)]
fn gen(input: &str) -> StarMap {
    let vec: Vec<(String, String)> = input.lines()
        .map(|line| line.split(")")
            .map(|string| string.to_string()))
        .map(iter_to_twople)
        .collect();
    let mut map = StarMap::new();
    for (k, v) in vec {
        if let Some(vec) = map.get_mut(&k) {
            vec.push(v)
        } else {
            map.insert(k, vec![v]);
        }
    }
    map
}

#[aoc(day6, part1)]
fn part1(map: &StarMap) -> u32 {
    map.keys().fold(0, |cnt, k| cnt + count_children(map, k))
}

#[aoc(day6, part2)]
fn part2(map: &StarMap) -> u32 {
    let you = &String::from("YOU");
    let san = &String::from("SAN");
    let shared_parent = map.iter()
        .find(|(parent, sats)| {
            is_child(map, you, parent) && is_child(map, san, parent) &&
                sats.iter().all(|sat| !(
                    is_child(map, you, sat) && is_child(map, san, sat)
                ))
        })
        .map(|p| p.0)
        .unwrap();
    to(map, you, shared_parent) + to(map, san, shared_parent)
}

fn count_children(map: &StarMap, k: &str) -> u32 {
    if let Some(vec) = map.get(k) {
        vec.iter()
            .fold(0, |cnt, v| cnt + 1 + count_children(map, v))
    } else {
        0
    }
}

fn is_child(map: &StarMap, child: &String, parent: &str) -> bool {
    if let Some(vec) = map.get(parent) {
        vec.contains(child) ||
            vec.iter()
                .any(|k| is_child(map, child, k))
    } else {
        false
    }
}

fn to(map: &StarMap, child: &String, parent: &str) -> u32 {
    if let Some(vec) = map.get(parent) {
        vec.iter()
            .find(|parent| is_child(map, child, parent))
            .map_or(0, |parent| to(map, child, parent) + 1)
    } else {
        0
    }
}

fn iter_to_twople<T, I>(mut iter: I) -> (T, T)
    where I: Iterator<Item=T> {
    (iter.next().unwrap(), iter.next().unwrap())
}
