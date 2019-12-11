#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<i32> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Vec<i32>) -> i32 {
    input.iter()
        .map(|n| n / 3 - 2)
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &Vec<i32>) -> i32 {
    input.iter()
        .map(|n| {
            let mut t = 0;
            let mut n = *n;
            while n > 0 {
                n = n / 3 - 2;
                if n > 0 { t += n };
            }
            t
        })
        .sum()
}