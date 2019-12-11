use crate::intcode::Computer;
use std::iter::empty;
use std::collections::HashMap;
use std::ops::{AddAssign, Add};
use crate::day11::Heading::*;
use std::cmp::{min, max};

#[aoc_generator(day11)]
fn gen(input: &str) -> Computer {
    input.into()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
struct Pt {
    x: i32,
    y: i32,
}

impl From<Heading> for Pt {
    fn from(heading: Heading) -> Self {
        match heading {
            Up => Pt { x: 0, y: 1 },
            Down => Pt { x: 0, y: -1 },
            Left => Pt { x: -1, y: 0 },
            Right => Pt { x: 1, y: 0 },
        }
    }
}

impl Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pt { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}


impl Add<Heading> for Pt {
    type Output = Pt;

    fn add(self, rhs: Heading) -> Self::Output {
        self + Pt::from(rhs)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

impl AddAssign for Heading {
    fn add_assign(&mut self, rhs: Self) {
        *self = match rhs {
            Left => match self {
                Up => Left,
                Down => Right,
                Left => Down,
                Right => Up,
            },
            Right => match self {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            },
            fail => unreachable!("tried to turn {:?}", fail)
        }
    }
}

struct Robot {
    brain: Computer,
    pos: Pt,
    heading: Heading,
}

impl Robot {
    fn new(brain: Computer) -> Self {
        Robot {
            brain,
            pos: Pt::default(),
            heading: Heading::Up,
        }
    }

    fn compute(mut self) -> Self {
        self.brain = self.brain.compute();
        self
    }

    fn walk(&mut self) {
        self.pos = self.pos + self.heading;
    }
}

#[aoc(day11, part1)]
fn part1(com: &Computer) -> usize {
    let (com, txin, rxout) = com.init(empty());

    let mut panels = HashMap::<Pt, bool>::new();
    let mut robot = Robot::new(com);

    loop {
        let cam_val = *panels.get(&robot.pos).unwrap_or(&false);
        txin.send(if cam_val { 1 } else { 0 }).unwrap();
        robot = robot.compute();

        let color = match rxout.try_recv() {
            Ok(color_code) => color_code == 1,
            Err(_) => break,
        };
        let turn = match rxout.try_recv().unwrap() {
            0 => Heading::Left,
            1 => Heading::Right,
            fail => unreachable!("turn recv'd {}", fail)
        };

        panels.insert(robot.pos, color);
        robot.heading += turn;

        robot.walk();
    }

    panels.len()
}

#[aoc(day11, part2)]
fn part2(com: &Computer) -> String {
    let (com, txin, rxout) = com.init(empty());

    let mut panels = HashMap::<Pt, bool>::new();
    panels.insert(Pt::default(), true);
    let mut robot = Robot::new(com);

    let (mut min_x, mut max_x) = (0, 0);
    let (mut min_y, mut max_y) = (0, 0);

    loop {
        let cam_val = *panels.get(&robot.pos).unwrap_or(&false);
        txin.send(if cam_val { 1 } else { 0 }).unwrap();
        robot = robot.compute();

        let color = match rxout.try_recv() {
            Ok(color_code) => color_code == 1,
            Err(_) => break,
        };
        let turn = match rxout.try_recv().unwrap() {
            0 => Heading::Left,
            1 => Heading::Right,
            fail => unreachable!("turn recv'd {}", fail)
        };

        panels.insert(robot.pos, color);
        robot.heading += turn;

        robot.walk();

        let Pt { x, y } = robot.pos;
        min_x = min(x, min_x);
        min_y = min(y, min_y);
        max_x = max(x, max_x);
        max_y = max(y, max_y);
    }

    let mut string = String::from("\n");

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let col = panels.get(&Pt { x, y })
                .filter(|&&white| white)
                .map(|_| '█')
                .unwrap_or(' ');
            string.push(col);
        }
        string.push('\n');
    }
    string
}