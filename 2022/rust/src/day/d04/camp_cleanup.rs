use crate::day::{Input, Loc, Output, Wrapper};

fn part_one(inp: &Vec<(Loc, Loc)>) -> Output {
    Output::Nusize(
        inp.iter()
            .filter(|r| {
                r.0 .0 <= r.1 .0 && r.1 .1 <= r.0 .1 || r.1 .0 <= r.0 .0 && r.0 .1 <= r.1 .1
            })
            .count(),
    )
}

fn part_two(inp: &Vec<(Loc, Loc)>) -> Output {
    Output::Nusize(
        inp.iter()
            .filter(|r| !(r.0 .1 < r.1 .0 || r.0 .0 > r.1 .1))
            .count(),
    )
}

pub fn run(input: Input) -> (Output, Output) {
    let input = input.unwrap();

    (part_one(&input), part_two(&input))
}

pub fn parse() -> Input {
    Input::VTLoc(
        std::fs::read_to_string("../input/04")
            .unwrap()
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(',').unwrap();
                let (x, y) = a.split_once('-').unwrap();
                let (sx, sy) = b.split_once('-').unwrap();

                (
                    (x.parse().unwrap(), y.parse().unwrap()),
                    (sx.parse().unwrap(), sy.parse().unwrap()),
                )
            })
            .collect::<Vec<(Loc, Loc)>>(),
    )
}
