#[aoc_generator(day4)]
fn gen(input: &str) -> (i32, i32) {
    let range: Vec<i32> = input.split("-")
        .map(|num| num.parse().unwrap())
        .collect();
    (range[0], range[1])
}

fn digits(num: i32) -> [i8; 6] {
    let mut num = num;
    let mut arr = [0; 6];
    for i in 0..6 {
        arr[5 - i] = (num % 10) as i8;
        num /= 10;
    }
    arr
}

#[allow(dead_code)]
fn num(digits: [i8; 6]) -> i32 {
    digits.iter()
        .map(|&digit| digit as i32)
        .enumerate()
        .map(|(i, digit)| digit * 10_i32.pow((5 - i) as u32))
        .sum()
}

#[aoc(day4, part1)]
fn part1(input: &(i32, i32)) -> usize {
    (input.0..input.1)
        .map(|num| digits(num))
        .filter(all_ascending)
        .filter(adj_pairs)
        .count()
}

fn adj_pairs(arr: &[i8; 6]) -> bool {
    arr.windows(2)
        .filter(|it| it[0] == it[1])
        .count() >= 1
}

fn all_ascending(arr: &[i8; 6]) -> bool {
    arr[0] <= arr[1] &&
        arr[1] <= arr[2] &&
        arr[2] <= arr[3] &&
        arr[3] <= arr[4] &&
        arr[4] <= arr[5]
}

#[aoc(day4, part2)]
fn part2(input: &(i32, i32)) -> usize {
    (input.0..input.1)
        .map(|num| digits(num))
        .filter(all_ascending)
        .filter(not_part_of_group)
        .count()
}

fn not_part_of_group(arr: &[i8; 6]) -> bool {
    for i in 1..arr.len() {
        if arr[i - 1] == arr[i] &&
            (i == 1 || arr[i - 2] != arr[i]) &&
            (i == 5 || arr[i + 1] != arr[i])        {
            return true;
        }
    }
    false
}