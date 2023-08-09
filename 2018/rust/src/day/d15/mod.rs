pub mod beverage_bandits;
pub use crate::day::Point;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum CellType {
    Wall,
    Elve,
    Empty,
    Goblin,
}

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    hp: i16,
    dmg: i16,
    pos: Point<usize>,
    is_elve: bool,
}
