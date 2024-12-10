use std::ops::{Add, Sub};

use itertools::Itertools;

pub fn process_d8p1(input: &str) -> i32 {
    let radio_map = Grid::from(input);

    // generate coords with a cartesian product over the width & height
    // filter all coordinates that are *not* a radio frequency
    let radio_stations = (0..radio_map.width())
        .cartesian_product(0..radio_map.height())
        .filter(|coord| radio_map.get(coord.0, coord.1) != '.');

    let mut antinodes: Vec<Coord> = Vec::new();

    // Antinodes are produced by pairs of towers. All pairs of towers is the cartesian_product of the list of towers.
    let station_pairs = radio_stations.clone().cartesian_product(radio_stations);

    /*
       filter out self-comparison. A single tower can't produce an antinode
       filter to match frequencies
       compute antinode position
       write antinodes to the antinode_map

    */
    let antinodes = station_pairs
        .into_iter()
        .filter(|(sta, stb)| sta != stb)
        .filter(|(sta, stb)| radio_map.get(sta.0, sta.1) == radio_map.get(stb.0, stb.1))
        .map(|(sta, stb)| compute_antinode(Coord::from(sta), Coord::from(stb)));

    let mut antinode_map = Grid::empty(radio_map.width(), radio_map.height());
    for antinode in antinodes {
        if let Some(char_in_map) = antinode_map.get_mut(antinode.0, antinode.1) {
            *char_in_map = '#';
        } // else, not in the map. Continue iterating
    }

    // count number of antinodes ('#' symbols in vec)
    let sum = antinode_map
        .letters
        .into_iter()
        .filter(|letter| letter == &'#')
        .count();

    return sum as i32;
}

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    letters: Vec<char>,
}

impl Grid {
    fn empty(width: isize, height: isize) -> Self {
        let mut letters = Vec::new();
        for _ in 0..(width * height) {
            letters.push('.')
        }

        Grid {
            width,
            height,
            letters,
        }
    }
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

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut char> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            let idx = x + y * self.width;
            let letters_len = self.letters.len();
            if let Some(letter) = self.letters.get_mut(idx as usize) {
                return Some(letter);
            } else {
                eprintln!("Failed to get letter at idx: {idx}, coords {x}, {y}");
                eprintln!(
                    "Vec size is {}, Grid size is ({}, {})",
                    letters_len, self.width, self.height
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coord(isize, isize);

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<(isize, isize)> for Coord {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0, value.1)
    }
}

/* Given two radio sources, computes an antinode. Order is important.
   Order is important. It is assumed the points are this way:

       A......
       ...B...
       ......#

   Where 'A' is the first radio, 'B' is the second, and '#' is the antinode.
   If 'B' and 'A' are reversed, the antinode would be in the top-left, instead.

   Panics when the points are the same. Violent failure, instead of error handling!
*/
fn compute_antinode(p1: Coord, p2: Coord) -> Coord {
    if p1 == p2 {
        panic!("You can't use a single tower to get an antinode!");
    }
    p2 + p2 - p1 // I don't have `Mul` implemented, so I'm adding
}

#[cfg(test)]
mod test {
    use crate::input_constants;

    use super::*;

    #[test]
    fn run_part1_real() {
        assert_eq!(214, process_d8p1(input_constants::DAY8))
    }

    const SAMPLE_TEXT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn run_part1_example() {
        assert_eq!(process_d8p1(SAMPLE_TEXT), 14)
    }

    #[test]
    fn test_overlapping_nodes() {
        /*
        The prompt says UNIQUE locations containing an antinode.
        The current algorithm will count two nodes if two different pairs create an antinode in the
        same place. This would not be a unique occurrence.
         */
        let input = "A....
.....
A....
.....
..A.A";
        /* Has antinodes like so:

        A....
        .....
        A....
        .....
        #.A.A

        There is 1 node.
         */
        assert_eq!(1, process_d8p1(input));
    }

    #[test]
    fn test_drop_oob_nodes() {
        let input = "A..
A..
...";
        /* Has antinodes like so. 'x' indicates an out-of-bounds coordinate
         x#xxx
         xA..x
         xA..x
         x#..x
         xxxxx
        There are 2 nodes, but only 1 is in bounds.
         */
        assert_eq!(1, process_d8p1(input));
    }
}
