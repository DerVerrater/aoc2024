use itertools::Itertools;

pub fn process_d4p1(input: &str) -> i32 {
    let grid = Grid::from(input);
    let point_iter = (0..grid.height()).cartesian_product(0..grid.width());
    let dir_list = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut count = 0;
    for (y, x) in point_iter {
        // for each coord, test each direction
        for dir in dir_list {
            if check_direction(&grid, (x, y), dir, "XMAS".chars()) {
                // eprintln!("Matched at ({}, {}) going ({}, {})", x, y, dir.0, dir.1);
                count += 1;
            }
        }
    }

    return count;
}

pub fn process_d4p2(input: &str) -> i32 {
    let grid = Grid::from(input);
    let point_iter = (0..grid.height()).cartesian_product(0..grid.width());
    /* X MAS patterns
    M.S     M.M     S.M     S.S
    .A.     .A.     .A.     .A.
    M.S     S.S     S.M     M.M
     */
    let patterns = [
        Grid::from("M.S\n.A.\nM.S"),
        Grid::from("M.M\n.A.\nS.S"),
        Grid::from("S.M\n.A.\nS.M"),
        Grid::from("S.S\n.A.\nM.M"),
    ];
    let mut count = 0;
    for (y, x) in point_iter {
        for pattern in patterns.iter() {
            if check_kernel(&grid, (x, y), &pattern) {
                count += 1;
            }
        }
    }

    return count;
}

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    letters: Vec<char>,
}

impl Grid {
    /* Get the character at the given coordinates.
       Returns a '.' for anything out of bounds.
    */
    fn get(&self, x: isize, y: isize) -> char {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return '.';
        } else {
            let idx = x + y * self.width;
            if let Some(letter) = self.letters.get(idx as usize) {
                return *letter;
            } else {
                eprintln!("Failed to get letter at idx: {idx}, coords {x}, {y}");
                eprintln!(
                    "Vec size is {}, Grid size is ({}, {})",
                    self.letters.len(),
                    self.width,
                    self.height
                );
                panic!("uh oh");
            }
        }
    }

    fn width(&self) -> isize {
        self.width
    }

    fn height(&self) -> isize {
        self.height
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.find("\n").unwrap();
        let height = value.chars().filter(|&c| c == '\n').count() + 1; // +1 because last line has no \n trailing it.
        Self {
            width: width as isize,
            height: height as isize,
            letters: Vec::from_iter(value.chars().filter(|c| *c != '\n')),
        }
    }
}

/*
Recursively compare elements of a reference input iterator with points in the grid.

Call function with current (x,y), a delta vector. (E.g.: (1, 0) to scan to the right.)
and an iterator yielding the characters of the search pattern.

Each call will check the input coordinate against the reference iterator.
1. Get char from reference iter.
    If empty: return success! We've checked all and they passed.
    Else: ...
2. Compare reference char against grid position
    If match: Recurse!
    Else: return failure.
 */
fn check_direction(
    grid: &Grid,
    current_location: (isize, isize),
    scan_dir: (isize, isize),
    mut comparison_iter: impl Iterator<Item = char>,
) -> bool {
    // get next reference character. If it is empty, then we've passed all tests. Return true!
    if let Some(char_to_find) = comparison_iter.next() {
        let char_on_grid = grid.get(current_location.0, current_location.1);
        if char_on_grid == char_to_find {
            let next_coord = (
                current_location.0 + scan_dir.0,
                current_location.1 + scan_dir.1,
            );
            // dig deeper...
            return check_direction(grid, next_coord, scan_dir, comparison_iter);
        } else {
            // No match!
            return false;
        }
    } else {
        // Good match!
        return true;
    }
}

/*
Iterate pattern grid and compare it against the input grid.

`location` represents the top-left corner to simplify reference frame conversions
 */
fn check_kernel(grid: &Grid, location: (isize, isize), pattern: &Grid) -> bool {
    let point_iter = (0..pattern.height()).cartesian_product(0..pattern.width());
    // (x,y) is pattern space
    // (u,v) is grid space
    for (y, x) in point_iter {
        let (u, v) = (x + location.0, y + location.1);
        let char_on_grid = grid.get(u, v);
        let char_on_pattern = pattern.get(x, y);
        // if chars match (or is the placeholder '.'), partial match. Keep searching.
        if char_on_grid == char_on_pattern || char_on_pattern == '.' {
            continue;
        } else {
            // if they don't match, definite fail.
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod test {
    use crate::input_constants;

    use super::*;

    const SAMPLE_TEXT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn run_part1_example() {
        let expected = 18;
        let result = process_d4p1(SAMPLE_TEXT);
        assert_eq!(expected, result);
    }

    #[test]
    fn run_part1_real() {
        let expected = 2454;
        let result = process_d4p1(input_constants::DAY4);
        assert_eq!(expected, result);
    }

    #[test]
    fn run_part2_example() {
        let expected = 9;
        let result = process_d4p2(SAMPLE_TEXT);
        assert_eq!(expected, result);
    }

    #[test]
    fn run_part2_real() {
        let expected = 1858;
        let result = process_d4p2(input_constants::DAY4);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_kernel_simple_left_to_right() {
        /* Input is left-to-right X MAX
        MES     M.S
        SAS     .A.
        MRS     M.S
         */
        let input = "MES\nSAS\nMRS";
        let pattern = Grid::from("M.S\n.A.\nM.S");
        let result = check_kernel(&Grid::from(input), (0, 0), &pattern);
        assert!(result);
    }
}
