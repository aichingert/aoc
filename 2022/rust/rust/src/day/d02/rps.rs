use crate::day::{wrapper::unwrap_vti32, Input, Output};

fn part_one(n: &Vec<(i32, i32)>) -> Output {
    Output::Ni32(
        n.iter()
            .map(|(lhs, rhs)| rhs + (3 - (2 + lhs - rhs) % 3) % 3 * 3)
            .sum::<i32>(),
    )
}

fn part_two(n: &Vec<(i32, i32)>) -> Output {
    Output::Ni32(
        n.iter()
            .map(|(lhs, rhs)| (lhs + rhs) % 3 + 1 + (rhs - 1) * 3)
            .sum::<i32>(),
    )
}

pub fn run(input: Input) -> (Output, Output) {
    let input: Vec<(i32, i32)> = unwrap_vti32(input);

    (part_one(&input), part_two(&input))
}

pub fn parse() -> Input {
    Input::VTi32(
        std::fs::read_to_string("../../input/02")
            .unwrap()
            .trim()
            .lines()
            .map(|l| {
                let (lhs, rhs) = l.split_once(' ').unwrap();
                (map_value(lhs), map_value(rhs))
            })
            .collect::<Vec<(i32, i32)>>(),
    )
}

fn map_value(val: &str) -> i32 {
    match val {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Invalid input!"),
    }
}
