use crate::day::{Calculation, Input, Loc, Pos, Sensor};
use std::collections::HashSet;

pub fn unwrap_vu32(input: Input) -> Vec<u32> {
    match input {
        Input::Vu32(v) => v,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_vstr(input: Input) -> Vec<String> {
    match input {
        Input::Vstr(s) => s,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_vti32(input: Input) -> Vec<(i32, i32)> {
    match input {
        Input::VTi32(v) => v,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_vtloc(input: Input) -> Vec<(Loc, Loc)> {
    match input {
        Input::VTLoc(v) => v,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_d14(input: Input) -> (HashSet<Pos>, i32) {
    match input {
        Input::D14(d) => d,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_d15(input: Input) -> (Vec<Sensor>, Vec<(i64, i64)>, (i64, i64)) {
    match input {
        Input::D15(d) => d,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_d17(input: Input) -> (Vec<char>, Vec<(char, Vec<Pos>)>) {
    match input {
        Input::D17(d) => d,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn unwrap_d21(input: Input) -> Calculation {
    match input {
        Input::D21(d) => d,
        _ => panic!("Invalid wrapping!"),
    }
}
