pub mod monkey_map;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Coordinate {
    y: usize,
    x: usize,
}

impl Coordinate {
    pub fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }

    pub fn apply(&self, vec: (i32, i32)) -> Option<Coordinate> {
        if self.y == 0 && vec.0 < 0 || self.x == 0 && vec.1 < 0 {
            return None;
        }

        Some(Coordinate::new(
            (self.y as i32 + vec.0) as usize,
            (self.x as i32 + vec.1) as usize,
        ))
    }
}

pub enum Path {
    Left(u32),
    Right(u32),
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn as_coordinate(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}
