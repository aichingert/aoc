use crate::day::{Input, InputResult, Output, Pos, Wrapper};
use std::collections::HashSet;

fn solve(bursts: &Vec<char>, patterns: &Vec<(char, Vec<(i32, i32)>)>, stones: u64) -> Output {
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut burst: usize = 0;
    let mut h_y: i32 = -1;
    let mut pattern = 0usize;

    for _ in 0..stones {
        h_y = match patterns[pattern].0 {
            '-' | 'L' | 'I' | 'o' => {
                drop(&patterns[pattern].1, 2, h_y, &mut map, &mut burst, bursts)
            }
            '+' => drop(
                &patterns[pattern].1,
                2,
                h_y + 1,
                &mut map,
                &mut burst,
                bursts,
            ),
            _ => panic!("invalid shape"),
        };

        pattern = (pattern + 1) % patterns.len();
    }

    Output::Ni32(h_y + 1)
}

fn drop(
    cords: &Vec<(i32, i32)>,
    x: i32,
    h_y: i32,
    map: &mut HashSet<(i32, i32)>,
    burst: &mut usize,
    bursts: &Vec<char>,
) -> i32 {
    let (mut x, mut y) = (x, h_y + 4);
    let mut biggest = -i32::MAX;

    for i in 0..cords.len() {
        if cords[i].0 > biggest {
            biggest = cords[i].0;
        }
    }

    loop {
        gas_burst(map, cords, bursts, burst, &mut x, y, biggest);

        if y > 0 && !check_for_collision(map, cords, x, y, (0, -1)) {
            y -= 1;
        } else {
            let mut h_y = h_y;

            for cord in cords.iter() {
                let c_y = y + cord.1;
                if h_y < c_y {
                    h_y = c_y;
                }
                map.insert((x + cord.0, c_y));
            }

            return h_y;
        }
    }
}

fn gas_burst(
    map: &HashSet<(i32, i32)>,
    cords: &Vec<(i32, i32)>,
    bursts: &Vec<char>,
    burst: &mut usize,
    x: &mut i32,
    y: i32,
    biggest: i32,
) {
    match bursts[*burst] {
        '>' => {
            if biggest + *x + 1 < 7 && !check_for_collision(map, cords, *x, y, (1, 0)) {
                *x += 1;
            }
        }
        '<' => {
            if *x - 1 > -1 && !check_for_collision(map, cords, *x, y, (-1, 0)) {
                *x -= 1;
            }
        }
        _ => panic!("invalid burst direction"),
    };

    *burst = (*burst + 1) % bursts.len();
}

fn check_for_collision(
    map: &HashSet<(i32, i32)>,
    cords: &Vec<(i32, i32)>,
    x: i32,
    y: i32,
    dir: (i32, i32),
) -> bool {
    for cord in cords {
        if map.contains(&(x + cord.0 + dir.0, y + cord.1 + dir.1)) {
            return true;
        }
    }

    false
}

pub fn run(input: Input) -> (Output, Output) {
    let (bursts, patterns): (Vec<char>, Vec<(char, Vec<Pos>)>) = input.unwrap();

    (solve(&bursts, &patterns, 2022), Output::Ni32(-1))
}

pub fn parse() -> InputResult<Input> {
    Ok(Input::D17((
        std::fs::read_to_string("../input/17")?
            .chars()
            .collect::<Vec<char>>(),
        vec![
            ('-', vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            ('+', vec![(0, 0), (1, 0), (2, 0), (1, -1), (1, 1)]),
            ('L', vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
            ('I', vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            ('o', vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        ],
    )))
}
