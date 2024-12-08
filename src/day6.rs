pub fn process_d6p1(input: &str) -> i32 {
    let (mut guard, mut board) = parse(input);
    
    // Wow, this is dumb.
    // Loop until we leave the board.
    while step_guard(&mut guard, &mut board) {
        continue;
    }

    // iterate the board tiles and count `VisitedSpace`s
    board.tiles.into_iter().filter(|tile| tile == &Tile::VisitedSpace).count() as i32
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

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Tile> {
        if x < 0 || x >= self.width || y < 0 || y > self.height {
            return None;
        }

        self.tiles.get_mut(x as usize + (y * self.width) as usize)
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
                        pos: (idx as isize % width, idx as isize / width),
                        facing: Facing::Up,
                    };
                    Tile::VisitedSpace
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

// Some(bool) indicates that the guard moved. The bool indicates if it is still on the board.
fn step_guard(guard: &mut Guard, board: &mut Board) -> bool {
    // get the step direction, then the tile at that spot.
    let next_pos = (
        guard.pos.0 + guard.facing.as_delta().0,
        guard.pos.1 + guard.facing.as_delta().1,
    );
    if let Some(next_tile) = board.get_mut(next_pos.0, next_pos.1) {
        // if there's a tile here, see if we can move
        match next_tile {
            Tile::Space => {
                // Move into empty space, turn Space into VisitedSpace
                guard.pos = next_pos;
                *next_tile = Tile::VisitedSpace;
            },
            Tile::VisitedSpace => {
                // Move into the empty visited space.
                guard.pos = next_pos;
            }
            Tile::Obstacle => {
                // obstacle. Rotate and stay put.
                guard.facing = guard.facing.following();
            }
        }
        // tile was Some(_), so we're on the board. return true
        return true;
    } else {
        // there's no tile, we have left the board. Stop walking.
        return false;
    }
}

// I'm anticipating additional tile types in part 2S
#[derive(PartialEq)]
enum Tile {
    Space,          // empty space
    VisitedSpace,   // space the guard has visited
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
            Facing::Up => (0, -1),
            Facing::Left => (-1, 0),
            Facing::Down => (0, 1),
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
