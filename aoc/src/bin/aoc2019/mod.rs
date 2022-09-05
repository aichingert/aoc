use crate::{Selector, Solution};

mod aoc2019_01;
mod aoc2019_02;
mod aoc2019_03;
mod aoc2019_04;

use aoc2019_01::*;
use aoc2019_02::*;
use aoc2019_03::*;
use aoc2019_04::*;

pub fn run_2019(which: Selector) {
    let mut day_01 = Aoc2019_01::new();
    let mut day_02 = Aoc2019_02::new();
    let mut day_03 = Aoc2019_03::new();
    let mut day_04 = Aoc2019_04::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04
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