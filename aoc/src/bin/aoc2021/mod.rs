use crate::{Selector, Solution};

mod aoc2021_01;
mod aoc2021_02;
mod aoc2021_03;
mod aoc2021_04;
mod aoc2021_05;
mod aoc2021_06;
mod aoc2021_07;
mod aoc2021_08;
mod aoc2021_09;
//mod aoc2021_16;
mod aoc2021_17;
mod aoc2021_22;

use aoc2021_01::*;
use aoc2021_02::*;
use aoc2021_03::*;
use aoc2021_04::*;
use aoc2021_05::*;
use aoc2021_06::*;
use aoc2021_07::*;
use aoc2021_08::*;
use aoc2021_09::*;
//use aoc2021_16::*;
use aoc2021_17::*;
use aoc2021_22::*;

pub fn run_2021(which: Selector) {
    let mut day_01 = Aoc2021_01::new();
    let mut day_02 = Aoc2021_02::new();
    let mut day_03 = Aoc2021_03::new();
    let mut day_04 = Aoc2021_04::new();
    let mut day_05 = Aoc2021_05::new();
    let mut day_06 = Aoc2021_06::new();
    let mut day_07 = Aoc2021_07::new();
    let mut day_08 = Aoc2021_08::new();
    let mut day_09 = Aoc2021_09::new();
    //let mut day_16 = Aoc2021_16::new();
    let mut day_17 = Aoc2021_17::new();
    let mut day_22 = Aoc2021_22::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04, &mut day_05, 
        &mut day_06, &mut day_07, &mut day_08, &mut day_09,
        /*&mut day_16, */&mut day_17,
        &mut day_22
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
