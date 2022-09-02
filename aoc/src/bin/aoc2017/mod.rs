use crate::{Solution, Selector};

mod aoc2017_01;
mod aoc2017_02;

use aoc2017_01::*;
use aoc2017_02::*;

pub fn run_2017(which: Selector) {
    let mut day_01 = Aoc2017_01::new();
    let mut day_02 = Aoc2017_02::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02
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