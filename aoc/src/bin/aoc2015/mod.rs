use crate::{Solution, Selector};

mod aoc2015_01;
mod aoc2015_02;
mod aoc2015_03;
mod aoc2015_04;

mod aoc2015_12;

pub use aoc2015_01::*;
pub use aoc2015_02::*;
pub use aoc2015_03::*;
pub use aoc2015_04::*;

pub use aoc2015_12::*;

pub fn run_2015(which: Selector) {
    let mut day_01 = Aoc2015_01::new();
    let mut day_02 = Aoc2015_02::new();
    let mut day_03 = Aoc2015_03::new();
    let mut day_04 = Aoc2015_04::new();

    let mut day_12 = Aoc2015_12::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04,

        &mut day_12
    ];

    match which {
        Selector::Last => {
            let last = days.len() -1;
            let d = &mut days[last];
            crate::run_solution(*d);
        },
        Selector::All => {
            for d in days {
                crate::run_solution(d);
            }
        },
        Selector::Day(day) => {
            let d = &mut days[day - 1];
            crate::run_solution(*d);
        },
        _ => {}
    }
}