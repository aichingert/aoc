mod aoc2017_01;
mod aoc2017_02;

use aoc2017_01::*;
use aoc2017_02::*;

use crate::run_solution;

pub fn run_2017() {
    let mut day_01 = Aoc2017_01::new();
    let mut day_02 = Aoc2017_02::new();


    run_solution(&mut day_01);
    run_solution(&mut day_02);
}