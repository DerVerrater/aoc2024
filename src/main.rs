mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;
mod input_constants;

use day1::{process_d1p1, process_d1p2};
use day2::{process_d2p1, process_d2p2};
use day3::{process_d3p1, process_d3p2};
use day4::{process_d4p1, process_d4p2};
use day5::process_d5p1;
use day6::{process_d6p1, process_d6p2};
use day8::process_d8p1;

fn main() {
    println!(
        "Day 1 Part 1 result: {}",
        process_d1p1(input_constants::DAY1)
    );
    println!(
        "Day 1 Part 2 result: {}",
        process_d1p2(input_constants::DAY1)
    );
    println!(
        "Day 2 Part 1 result: {}",
        process_d2p1(input_constants::DAY2)
    );
    println!(
        "Day 2 Part 2 result: {}",
        process_d2p2(input_constants::DAY2)
    );
    println!(
        "Day 3 Part 1 result: {}",
        process_d3p1(input_constants::DAY3)
    );
    println!(
        "Day 3 Part 2 result: {}",
        process_d3p2(input_constants::DAY3)
    );
    println!(
        "Day 4 Part 1 Result: {}",
        process_d4p1(input_constants::DAY4)
    );
    println!(
        "Day 4 Part 2 Result: {}",
        process_d4p2(input_constants::DAY4)
    );
    println!(
        "Day 5 Part 1 Result: {}",
        process_d5p1(input_constants::DAY5)
    );
    println!(
        "Day 6 Part 1 Result: {}",
        process_d6p1(input_constants::DAY6)
    );
    println!(
        "Day 8 Part 1 Result: {}",
        process_d8p1(input_constants::DAY8)
    );
}
