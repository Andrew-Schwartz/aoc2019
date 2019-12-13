use std::thread;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
struct Moon {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Moon {
    fn energy(&self) -> i32 {
        self.kin() * self.pot()
    }

    fn kin(&self) -> i32 {
        self.vel.iter()
            .map(|&it| it.abs())
            .sum()
    }

    fn pot(&self) -> i32 {
        self.pos.iter()
            .map(|&it| it.abs())
            .sum()
    }
}

impl<'a, I: IntoIterator<Item=&'a str>> From<I> for Moon {
    fn from(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        let z = iter.next().unwrap().parse().unwrap();
        Moon {
            pos: [x, y, z],
            vel: [0, 0, 0],
        }
    }
}

#[aoc_generator(day12)]
fn gen(input: &str) -> [Moon; 4] {
    let mut moons: [Moon; 4] = [Moon::default(); 4];
    input.lines()
        .map(|line| line
            .split(",")
            .into()
        ).enumerate()
        .for_each(|(i, moon)| moons[i] = moon);
    moons
}

#[aoc(day12, part1)]
fn part1(moons: &[Moon; 4]) -> i32 {
    let mut moons = *moons;
    for _ in 0..1000 {
        let temp_moons = moons;
        for apply_grav in moons.iter_mut() {
            for other in &temp_moons {
                (0..=2).for_each(|i| match apply_grav.pos[i] {
                    pos if pos < other.pos[i] => apply_grav.vel[i] += 1,
                    pos if pos > other.pos[i] => apply_grav.vel[i] -= 1,
                    _ => {}
                })
            }
        }
        for apply_vel in moons.iter_mut() {
            apply_vel.pos[0] += apply_vel.vel[0];
            apply_vel.pos[1] += apply_vel.vel[1];
            apply_vel.pos[2] += apply_vel.vel[2];
        }
    }
    moons.iter()
        .map(|&planet| planet.energy())
        .sum()
}

#[aoc(day12, part2)]
fn part2(moons: &[Moon; 4]) -> usize {
    let moons = *moons;

    let xt = thread::spawn(move || find_period(moons, 0));
    let yt = thread::spawn(move || find_period(moons, 1));
    let zt = thread::spawn(move || find_period(moons, 2));

    let x_per = xt.join().unwrap();
    let y_per = yt.join().unwrap();
    let z_per = zt.join().unwrap();

    lcm(lcm(x_per, y_per), z_per) * 2
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn find_period(moons: [Moon; 4], coord: usize) -> usize {
    let mut moons: [(i32, i32); 4] = moons.iter()
        .map(|moon| (moon.pos[coord], moon.vel[coord]))
        .to_4rray();

    let init_vels = moons;
    let mut count = 0;

    loop {
        count += 1;

        let temp_moons = moons;
        for apply_grav in moons.iter_mut() {
            for planet in &temp_moons {
                match apply_grav.0 {
                    x if x < planet.0 => apply_grav.1 += 1,
                    x if x > planet.0 => apply_grav.1 -= 1,
                    _ => {}
                };
            }
        }
        for apply_vel in moons.iter_mut() {
            apply_vel.0 += apply_vel.1
        }

        if moons.iter().zip(init_vels.iter()).all(|(&(_, mv), &(_, init_v))| mv == init_v) {
            break count;
        }
    }
}

trait To4rray<T> {
    //noinspection ALL
    fn to_4rray(self) -> [T; 4];
}

impl<T, I: Iterator<Item=T>> To4rray<T> for I {
    fn to_4rray(mut self) -> [T; 4] {
        let a = self.next().unwrap();
        let b = self.next().unwrap();
        let c = self.next().unwrap();
        let d = self.next().unwrap();
        [a, b, c, d]
    }
}