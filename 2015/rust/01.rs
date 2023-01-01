fn solve(lines: &[char], part: bool) -> i32 {
    let mut loc: i32 = 0;

    for (i,c) in lines.iter().enumerate() {
        loc += match c {
            '(' => 1,
            _ => -1
        };

        if part && loc < 0 {
            return i as i32 + 1;
        }
    }

    loc
}

fn main() {
    let input: Vec<char> = std::fs::read_to_string("../input/01").unwrap().chars().collect();

    println!("Part 1: {}", solve(&input, false));
    println!("Part 2: {}", solve(&input, true));
}
