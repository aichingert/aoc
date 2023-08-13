pub mod beverage_bandits;
pub use crate::day::Point;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Coordinate {
    y: usize,
    x: usize,
}

impl Coordinate {
    fn with_y(self, y: usize) -> Self {
        Self { y, ..self }
    }

    fn with_x(self, x: usize) -> Self {
        Self { x, ..self }
    }

    pub fn adjacent(&self) -> [Coordinate; 4] {
        [
            self.with_y(self.y - 1),
            self.with_x(self.x - 1),
            self.with_x(self.x + 1),
            self.with_y(self.y + 1),
        ]
    }
}

#[derive(Copy, Clone)]
pub struct Unit {
    hp: i16,
    dmg: i16,
    kind: char,
    pos: Coordinate,
}
