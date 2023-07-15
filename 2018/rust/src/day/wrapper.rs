use crate::day::{d03::no_matter::Claim, Input};

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

wrapping!(Vi32, Vec<i32>);
wrapping!(VVch, Vec<Vec<char>>);
wrapping!(D03, Vec<Claim>);
