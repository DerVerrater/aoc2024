mod day1;
mod day2;
mod day3;
mod input_constants;

use day1::{process_d1p1, process_d1p2};
use day2::{process_d2p1, process_d2p2};
use day3::{process_d3p1, process_d3p2};

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
}
