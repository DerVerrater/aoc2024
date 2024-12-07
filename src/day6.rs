
pub fn process_d6p1(input: &str) -> i32 {
    todo!();
}

pub fn process_d6p2(input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod test{
    use std::os::unix::process;

    use crate::input_constants;
    use super::*;

    const SAMPLE_TEXT: &str = "....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#...";

    #[test]
    fn run_part1_example() {
        let expected = 41;
        let result = process_d6p1(SAMPLE_TEXT);
        assert_eq!(result, expected);
    }

    #[test]
    fn run_part2_example() {
        todo!("Complete part 1 first. Can't fill out test until we see example.");
    }
}