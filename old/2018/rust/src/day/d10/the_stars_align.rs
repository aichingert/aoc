use crate::day::d10::Star;
use crate::day::{Input, InputError, InputResult, Output, Wrapper};

// DECREASE THE OFFSET IF YOU DIDN'T GET THE RIGHT ANSWER
const OFFSET: i32 = 65;

fn solve(stars: &mut Vec<Star>) -> i32 {
    let mut ans: i32 = 0;

    loop {
        for i in 0..stars.len() {
            stars[i].update();
        }

        ans += 1;
        let (le_x, ri_x, bo_y, to_y) = Star::furthest_points(stars);

        if (le_x.abs() - ri_x.abs()).abs() < OFFSET && (bo_y.abs() - to_y.abs()).abs() < OFFSET {
            Star::print(stars, le_x, ri_x, bo_y, to_y);
            return ans;
        }
    }
}

pub fn run(input: Input) -> (Output, Output) {
    let mut input: Vec<Star> = input.unwrap();
    let ans: i32 = solve(&mut input);

    (Output::S("Picture".to_string()), Output::Ni32(ans))
}

pub fn parse() -> InputResult<Input> {
    let mut stars: Vec<Star> = Vec::new();
    let extract_position = |s: &str| -> InputResult<(i32, i32)> {
        match s.split_once(", ") {
            Some((x, y)) => Ok((x.trim().parse()?, y.trim().parse()?)),
            None => Err(InputError::InvalidInput),
        }
    };

    for line in std::fs::read_to_string("../input/10")?.lines() {
        if let Some((lhs, rhs)) = line.split_once("> velocity=<") {
            stars.push(Star::new(
                extract_position(&lhs[10..])?,
                extract_position(&rhs[..rhs.len() - 1])?,
            ));
        } else {
            return Err(InputError::InvalidInput);
        }
    }

    Ok(Input::D10(stars))
}
