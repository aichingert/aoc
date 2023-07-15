use crate::day::{d03::no_matter::Claim, Input};
use std::collections::{HashMap, VecDeque};

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

wrapping!(VDu8, VecDeque<u8>);
wrapping!(Vi32, Vec<i32>);
wrapping!(VVch, Vec<Vec<char>>);
wrapping!(D03, Vec<Claim>);
wrapping!(D04, HashMap<u16, Vec<[u16; 60]>>);
