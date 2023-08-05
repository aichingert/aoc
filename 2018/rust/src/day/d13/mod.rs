pub mod mine_cart_madness;
use crate::day::Point;

#[derive(Clone, Copy, Debug)]
pub struct Cart {
    pub pos: Point<usize>,
    pub dir: Direction,
    pub state: u8,
    pub crashed: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn apply(self, pos: Point<usize>) -> Point<usize> {
        match self {
            Direction::North => (pos.0 - 1, pos.1),
            Direction::South => (pos.0 + 1, pos.1),
            Direction::East => (pos.0, pos.1 + 1),
            Direction::West => (pos.0, pos.1 - 1),
        }
    }
}

#[derive(Copy, Clone)]
pub enum CellType {
    Empty,
    Vertical,
    Horizontal,
    ForwardSlash,
    BackSlash,
    Intersection,
}

impl CellType {
    fn apply(self, dir: Direction, state: &mut u8) -> Direction {
        match self {
            CellType::Vertical | CellType::Horizontal => dir,
            CellType::ForwardSlash => match dir {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
            CellType::BackSlash => match dir {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },
            CellType::Intersection => {
                *state = (*state + 1) % 3;
                match (dir, state) {
                    (dir, 2) => dir,
                    (Direction::North, 0) => Direction::East,
                    (Direction::North, 1) => Direction::West,
                    (Direction::South, 0) => Direction::West,
                    (Direction::South, 1) => Direction::East,
                    (Direction::East, 0) => Direction::South,
                    (Direction::East, 1) => Direction::North,
                    (Direction::West, 0) => Direction::North,
                    (Direction::West, 1) => Direction::South,
                    _ => panic!("invalid state"),
                }
            }
            CellType::Empty => panic!("support error handling for bad grid [later]"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Cell {
    cell_type: CellType,
    occupied: bool,
}
