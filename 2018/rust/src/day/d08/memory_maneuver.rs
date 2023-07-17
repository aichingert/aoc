use crate::day::{Input, InputResult, Output, Wrapper};

fn part_one(tree: &Vec<u32>, cur: &mut usize) -> u32 {
    let mut metadata: u32 = 0;
    let amount_of_metadata = tree[*cur + 1];

    for _ in 0..tree[*cur] {
        *cur += 2;
        metadata += part_one(tree, cur);
    }

    for _ in 0..amount_of_metadata {
        metadata += tree[*cur + 2];
        *cur += 1;
    }

    metadata
}

fn part_two(tree: &Vec<u32>, cur: &mut usize) -> u32 {
    let mut metadata = 0u32;
    let mut metadatas = Vec::<u32>::new();
    let amount_of_metadata = tree[*cur + 1] as usize;

    for _ in 0..tree[*cur] {
        *cur += 2;
        metadatas.push(part_two(tree, cur));
    }

    for _ in 0..amount_of_metadata {
        metadata += if metadatas.len() > 0 {
            if tree[*cur + 2] - 1 < metadatas.len() as u32 {
                metadatas[(tree[*cur + 2] - 1) as usize]
            } else {
                0
            }
        } else {
            tree[*cur + 2]
        };
        *cur += 1;
    }

    metadata
}

pub fn run(input: Input) -> (Output, Output) {
    let input: Vec<u32> = input.unwrap();

    (
        Output::Nu32(part_one(&input, &mut 0)),
        Output::Nu32(part_two(&input, &mut 0)),
    )
}

pub fn parse() -> InputResult<Input> {
    Ok(Input::Vu32(
        std::fs::read_to_string("../input/08")?
            .trim()
            .split(' ')
            .map(|s| s.parse())
            .collect::<Result<Vec<u32>, _>>()?,
    ))
}
