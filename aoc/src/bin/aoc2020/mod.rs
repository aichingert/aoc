mod aoc2020_01;
mod aoc2020_02;

use aoc2020_01::*;
use aoc2020_02::*;

use crate::run_solution;

pub fn run_2020() {
    let mut day_01 = Aoc2020_01::new();
    let mut day_02 = Aoc2020_02::new();

    run_solution(&mut day_01);
    run_solution(&mut day_02);
}