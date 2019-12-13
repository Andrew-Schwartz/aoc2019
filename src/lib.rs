#[allow(non_snake_case)]

#[macro_use]
extern crate aoc_runner_derive;

#[macro_use]
pub mod intcode;
pub mod day1;
pub mod day2;
pub mod day3;  // 4,003.4 us
pub mod day4;
pub mod day5;  // 5,293.1 us
pub mod day6;
pub mod day7;
pub mod day8;  // 26,703.8 us
pub mod day9;
pub mod day10; // 32,469.2 us
pub mod day11;
pub mod day12;
pub mod day13; // 40,557.2 us

aoc_lib! { year = 2019 }
// 109.0267 ms