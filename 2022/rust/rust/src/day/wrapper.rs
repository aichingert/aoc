use crate::day::{Input, Loc};

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
