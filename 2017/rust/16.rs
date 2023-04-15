// Advent of Code 2017, day 16
// (c) aichingert

use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Debug)]
enum Step {
    Spin(usize),
    Exchange(usize,usize),
    Partner(char,char),
}

#[derive(Debug)]
struct StepError;

impl FromStr for Step {
    type Err = StepError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..1] {
            "s" => Ok(Step::Spin(s[1..].parse().unwrap())),
            "x" => {
                let (x,y) = s[1..].split_once('/').unwrap();
                Ok(Step::Exchange(x.parse().unwrap(), y.parse().unwrap()))
            },
            "p" => {
                let chrs = s[1..].chars().collect::<Vec<char>>();
                Ok(Step::Partner(chrs[0],chrs[2]))
            }
            _ => Err(StepError {})
        }
    }
}

impl Step {
    fn execute(&self, program: &mut VecDeque<char>) {
        match self {
            Step::Spin(x) => {
                for _ in 0..*x {
                    let end = program.pop_back().unwrap();
                    program.push_front(end);
                }
            },
            Step::Exchange(x,y) => {
                let item = program[*x];
                program[*x] = program[*y];
                program[*y] = item;
            },
            Step::Partner(a,b) => {
                let x = (0..program.len()).find(|&i| program[i] == *a).unwrap();
                let y = (0..program.len()).find(|&i| program[i] == *b).unwrap();
                Step::Exchange(x,y).execute(program);
            }
        }
    }
}

fn part2(inp: &Vec<Step>, program: &mut VecDeque<char>) -> String {
    let mut seen: Vec<VecDeque<char>> = vec![('a'..='p').collect::<VecDeque<char>>()];

    for _ in 0..999999999 {
        if seen.contains(&program) {
            break;
        }

        seen.push(program.clone());
        inp.iter().for_each(|step| step.execute(program));
    }

    let len = seen.len();
    seen[1000000000 % len].iter().collect::<String>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap().trim()
        .split(',')
        .map(|l| {
            Step::from_str(l).unwrap()
        }).collect::<Vec<Step>>();
    let mut program: VecDeque<char> = ('a'..='p').collect();
    inp.iter().for_each(|step| step.execute(&mut program));

    println!("Part 1: {}", program.iter().collect::<String>());
    println!("Part 2: {}", part2(&inp, &mut program));
}
