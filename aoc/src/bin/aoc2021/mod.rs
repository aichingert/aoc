mod aoc2021_01;
mod aoc2021_02;

use aoc2021_01::*;
use aoc2021_02::*;

use crate::run_solution;

pub fn run_2021() {
    let mut day_01 = Aoc2021_01::new();
    let mut day_02 = Aoc2021_02::new();

    run_solution(&mut day_01);
    run_solution(&mut day_02);
}