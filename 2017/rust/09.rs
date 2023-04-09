// Advent of Code 2017, day 9
// (c) aichingert

fn solve(stream: &Vec<char>) -> (u32,u32) {
    let (mut groups, mut garbage) = (0, 0);
    let mut depth = 0;
    let mut i = 0usize;
    let mut within_garbage = false;

    while i < stream.len() {
        match stream[i] {
            '{' => if !within_garbage { depth += 1; } else { garbage += 1; },
            '<' => if within_garbage { garbage += 1; } else { within_garbage = true; },
            '!' => if within_garbage { i+= 1; },
            '>' => within_garbage = false,
            '}' => if !within_garbage { groups += depth; depth -= 1; } else { garbage += 1; },
            _ => if within_garbage { garbage += 1; }
        };

        i += 1;
    }

    (groups, garbage)
}

fn main() {
    let inp = std::fs::read_to_string("../input/09").unwrap().chars().collect::<Vec<char>>();
    let (part1,part2) = solve(&inp);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
