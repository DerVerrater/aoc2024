mod input_constants;

use day1::{process_d1p1, process_d1p2};

fn main() {
    println!(
        "Day 1 Part 1 result: {}",
        process_d1p1(input_constants::DAY1)
    );
    println!(
        "Day 1 Part 2 result: {}",
        process_d1p2(input_constants::DAY1)
    );
}

mod day1 {
    use std::collections::HashMap;

    fn parse_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
        input
            .split("\n")
            .map(|line| -> (i32, i32) {
                let mut parts = line.split("   ");
                let first = parts.next().expect("Failed to get number 1 in a line");
                let second = parts.next().expect("Failed to get number 2 in a line");

                let first_int = first
                    .parse::<i32>()
                    .expect("Failed to parse number 1 in a line");

                let second_int = second
                    .parse::<i32>()
                    .expect("Failed to parse number 2 in a line");

                return (first_int, second_int);
            })
            .unzip()
    }

    pub fn process_d1p1(input: &str) -> i32 {
        let (mut col1, mut col2): (Vec<_>, Vec<_>) = parse_lists(input);

        col1.sort();
        col2.sort();

        let element_diffs: i32 = col1
            .iter()
            .zip(col2.iter())
            .map(|(left, right)| (left - right).abs())
            .sum();
        return element_diffs;
    }

    pub fn process_d1p2(input: &str) -> i32 {
        let (col1, col2): (Vec<_>, Vec<_>) = parse_lists(input);

        let frequencies = col2.iter().copied().fold(HashMap::new(), |mut map, val| {
            map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
            map
        });

        let sum: i32 = col1
            .into_iter()
            .map(|value| {
                if let Some(freq) = frequencies.get(&value) {
                    return value * freq;
                } else {
                    return 0;
                }
            })
            .sum();

        return sum;
    }

    #[cfg(test)]
    mod day1_tests {
        use super::*;

        const SAMPLE_TEXT: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

        #[test]
        fn run_part1_example() {
            assert_eq!(process_d1p1(SAMPLE_TEXT), 11);
        }

        #[test]
        fn run_part2_example() {
            let result = process_d1p2(SAMPLE_TEXT);
            eprintln!("Got result: {result}");
            assert_eq!(result, 31);
        }
    }
}
