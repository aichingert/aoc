mod aoc2018_01;
mod aoc2018_02;

use aoc2018_01::*;
use aoc2018_02::*;

use crate::run_solution;

pub fn run_2018() {
    let mut day_01 = Aoc2018_01::new();
    let mut day_02 = Aoc2018_02::new();

    run_solution(&mut day_01);
    run_solution(&mut day_02);
}