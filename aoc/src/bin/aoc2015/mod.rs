mod aoc2015_01;
mod aoc2015_02;

use aoc2015_01::*;
use aoc2015_02::*;

use crate::run_solution;

pub fn run_2015() {
    let mut day_01 = Aoc2015_01::new();
    let mut day_02 = Aoc2015_02::new();

    run_solution(&mut day_01);
    run_solution(&mut day_02);
}