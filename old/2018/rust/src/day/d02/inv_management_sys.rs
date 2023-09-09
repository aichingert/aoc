use crate::day::{Input, InputResult, Output, Wrapper};

fn part_one(lines: &Vec<Vec<char>>) -> i32 {
    let (mut twos, mut threes) = (0, 0);

    for i in 0..lines.len() {
        let mut buf: [u16; 26] = [0; 26];

        for j in 0..lines[i].len() {
            buf[(lines[i][j] as u8 - b'a') as usize] += 1;
        }

        for j in 0..buf.len() {
            match buf[j] {
                2 => twos += 1,
                3 => threes += 1,
                _ => (),
            }
        }
    }

    twos * threes
}

fn part_two(lines: &Vec<Vec<char>>) -> Output {
    for i in 0..lines.len().saturating_sub(1) {
        for j in i + 1..lines[i].len() {
            if lines[i].len() != lines[j].len() {
                continue;
            }

            if (0..lines[i].len())
                .map(|k| if lines[i][k] != lines[j][k] { 1 } else { 0 })
                .sum::<u8>()
                != 1
            {
                continue;
            }

            let mut output = lines[i].clone();

            for i in 0..output.len() {
                if output[i] != lines[j][i] {
                    output.remove(i);
                    return Output::S(output.iter().collect::<String>());
                }
            }
        }
    }
    Output::None
}

pub fn run(input: Input) -> (Output, Output) {
    let input: Vec<Vec<char>> = input.unwrap();

    (Output::Ni32(part_one(&input)), part_two(&input))
}

pub fn parse() -> InputResult<Input> {
    Ok(Input::VVch(
        std::fs::read_to_string("../input/02")?
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    ))
}
