mod aoc2015_01;
mod aoc2015_02;
mod aoc2015_03;

mod aoc2015_12;

use aoc2015_01::*;
use aoc2015_02::*;
use aoc2015_03::*;

use aoc2015_12::*;

use crate::run_solution;

pub fn run_2015() {
    let mut day_01 = Aoc2015_01::new();
    let mut day_02 = Aoc2015_02::new();
    let mut day_03 = Aoc2015_03::new();
    let mut day_12 = Aoc2015_12::new();

    run_solution(&mut day_01);
    run_solution(&mut day_02);
    run_solution(&mut day_03);
    run_solution(&mut day_12);
}