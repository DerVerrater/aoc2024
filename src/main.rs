mod input_constants;

use day1_part1::process_d1p1;

fn main() {
    println!("Day 1 result: {}", process_d1p1(input_constants::DAY1));
}

mod day1_part1 {

    pub fn process_d1p1(input: &str) -> i32 {
        let (mut col1, mut col2): (Vec<_>, Vec<_>) = input
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
            .unzip();

        col1.sort();
        col2.sort();

        let element_diffs: i32 = col1
            .iter()
            .zip(col2.iter())
            .map(|(left, right)| (left - right).abs())
            .sum();
        return element_diffs;
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
        fn run_example() {
            assert_eq!(process_d1p1(SAMPLE_TEXT), 11);
        }
    }
}
