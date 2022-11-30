use crate::{Selector, Solution};

mod aoc2022_01;

use aoc2022_01::*;

pub fn run_2022(which: Selector) {
    let mut day_01 = Aoc2022_01::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01
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
