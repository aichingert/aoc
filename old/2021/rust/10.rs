use std::collections::VecDeque;

const COST: [u32; 4] = [3, 57, 1197, 25137];

fn reverse(ch: char) -> char {
    match ch {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        '>' => '<',
        _ => panic!("reverse: invalid brace {}", ch),
    }
}

fn index(ch: char) -> usize {
    match ch {
        ')' => 0,
        ']' => 1,
        '}' => 2,
        '>' => 3,
        _ => panic!("index: invalid brace {}", ch),
    }
}

fn part_one(lines: &[Vec<char>]) -> u32 {
    let mut illegal = [0; 4];

    for i in 0..lines.len() {
        let mut opening_braces = VecDeque::new();

        for j in 0..lines[i].len() {
            match lines[i][j] {
                ')' | '}' | ']' | '>' => {
                    let next = opening_braces.pop_front();
                    let c = lines[i][j];

                    if next.and_then(|b| if b == reverse(c) { Some(b) } else { None }).is_none() {
                        illegal[index(c)] += 1;
                        break;
                    }
                }
                _ => opening_braces.push_front(lines[i][j]),
            }
        }
    }

    (0..illegal.len()).map(|i| COST[i] * illegal[i]).sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
}
