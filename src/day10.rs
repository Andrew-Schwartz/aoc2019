use std::collections::HashSet;
use std::f64::consts::PI;
use std::cmp::Ordering;

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<((i8, i8), u8)> {
    let int: Vec<Vec<u8>> = input.lines()
        .map(|line| line.split("")
            .skip(1)
            .take(line.len())
            .map(|c| if c == "#" { 1 } else { 0 })
            .collect()
        )
        .collect();

    int.iter()
        .enumerate()
        .flat_map(|(y, vec)| vec.iter()
            .enumerate()
            .map(move |(x, ast)| ((x as i8, y as i8), *ast))
        )
        .collect()
}

#[aoc(day10, part1)]
fn part1(map: &Vec<((i8, i8), u8)>) -> usize {
    map.clone().iter()
        .filter(|(_, ast)| *ast == 1)
        .map(|((sx, sy), _)| visible_astrs(map, *sx, *sy).len())
        .max()
        .unwrap()
}

const SX: i8 = 22;
const SY: i8 = 25;

#[aoc(day10, part2)]
fn part2(map: &Vec<((i8, i8), u8)>) -> i32 {
    let mut astrs = map.clone();
    let mut curr_angle = -0.5 * PI - 0.00001;
    let mut count = 0;

    loop {
        let deltas = visible_astrs(&astrs, SX, SY);

        let option = deltas.iter()
            .map(|&(x, y)| ((x, y), (y as f64).atan2(x as f64)))
            .filter(|&(_, angle)| angle > curr_angle)
            .min_by(|&(_, angle1), &(_, angle2)| {
                let diff = angle1 - angle2;
                cmp_f64(diff)
            });

        if let Some(((dx, dy), angle)) = option {
            count += 1;
            let (x, y) = (SX + dx, SY + dy);
            if count == 200 { break 100 * x as i32 + y as i32; };
            let i = astrs.iter()
                .enumerate()
                .filter(|&(_, (_, ast))| *ast == 1)
                .filter(|(_, &(pt, _))| pt != (SX, SY))
                .filter(|(_, &(pt, _))| (pt.0 - SX, pt.1 - SY).simplify() == (x - SX, y - SY))
                .map(|(i, &((x, y), _))| (i, (x - SX, y - SY)))
                .min_by(|&(_, pt1), &(_, pt2)| {
                    let diff = pt1.len() - pt2.len();
                    cmp_f64(diff)
                })
                .map(|(i, _)| i)
                .unwrap();
            astrs.remove(i);
            curr_angle = angle;
        } else {
            curr_angle = -PI;
        }
    }
}

fn visible_astrs(map: &Vec<((i8, i8), u8)>, start_x: i8, start_y: i8) -> HashSet<(i8, i8)> {
    map.iter()
        .filter(|((x, y), _)| !(start_x == *x && start_y == *y))
        .filter_map(|((x, y), ast)| if *ast == 1 { Some((x, y)) } else { None })
        .map(|(&x, &y)| {
            (x - start_x, y - start_y).simplify()
        })
        .collect()
}

fn cmp_f64(diff: f64) -> Ordering {
    match diff {
        _ if diff < 0.0 => Ordering::Less,
        _ if diff > 0.0 => Ordering::Greater,
        _ => Ordering::Equal
    }
}

trait TupleExt {
    fn simplify(&self) -> Self;

    fn len(&self) -> f64;
}

impl TupleExt for (i8, i8) {
    fn simplify(&self) -> Self {
        let gcd = gcd(self.0.abs() as u8, self.1.abs() as u8);
        (self.0 / gcd as i8, self.1 / gcd as i8)
    }

    fn len(&self) -> f64 {
        (self.0 as f64).hypot(self.1 as f64)
    }
}

fn gcd(a: u8, b: u8) -> u8 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}