use crate::day::{Cart, Cell, Claim, Coordinate, Input, Point, Star, Unit};
use std::collections::{HashMap, HashSet, VecDeque};

pub trait Wrapper<T> {
    fn unwrap(self) -> T;
}

macro_rules! wrapping {
    ($element:ident, $type:ty) => {
        impl Wrapper<$type> for Input {
            fn unwrap(self) -> $type {
                match self {
                    Input::$element(x) => x,
                    _ => panic!("invalid wrapping!"),
                }
            }
        }
    };
}

wrapping!(Ni32, i32);
wrapping!(Nusize, usize);
wrapping!(Vu32, Vec<u32>);
wrapping!(Vi32, Vec<i32>);
wrapping!(Vusize, Vec<usize>);
wrapping!(VDu8, VecDeque<u8>);
wrapping!(VVch, Vec<Vec<char>>);
wrapping!(HchVch, HashMap<char, Vec<char>>);
wrapping!(D03, Vec<Claim>);
wrapping!(D04, HashMap<u16, Vec<[u16; 60]>>);
wrapping!(D06, (Vec<Point<i32>>, [i32; 4]));
wrapping!(D10, Vec<Star>);
wrapping!(D12, (HashSet<i64>, HashMap<String, String>));
wrapping!(D13, (Vec<Cart>, Vec<Vec<Cell>>));
wrapping!(D15, (Vec<Unit>, HashMap<Coordinate, char>));
