pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;

pub use d03::Claim;
pub use d06::Point;
pub use d10::Star;
pub use d13::{Cart, Cell};

pub mod wrapper;
pub use wrapper::Wrapper;

use std::collections::{HashMap, HashSet, VecDeque};

pub enum Input {
    Ni32(i32),
    Nusize(usize),
    Vu32(Vec<u32>),
    Vi32(Vec<i32>),
    Vusize(Vec<usize>),
    VDu8(VecDeque<u8>),
    VVch(Vec<Vec<char>>),
    HchVch(HashMap<char, Vec<char>>),
    D03(Vec<Claim>),
    D04(HashMap<u16, Vec<[u16; 60]>>),
    D06((Vec<Point<i32>>, [i32; 4])),
    D10(Vec<Star>),
    D12((HashSet<i64>, HashMap<String, String>)),
    D13((Vec<Cart>, Vec<Vec<Cell>>)),
}

#[derive(Debug)]
pub enum Output {
    None,
    Ni32(i32),
    Ni64(i64),
    Nu16(u16),
    Nu32(u32),
    Nusize(usize),
    S(String),
}

pub enum InputError {
    NoInput,
    InvalidInput,
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InputError::NoInput => write!(f, "No input provided"),
            InputError::InvalidInput => write!(f, "Input is invalid"),
        }
    }
}

impl From<std::io::Error> for InputError {
    fn from(_: std::io::Error) -> Self {
        InputError::NoInput
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(_: std::num::ParseIntError) -> Self {
        InputError::InvalidInput
    }
}

pub type InputResult<T> = std::result::Result<T, InputError>;
