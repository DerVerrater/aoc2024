pub fn process_d6p1(input: &str) -> i32 {
    let (mut guard, board) = parse(input);
    let mut paces = 0;
    loop {
        if let Some(step) = step_guard(&mut guard, &board) {
            if step {
                paces += 1;
            } else {
                continue;
            }
        } else {
            break;
        }
    }
    paces
}

pub fn process_d6p2(input: &str) -> i32 {
    todo!();
}

struct Board {
    tiles: Vec<Tile>,
    width: isize,
    height: isize,
}

impl Board {
    fn get(&self, x: isize, y: isize) -> Option<&Tile> {
        if x < 0 || x >= self.width || y < 0 || y > self.height {
            return None;
        }

        self.tiles.get(x as usize + (y * self.width) as usize)
    }
}

fn parse(input: &str) -> (Guard, Board) {
    let width = input.find("\n").unwrap() as isize;
    let height = input.chars().filter(|&c| c == '\n').count() as isize + 1; // +1 because last line has no newline trailer
    let mut guard = Guard {
        pos: (0, 0),
        facing: Facing::Up,
    };
    let tiles = input
        .chars()
        .filter(|&c| c != '\n')
        .enumerate()
        .map(|(idx, c)| {
            match c {
                '.' => Tile::Space,    // this is a space
                '#' => Tile::Obstacle, // this is an obstacle
                '^' => {
                    // this is the guard, but the guard is *on* a space.
                    guard = Guard {
                        pos: (idx as isize / width, idx as isize % width),
                        facing: Facing::Up,
                    };
                    Tile::Space
                }
                _ => panic!("Unexpected character in input string! Char: {}", c as i32),
            }
        })
        .collect();
    return (
        guard,
        Board {
            tiles,
            width,
            height,
        },
    )
}

#[derive(Default)]
struct Guard {
    pos: (isize, isize),
    facing: Facing,
}

// Some(bool) indicates that the guard moved. The bool indicates if it moved forward, or just rotated
// None indicates that the guard did not move, because it left the board. Stop moving.
fn step_guard(guard: &mut Guard, board: &Board) -> Option<bool> {
    // get the step direction, then the tile at that spot.
    let next_pos = (
        guard.pos.0 + guard.facing.as_delta().0,
        guard.pos.1 + guard.facing.as_delta().1,
    );
    if let Some(next_tile) = board.get(next_pos.0, next_pos.1) {
        // if there's a tile here, see if we can move
        match next_tile {
            Tile::Space => {
                // Move into empty space, indicate successful move to caller
                guard.pos = next_pos;
                return Some(true);
            }
            Tile::Obstacle => {
                // obstacle. Rotate and stay put. Indicate successful not-move to caller.
                guard.facing = guard.facing.following();
                return Some(false);
            }
        }
    } else {
        // there's no tile, we have left the board. Stop walking.
        return None;
    }
}

// I'm anticipating additional tile types in part 2S
enum Tile {
    Space,
    Obstacle,
}

#[derive(Default)]
enum Facing {
    #[default]
    Right,
    Up,
    Left,
    Down,
}

impl Facing {
    // returns the unit vector for the named direction
    fn as_delta(&self) -> (isize, isize) {
        match self {
            Facing::Right => (1, 0),
            Facing::Up => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Down => (0, -1),
        }
    }

    fn following(&self) -> Self {
        match self {
            Facing::Right => Facing::Down,
            Facing::Up => Facing::Right,
            Facing::Left => Facing::Up,
            Facing::Down => Facing::Left,
        }
    }
}

#[cfg(test)]
mod test {
    use std::os::unix::process;

    use super::*;
    use crate::input_constants;

    const SAMPLE_TEXT: &str = 
"....#.....
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
