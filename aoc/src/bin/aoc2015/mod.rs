use crate::{Solution, Selector};

mod aoc2015_01;
mod aoc2015_02;
mod aoc2015_03;
mod aoc2015_04;
mod aoc2015_05;
mod aoc2015_09;
mod aoc2015_10;
mod aoc2015_11;
mod aoc2015_12;
mod aoc2015_13;
mod aoc2015_15;
mod aoc2015_16;
mod aoc2015_17;

use aoc2015_01::*;
use aoc2015_02::*;
use aoc2015_03::*;
use aoc2015_04::*;
use aoc2015_05::*;
use aoc2015_09::*;
use aoc2015_10::*;
use aoc2015_11::*;
use aoc2015_12::*;
use aoc2015_13::*;
use aoc2015_15::*;
use aoc2015_16::*;
use aoc2015_17::*;

pub fn run_2015(which: Selector) {
    let mut day_01 = Aoc2015_01::new();
    let mut day_02 = Aoc2015_02::new();
    let mut day_03 = Aoc2015_03::new();
    let mut day_04 = Aoc2015_04::new();
    let mut day_05 = Aoc2015_05::new();
    let mut day_09 = Aoc2015_09::new();
    let mut day_10 = Aoc2015_10::new();
    let mut day_11 = Aoc2015_11::new();
    let mut day_12 = Aoc2015_12::new();
    let mut day_13 = Aoc2015_13::new();
    let mut day_15 = Aoc2015_15::new();
    let mut day_16 = Aoc2015_16::new();
    let mut day_17 = Aoc2015_17::new();

    let mut days: Vec<&mut dyn Solution> = vec![
        &mut day_01, &mut day_02, &mut day_03, &mut day_04 , &mut day_05, 
	&mut day_09, &mut day_10, 
	&mut day_11, &mut day_12, &mut day_13, &mut day_15, 
	&mut day_16, &mut day_17
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
