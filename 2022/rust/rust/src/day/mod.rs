pub mod d01;
pub mod d02;
pub mod wrapper;

pub enum Input {
    Vu32(Vec<u32>),
    VTi32(Vec<(i32, i32)>),
}

#[derive(Debug)]
pub enum Output {
    Nu32(u32),
    Ni32(i32),
}
