use std::fmt::Display;

mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2018;
mod aoc2019;
mod aoc2020;
mod aoc2021;

use aoc2015::*;
use aoc2016::*;
use aoc2017::*;
use aoc2018::*;
use aoc2019::*;
use aoc2020::*;
use aoc2021::*;

pub trait Solution {
    fn name(&self) -> (usize, usize);
    fn parse(&mut self);
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
}

fn main() {
    let _args = std::env::args().collect::<Vec<String>>();

    run_2015();
    run_2016();
    run_2017();
    run_2018();
    run_2019();
    run_2020();
    run_2021();
}

pub fn run_solution<T: Solution>(solution: &mut T) {
    let name = solution.name();
    println!("---- {}, Day {} ----", name.0, name.1);

    solution.parse();

    let p1 = solution.part1();
    print_solution(1, &p1);

    let p2 = solution.part2();
    print_solution(2, &p2);
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
}

fn print_solution(which: usize, output: &[String]) {
    let mut iter = output.iter();

    println!("Part {which}: {}", iter.next().unwrap());
    
    for line in iter {
        println!("       {}", line)
    }
}