use crate::day::{Input, InputResult, Output, Wrapper};

fn solve(n: usize) -> (String, usize) {
    let mut recipes: Vec<u8> = Vec::with_capacity(n + 10);
    recipes.extend([3u8, 7u8].iter());
    let (mut f, mut s) = (0, 1);

    let slice = n.to_string().bytes().map(|b| b - b'0').collect::<Vec<u8>>();
    let mut position: Option<usize> = None;

    while recipes.len() < n + 10 || position.is_none() {
        let next = (recipes[f] + recipes[s])
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        for e in next.iter() {
            recipes.push(*e);
        }

        f = (recipes[f] as usize + 1 + f) % recipes.len();
        s = (recipes[s] as usize + 1 + s) % recipes.len();

        if position.is_none() && recipes.len() > slice.len() {
            for i in 0..next.len() {
                let start = recipes.len() - slice.len() - i;

                if &recipes[start..start + slice.len()] == &slice {
                    position = Some(recipes.len() - slice.len() - i);
                }
            }
        }
    }

    (
        recipes[n..n + 10]
            .iter()
            .map(|b| (b + b'0') as char)
            .collect::<String>(),
        position.expect("while is broken..."),
    )
}

pub fn run(input: Input) -> (Output, Output) {
    let input: usize = input.unwrap();

    let (part_one, part_two) = solve(input);

    (Output::S(part_one), Output::Nusize(part_two))
}

pub fn parse() -> InputResult<Input> {
    Ok(Input::Nusize(
        std::fs::read_to_string("../input/14")?.trim().parse()?,
    ))
}
