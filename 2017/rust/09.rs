// Advent of Code 2017, day 9
// (c) aichingert

fn part1(stream: &Vec<char>) -> u32 {
    let mut groups = 0;
    let mut depth = 0;
    let mut i = 0usize;
    let mut within_garbage = false;

    while i < stream.len() {
        match stream[i] {
            '{' => if !within_garbage { depth += 1; },
            '<' => within_garbage = true,
            '!' => if within_garbage { i+= 1; },
            '>' => within_garbage = false,
            '}' => if !within_garbage { groups += depth; depth -= 1; },
            _ => {}
        };

        i += 1;
    }

    groups
}

fn main() {
    let inp = std::fs::read_to_string("../input/09").unwrap().chars().collect::<Vec<char>>();

    println!("Part 1: {}", part1(&inp));
}
