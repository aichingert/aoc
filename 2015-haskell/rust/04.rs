// Advent of Code 2015, day 4
// (c) aichingert

#[path = "../../utils/rust/md5.rs"]
mod md5;

fn solve(key: &str, zeros: usize) -> u32 {
    let mut n: u32 = 0;

    'md5: loop {
        n += 1;
        let s: Vec<char> = md5::md5_utf8((key.to_owned() + &n.to_string()).as_str())
            .chars()
            .collect();

        for ch in s[..zeros].iter() {
            if ch != &'0' {
                continue 'md5;
            }
        }

        return n;
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/04").unwrap();

    println!("Part 1: {:?}", solve(input.trim(), 5));
    println!("Part 2: {:?}", solve(input.trim(), 6));
}
