use crate::day::{Claim, Input, InputError, InputResult, Output, Wrapper};

fn part_one(claims: &Vec<Claim>, fabric: &mut [[u16; 1000]; 1000]) -> u32 {
    claims.iter().for_each(|claim| {
        for x in claim.loc.0..claim.loc.0 + claim.width {
            for y in claim.loc.1..claim.loc.1 + claim.height {
                fabric[x][y] += 1;
            }
        }
    });

    fabric
        .iter()
        .map(|line| {
            line.iter()
                .map(|element| if *element > 1 { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn part_two(claims: &Vec<Claim>, fabric: &[[u16; 1000]; 1000]) -> Output {
    'outer: for i in 0..claims.len() {
        for x in claims[i].loc.0..claims[i].loc.0 + claims[i].width {
            for y in claims[i].loc.1..claims[i].loc.1 + claims[i].height {
                if fabric[x][y] != 1 {
                    continue 'outer;
                }
            }
        }

        return Output::Nu32(claims[i].id);
    }
    Output::None
}

pub fn run(input: Input) -> (Output, Output) {
    let input: Vec<Claim> = input.unwrap();
    let mut fabric: [[u16; 1000]; 1000] = [[0; 1000]; 1000];

    (
        Output::Nu32(part_one(&input, &mut fabric)),
        part_two(&input, &fabric),
    )
}

pub fn parse() -> InputResult<Input> {
    let mut claims: Vec<Claim> = Vec::new();

    for line in std::fs::read_to_string("../input/03")?.lines() {
        let elements: Vec<&str> = line.split(' ').collect();

        let id: u32 = elements[0][1..].parse()?;

        let loc: (usize, usize) = if let Some((x, y)) = elements[2].split_once(',') {
            (x.parse()?, y[..y.len() - 1].parse()?)
        } else {
            return Err(InputError::InvalidInput);
        };

        let (width, height): (usize, usize) =
            if let Some((width, height)) = elements[3].split_once('x') {
                (width.parse()?, height.parse()?)
            } else {
                return Err(InputError::InvalidInput);
            };

        claims.push(Claim {
            id,
            loc,
            width,
            height,
        });
    }

    Ok(Input::D03(claims))
}
