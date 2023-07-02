use crate::day::{wrapper, Input, Output};

fn solve(cal: &Vec<u32>, to: usize) -> Output {
    Output::Nu32(cal[cal.len() - to..].iter().sum::<u32>())
}

pub fn run(cal: Input) -> (Output, Output) {
    let cal: Vec<u32> = wrapper::wrapper_vu32(cal);

    (solve(&cal, 1), solve(&cal, 3))
}

pub fn parse() -> Input {
    let mut inp = std::fs::read_to_string("../../input/01")
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|n| n.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    inp.sort();

    Input::Vu32(inp)
}
