use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Planet {
    pos: [i32; 3],
    vel: [i32; 3],
}

impl Planet {
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

impl<'a, I: IntoIterator<Item=&'a str>> From<I> for Planet {
    fn from(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        let z = iter.next().unwrap().parse().unwrap();
        Planet {
            pos: [x, y, z],
            vel: [0, 0, 0],
        }
    }
}

#[aoc_generator(day12)]
fn gen(input: &str) -> Vec<Planet> {
    input.lines()
        .map(|line| line
            .split(",")
            .into()
        )
        .collect()
}

#[aoc(day12, part1)]
fn part1(planets: &Vec<Planet>) -> i32 {
    let mut planets = planets.clone();
    for _ in 0..1000 {
        let temp_planets = planets.clone();
        for apply_grav in &mut planets {
            for planet in &temp_planets {
                if *apply_grav == *planet { continue; }

                match apply_grav.pos[0] {
                    x if x < planet.pos[0] => apply_grav.vel[0] += 1,
                    x if x > planet.pos[0] => apply_grav.vel[0] -= 1,
                    _ => {}
                };
                match apply_grav.pos[1] {
                    y if y < planet.pos[1] => apply_grav.vel[1] += 1,
                    y if y > planet.pos[1] => apply_grav.vel[1] -= 1,
                    _ => {}
                };
                match apply_grav.pos[2] {
                    z if z < planet.pos[2] => apply_grav.vel[2] += 1,
                    z if z > planet.pos[2] => apply_grav.vel[2] -= 1,
                    _ => {}
                };
            }
        }
        for mut apply_vel in &mut planets {
            apply_vel.pos[0] += apply_vel.vel[0];
            apply_vel.pos[1] += apply_vel.vel[1];
            apply_vel.pos[2] += apply_vel.vel[2];
        }
    }
    planets.iter()
        .map(|&planet| planet.energy())
        .sum()
}


#[aoc(day12, part2)]
fn part2(planets: &Vec<Planet>) -> usize {
    let x_per = find_period(planets, 0);
    let y_per = find_period(planets, 1);
    let z_per = find_period(planets, 2);

    let xy = lcm(x_per, y_per);
    lcm(xy, z_per)
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}

fn find_period(planets: &Vec<Planet>, coord: usize) -> usize {
    let mut planets: Vec<(i32, i32)> = planets.clone().iter()
        .map(|planet| (planet.pos[coord], planet.vel[coord]))
        .collect();
    let mut states = HashSet::new();

    loop {
        if states.contains(&planets) {
            break;
        } else {
            states.insert(planets.clone());
        }

        let temp_planets = planets.clone();
        for apply_grav in &mut planets {
            for planet in &temp_planets {
                if *apply_grav == *planet { continue; }

                match apply_grav.0 {
                    x if x < planet.0 => apply_grav.1 += 1,
                    x if x > planet.0 => apply_grav.1 -= 1,
                    _ => {}
                };
            }
        }
        for apply_vel in &mut planets {
            apply_vel.0 += apply_vel.1
        }
    }

    states.len()
}