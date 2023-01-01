fn main() {
    let input: Vec<char> = std::fs::read_to_string("../input/01").unwrap().chars().collect();

    println!("Part 1: {}", solve(&input, false));
    println!("Part 2: {}", solve(&input, true));
}

fn solve(lines: &[char], part: bool) -> i32 {
    let mut d: i32 = 0;

    for (i,c) in lines.iter().enumerate() {
        d += match c {
            '(' => 1,
            _ => -1
        };

        if part && d < 0 {
            return i as i32 + 1;
        }
    }

    d
}
