mod aoc2019_01;
mod aoc2019_02;

use aoc2019_01::*;
use aoc2019_02::*;

use crate::run_solution;

pub fn run_2019() {
    let mut day_01 = Aoc2019_01::new();
    let mut day_02 = Aoc2019_02::new();

    run_solution(&mut day_01);
    run_solution(&mut day_02);
}