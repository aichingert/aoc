pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod wrapper;

pub type Loc = (u32, u32);

pub enum Input {
    Vu32(Vec<u32>),
    Vstr(Vec<String>),
    VTi32(Vec<(i32, i32)>),
    VTLoc(Vec<(Loc, Loc)>),
}

#[derive(Debug)]
pub enum Output {
    Nu32(u32),
    Ni32(i32),
    Nusize(usize),
}
