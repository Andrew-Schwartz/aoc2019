use std::collections::HashMap;
use itertools::Itertools;

type RxnMap = HashMap<String, (SIZE, Vec<(String, SIZE)>)>;
type SIZE = u128;

#[aoc_generator(day14)]
fn gen(input: &str) -> RxnMap {
    input.lines()
        .map(|line| {
            let mut iter = line.split("=>");
            let ingreds = iter.next().unwrap()
                .split(",")
                .map(|ingred| {
                    let mut iter = ingred.split_ascii_whitespace();
                    let num = iter.next().unwrap().parse().unwrap();
                    let name = iter.next().unwrap().to_string();

                    (name, num)
                })
                .collect();
            let mut product = iter.next().unwrap().split_ascii_whitespace();
            let num = product.next().unwrap().parse().unwrap();
            let name = product.next().unwrap().to_string();

            (name, (num, ingreds))
        })
        .collect()
}

#[aoc(day14, part1)]
fn part1(recipes: &RxnMap) -> SIZE {
    let mut store = HashMap::new();

    make("FUEL", 1, &mut store, recipes)
}

#[allow(mutable_borrow_reservation_conflict)]
fn make(goal: &str, goal_num: SIZE, store: &mut HashMap<String, SIZE>, recipes: &RxnMap) -> SIZE {
    if goal == "ORE" {
        store.insert(goal.to_string(), goal_num);
        return goal_num;
    }
    let mut ore_used = 0;
    while *store.get(goal).unwrap_or(&0) < goal_num {
        let tmp = recipes.get(goal).unwrap();
        let ingreds = &tmp.1;
        let num_produced = tmp.0;
        for ingred in ingreds {
            ore_used += make(&ingred.0, ingred.1, store, recipes);
            let curr = store.get(&ingred.0).unwrap();
            store.insert(ingred.0.clone(), curr - ingred.1);
        }
        let curr = *store.get(goal).unwrap_or(&0);
        store.insert(goal.to_string(), curr + num_produced);
    }

    ore_used
}

const TRILLION: SIZE = 1_000_000_000_000;

#[aoc(day14, part2)]
fn part2(recipes: &RxnMap) -> SIZE {
    let mut scaled_recipes = RxnMap::new();
    scale("FUEL", recipes, &mut scaled_recipes);

//    for scaled in &scaled_recipes {
//        println!("{}: {:?}", scaled.0, scaled.1);
//    }

    let mut ore = HashMap::new();
    ore.insert("ORE".to_string(), (1, 1));
    collapse_to_ore("FUEL", &scaled_recipes, &mut ore);

//    println!("{:?}", ore);

    let &(fuel_made, ore_used) = ore.get("FUEL").unwrap();

    println!("made {}, used {}", fuel_made, ore_used);

//    if TRILLION * fuel_made % ore_used != 0 {
//        panic!();
//    }
    TRILLION * fuel_made / ore_used
}

/// recursively replace every
fn collapse_to_ore(goal: &str, scaled: &RxnMap, ore: &mut HashMap<String, (SIZE, SIZE)>) {
    let (qty, ingreds) = scaled.get(goal).unwrap();

    for (ingr_name, _) in ingreds {
        if !ore.contains_key(ingr_name) {
            collapse_to_ore(ingr_name, scaled, ore);
        }
    }

    let total_ore = ingreds.iter()
        .map(|(name, qty)| {
            println!("{} {} takes {} ORE", ore.get(name).unwrap().0, name, ore.get(name).unwrap().1);
            (qty, ore.get(name).unwrap())
        })
        .map(|(&needed_qty, &(prod_qty, ore_qty))| {
            if needed_qty % prod_qty != 0 {
                panic!()
            }
            needed_qty / prod_qty * ore_qty
        })
        .sum();
    ore.insert(goal.to_string(), (*qty as SIZE, total_ore));
}

fn scale(goal: &str, recipes: &RxnMap, scaled_recipes: &mut RxnMap) {
    let tmp = recipes.get(goal).unwrap();
    let orig_goal_qty = tmp.0;
    let ingreds = &tmp.1;

    let mut lcm = Vec::new();

    for (ingred, qty) in ingreds {
        if ingred == "ORE" {
            // ex: 157 ORE => 5 N
            // ingred = ORE, qty = 157, goal = N
            scaled_recipes.insert(goal.to_string(), (orig_goal_qty, vec![("ORE".to_string(), *qty)]));
        } else {
            if !scaled_recipes.contains_key(ingred) {
                scale(ingred, recipes, scaled_recipes);
            }
            let recipe = scaled_recipes.get(ingred).unwrap();
            lcm.push(recipe.0)

            // ex: 12 H, 1 G, 8 P => 9 Q
            // ingred = H, qty = 12, goal = Q, ogq = 9
            // recipe = (5, [177 ORE])
        }
    }

    // ex: 12 H, 1 G, 8 P => 9 Q
    // ORE => 5 H, => 2 G, => 7 P
    // LCM = 70
    // scaled recipe = Q = (630, [H 840, G 70, P 560])
    println!("lcmvec={:?}", &lcm);
    let lcm = lcm.into_iter()
        .fold1(|a, b| lcm2(a, b)).unwrap_or(1);

    let scaled_ingreds = ingreds.iter()
        .map(|(name, qty)| (name.clone(), *qty * lcm))
//        .map(|(name, qty)| (name.clone(), lcm2(*qty, lcm)))
        .collect();

    let scaled_output = orig_goal_qty * lcm;
//    let scaled_output = lcm2(orig_goal_qty, lcm);

    println!("{} {}: {:?}", scaled_output, goal, scaled_ingreds);

    let scaled_recipe = (scaled_output, scaled_ingreds);
    scaled_recipes.insert(goal.to_string(), scaled_recipe);
}

fn lcm2(a: SIZE, b: SIZE) -> SIZE {
    if (a * b) % gcd2(a, b) != 0 {
        panic!();
    }
    (a * b) / gcd2(a, b)
}

fn gcd2(mut a: SIZE, mut b: SIZE) -> SIZE {
    while b > 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}