use crate::day::Input;

pub fn wrapper_vu32(input: Input) -> Vec<u32> {
    match input {
        Input::Vu32(v) => v,
        _ => panic!("Invalid wrapping!"),
    }
}

pub fn wrapper_vti32(input: Input) -> Vec<(i32, i32)> {
    match input {
        Input::VTi32(v) => v,
        _ => panic!("Invalid wrapping!"),
    }
}
