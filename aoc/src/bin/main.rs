use std::{fmt::Display, time};

mod aoc2015;
mod aoc2016;
mod aoc2017;
mod aoc2018;
mod aoc2019;
mod aoc2020;
mod aoc2021;
mod aoc2022;

use aoc2015::*;
use aoc2016::*;
use aoc2017::*;
use aoc2018::*;
use aoc2019::*;
use aoc2020::*;
use aoc2021::*;
use aoc2022::*;

pub enum Selector {
    All,
    Year,
    Day(usize),
    Last,
}

pub trait Solution {
    fn name(&self) -> (usize, usize);
    fn parse(&mut self);
    fn part1(&mut self) -> Vec<String>;
    fn part2(&mut self) -> Vec<String>;
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 {
        if let Ok(year) = args[1].parse::<usize>() {
            if args.len() > 2 {
                if let Ok(day) = args[2].parse::<usize>() {
                    match year {
                        2015 => run_2015(Selector::Day(day)),
                        2016 => run_2016(Selector::Day(day)),
                        2017 => run_2017(Selector::Day(day)),
                        2018 => run_2018(Selector::Day(day)),
                        2019 => run_2019(Selector::Day(day)),
                        2020 => run_2020(Selector::Day(day)),
                        2021 => run_2021(Selector::Day(day)),
                        2022 => run_2022(Selector::Day(day)),
                        _ => panic!("invalid year {year}")
                    }
                }
            } else {
                match year {
                    2015 => run_2015(Selector::All),
                    2016 => run_2016(Selector::All),
                    2017 => run_2017(Selector::All),
                    2018 => run_2018(Selector::All),
                    2019 => run_2019(Selector::All),
                    2020 => run_2020(Selector::All),
                    2021 => run_2021(Selector::All),
                    2022 => run_2022(Selector::All),
                    _ => panic!("invalid year {year}")
                }
            }
        } else {
            run_2015(Selector::All);
            run_2016(Selector::All);
            run_2017(Selector::All);
            run_2018(Selector::All);
            run_2019(Selector::All);
            run_2020(Selector::All);
            run_2021(Selector::All);
            run_2022(Selector::All);
        }
    } else {
        run_2015(Selector::Last);
        run_2016(Selector::Last);
        run_2017(Selector::Last);
        run_2018(Selector::Last);
        run_2019(Selector::Last);
        run_2020(Selector::Last);
        run_2021(Selector::Last);
        run_2022(Selector::Last);
    }
}

pub fn run_solution<T: Solution +?Sized>(solution: &mut T) {
    let name = solution.name();
    println!("------ {}, Day {} ------", name.0, name.1);

    solution.parse();

    let time = time::Instant::now();
    let p1 = solution.part1();
    let duration = time.elapsed();
    print_solution(1, &p1, duration);

    let time = time::Instant::now();
    let p2 = solution.part2();
    let duration = time.elapsed();
    print_solution(2, &p2, duration);
}

pub fn output<T: Display>(output: T) -> Vec<String> {
    vec![format!("{}", output)]
}

fn print_solution(which: usize, output: &[String], duration: time::Duration) {
    let mut iter = output.iter();

    println!("{:.6} | Part {which}: {}",duration.as_secs_f64() , iter.next().unwrap());
    
    for line in iter {
        println!("       {}", line)
    }
}
