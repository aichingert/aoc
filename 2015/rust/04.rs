// Advent of Code 2015, day 4
// (c) aichingert

mod md5;

fn solve(key: &str, zeros: usize) -> u32 {
    let mut n: u32 = 0;
    
    loop {
        n+=1;
        let s: Vec<char> = md5::md5_utf8((key.to_owned() + &n.to_string()).as_str()).chars().collect();
        if s[..zeros].iter().map(|ch| match ch {
            '0' => 0,
            _ => 1,
        }).sum::<i32>() == 0 {
            return n;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/04").unwrap();

    println!("Part 1: {:?}", solve(input.trim(), 5));
    println!("Part 2: {:?}", solve(input.trim(), 6));
}
