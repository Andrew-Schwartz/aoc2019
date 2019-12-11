#[aoc_generator(day2)]
fn gen(input: &str) -> Vec<i32> {
    input.split(",")
        .map(|n| n.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Vec<i32>) -> i32 {
    let mut mem = input.clone();
    mem[1] = 12;
    mem[2] = 2;
    compute(&mut mem)
}

#[aoc(day2, part2)]
fn part2(input: &Vec<i32>) -> i32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut mem = input.clone();
            mem[1] = noun;
            mem[2] = verb;
            if compute(&mut mem) == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    unreachable!()
}

fn compute(mem: &mut Vec<i32>) -> i32 {
    let mut ptr = 0;
    loop {
        match mem[ptr] {
            99 => break mem[0],
            1 => {
                let a = mem[ptr + 1] as usize;
                let b = mem[ptr + 2] as usize;
                let store = mem[ptr + 3] as usize;
                mem[store] = mem[a] + mem[b];
            }
            2 => {
                let a = mem[ptr + 1] as usize;
                let b = mem[ptr + 2] as usize;
                let store = mem[ptr + 3] as usize;
                mem[store] = mem[a] * mem[b];
            }
            _ => unimplemented!(),
        }
        ptr += 4;
    }
}
