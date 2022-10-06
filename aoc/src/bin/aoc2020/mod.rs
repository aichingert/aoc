use crate::{Solution, Selector};

mod aoc2020_01;
mod aoc2020_02;
mod aoc2020_03;
mod aoc2020_04;
mod aoc2020_05;
mod aoc2020_15;
mod aoc2020_18;

use aoc2020_01::*;
use aoc2020_02::*;
use aoc2020_03::*;
use aoc2020_04::*;
use aoc2020_05::*;
use aoc2020_15::*;
use aoc2020_18::*;

pub fn run_2020(which: Selector) {
    let mut day_01 = Aoc2020_01::new();
    let mut day_02 = Aoc2020_02::new();
    let mut day_03 = Aoc2020_03::new();
    let mut day_04 = Aoc2020_04::new();
    let mut day_05 = Aoc2020_05::new();
    let mut day_15 = Aoc2020_15::new();
	let mut day_18 = Aoc2020_18::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04, &mut day_05, &mut day_15, &mut day_18
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
