use std::ops::{Add, AddAssign, BitAnd, Mul, Sub, BitXor};
use std::cmp::{min, max};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pt(i32, i32);

impl Pt {
    fn mag(&self) -> i32 {
        self.0 + self.1
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Wire {
    start: Pt,
    end: Pt,
}

impl Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pt(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Pt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Pt(self.0 - rhs.0, self.1 - rhs.1)
    }
}

/// cross product
impl Mul for Pt {
    type Output = i32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.1 + self.1 * rhs.0 // 2d cross product sort of
    }
}

/// dot product
impl BitXor for Pt {
    type Output = i32;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl<N> Mul<N> for Pt
    where N: Into<f64> {
    type Output = Pt;

    fn mul(self, rhs: N) -> Self::Output {
        let n = rhs.into();
        Pt(self.0 * n as i32, self.1 * n as i32)
    }
}

impl Wire {
    fn is_vert(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn len(&self) -> i32 {
        (self.start - self.end).mag().abs()
    }

    fn bottom(&self) -> i32 {
        min(self.start.1, self.end.1)
    }

    fn top(&self) -> i32 {
        max(self.start.1, self.end.1)
    }

    fn left(&self) -> i32 {
        min(self.start.0, self.end.0)
    }

    fn right(&self) -> i32 {
        max(self.start.0, self.end.0)
    }

    fn dist_to(&self, Pt(x, y): &Pt) -> Result<u32, u32> {
        if self.left() <= *x && self.right() >= *x &&
            self.bottom() <= *y && self.top() >= *y {
            Ok(((*x - self.start.0).abs() + (*y - self.start.1).abs()) as u32)
        } else {
            Err(self.len() as u32)
        }
    }
}

impl BitAnd for Wire {
    type Output = Option<Pt>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let (slx, srx) = (self.left(), self.right());
        let (sdy, suy) = (self.bottom(), self.top());
        let (rlx, rrx) = (rhs.left(), rhs.right());
        let (rdy, ruy) = (rhs.bottom(), rhs.top());

        if self.is_vert() == rhs.is_vert() {
            None // lol who needs to implement this
        } else {
            if self.is_vert() {
                if (rlx..rrx).contains(&slx) && (sdy..suy).contains(&rdy) {
                    Some(Pt(slx, ruy))
                } else { None }
            } else {
                if (slx..srx).contains(&rlx) && (rdy..ruy).contains(&sdy) {
                    Some(Pt(rlx, suy))
                } else { None }
            }
        }
    }
}

#[aoc_generator(day3)]
fn gen(input: &str) -> (Wires, Wires) {
    let mut lines: Vec<Wires> = input.lines()
        .map(|line| {
            let mut curr = Pt(0, 0);
            line.split(",")
                .map(|s: &str| {
                    let (dir, num_str) = s.split_at(1);
                    let num: i32 = num_str.parse().unwrap();
                    let mv = match dir {
                        "L" => Pt(-num, 0),
                        "R" => Pt(num, 0),
                        "U" => Pt(0, num),
                        "D" => Pt(0, -num),
                        _ => unreachable!(),
                    };
                    let wire = Wire {
                        start: curr,
                        end: curr + mv,
                    };
                    curr += mv;
                    wire
                })
                .collect()
        })
        .collect();
    (lines.remove(0), lines.remove(0))
}

#[aoc(day3, part1)]
fn part1(input: &(Wires, Wires)) -> i32 {
    let (w1, w2) = input;
    w1.iter()
        .copied()
        .flat_map(|wire1| {
            w2.iter()
                .copied()
                .filter_map(move |wire2| wire1 & wire2)
        })
        .filter(|&pt| pt != Pt(0, 0))
        .map(|Pt(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
fn part2(input: &(Wires, Wires)) -> u32 {
    let (w1, w2) = input;
    w1.iter()
        .copied()
        .flat_map(|wire1| {
            w2.iter()
                .copied()
                .filter_map(move |wire2| wire1 & wire2)
        })
        .filter(|&pt| pt != Pt(0, 0))
        .map(|pt| w1.dist_to(&pt) + w2.dist_to(&pt))
        .min()
        .unwrap()
}

type Wires = Vec<Wire>;

trait WiresExt {
    fn dist_to(&self, pt: &Pt) -> u32;
}

impl WiresExt for Wires {
    fn dist_to(&self, pt: &Pt) -> u32 {
        let mut dist = 0;
        for wire in self {
            match wire.dist_to(pt) {
                Ok(d) => return dist + d,
                Err(d) => dist += d,
            }
        }
        unreachable!()
    }
}
