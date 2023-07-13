use std::collections::HashSet;
use std::fmt;

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d14;
pub mod d15;
use d15::Sensor;
pub mod d17;
pub mod d21;
use d21::Calculation;

pub mod wrapper;

pub type Loc = (u32, u32);
pub type Pos = (i32, i32);

pub enum Input {
    Vu32(Vec<u32>),
    Vstr(Vec<String>),
    VTi32(Vec<(i32, i32)>),
    VTLoc(Vec<(Loc, Loc)>),
    D14((HashSet<Pos>, i32)),
    D15((Vec<Sensor>, Vec<(i64, i64)>, (i64, i64))),
    D17((Vec<char>, Vec<(char, Vec<Pos>)>)),
    D21(Calculation),
}

#[derive(Debug)]
pub enum Output {
    Nu32(u32),
    Ni32(i32),
    Ni64(i64),
    Nusize(usize),
}

pub enum InputError {
    NoInput,
    InvalidInput,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputError::NoInput => write!(f, "No input provided"),
            InputError::InvalidInput => write!(f, "Input is invalid"),
        }
    }
}

impl From<std::io::Error> for InputError {
    fn from(error: std::io::Error) -> Self {
        InputError::NoInput
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(error: std::num::ParseIntError) -> Self {
        InputError::InvalidInput
    }
}

pub type InputResult<T> = std::result::Result<T, InputError>;
