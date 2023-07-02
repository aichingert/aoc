use crate::day::{wrapper, Input, Output};
use std::collections::HashMap;

#[derive(Eq, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Div,
    Mul,
}

pub enum Calculation {
    Human(i64),
    Value(i64),
    Nested(Box<Calculation>, Operation, Box<Calculation>),
}

impl Operation {
    fn from_str(character: &str) -> Self {
        match character {
            "+" => Operation::Plus,
            "-" => Operation::Minus,
            "/" => Operation::Div,
            "*" => Operation::Mul,
            _ => panic!("invalid input!"),
        }
    }

    fn use_operator(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operation::Plus => lhs + rhs,
            Operation::Minus => lhs - rhs,
            Operation::Div => lhs / rhs,
            Operation::Mul => lhs * rhs,
        }
    }

    fn inverse(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operation::Plus => lhs - rhs,
            Operation::Minus => lhs + rhs,
            Operation::Div => lhs * rhs,
            Operation::Mul => lhs / rhs,
        }
    }
}

impl Calculation {
    fn build_root(all: &HashMap<String, String>, cur: &str) -> Self {
        let values: Vec<&str> = all[cur].split(' ').collect();

        if values.len() == 1 {
            if cur == "humn" {
                Calculation::Human(values[0].parse::<i64>().unwrap())
            } else {
                Calculation::Value(values[0].parse::<i64>().unwrap())
            }
        } else {
            Calculation::Nested(
                Box::new(Self::build_root(all, values[0])), // lhs
                Operation::from_str(values[1]),             // Operator
                Box::new(Self::build_root(all, values[2])), // rhs
            )
        }
    }

    fn execute(&self) -> i64 {
        match self {
            Calculation::Value(x) => *x,
            Calculation::Human(x) => *x,
            Calculation::Nested(lhs, op, rhs) => op.use_operator(lhs.execute(), rhs.execute()),
        }
    }

    fn contains_node_humn(&self) -> bool {
        match self {
            Calculation::Value(_) => false,
            Calculation::Human(_) => true,
            Calculation::Nested(lhs, _, rhs) => {
                lhs.contains_node_humn() || rhs.contains_node_humn()
            }
        }
    }

    fn solve_for_humn(&self, value: &mut i64) {
        match self {
            Calculation::Value(_) => (),
            Calculation::Human(_) => (),
            Calculation::Nested(lhs, op, rhs) => {
                if lhs.contains_node_humn() {
                    *value = op.inverse(*value, rhs.execute());
                    lhs.solve_for_humn(value);
                } else {
                    let r = lhs.execute();

                    *value = match *op {
                        Operation::Minus => (*value - r) * -1,
                        Operation::Div => r / *value,
                        _ => op.inverse(*value, r),
                    };

                    rhs.solve_for_humn(value);
                }
            }
        }
    }
}

fn part_two(root: Calculation) -> Output {
    match root {
        Calculation::Value(_) => (),
        Calculation::Human(_) => (),
        Calculation::Nested(lhs, _, rhs) => {
            let mut value: i64;

            if lhs.contains_node_humn() {
                value = rhs.execute();
                lhs.solve_for_humn(&mut value);
            } else {
                value = lhs.execute();
                rhs.solve_for_humn(&mut value);
            }

            return Output::Ni64(value);
        }
    };

    panic!("no solution found!")
}

pub fn run(input: Input) -> (Output, Output) {
    let root = wrapper::unwrap_d21(input);

    (Output::Ni64(root.execute()), part_two(root))
}

pub fn parse() -> Input {
    let inp = std::fs::read_to_string("../input/21").unwrap();
    let mut calculations: HashMap<String, String> = HashMap::new();

    for line in inp.lines() {
        let (lhs, rhs) = line.split_once(": ").unwrap();
        calculations.insert(lhs.to_string(), rhs.to_string());
    }

    Input::D21(Calculation::build_root(&calculations, "root"))
}
