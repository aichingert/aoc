use crate::{Selector, Solution};

mod aoc2019_01;
mod aoc2019_02;
mod aoc2019_03;
mod aoc2019_04;
mod aoc2019_05;
mod aoc2019_06;
mod aoc2019_12;

use aoc2019_01::*;
use aoc2019_02::*;
use aoc2019_03::*;
use aoc2019_04::*;
use aoc2019_05::*;
use aoc2019_06::*;
use aoc2019_12::*;

pub fn run_2019(which: Selector) {
    let mut day_01 = Aoc2019_01::new();
    let mut day_02 = Aoc2019_02::new();
    let mut day_03 = Aoc2019_03::new();
    let mut day_04 = Aoc2019_04::new();
	let mut day_05 = Aoc2019_05::new();
    let mut day_06 = Aoc2019_06::new();
    let mut day_12 = Aoc2019_12::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04,  &mut day_05,
        &mut day_06,
        &mut day_12,
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
