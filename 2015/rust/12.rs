// Advent of Code 2015, day 12
// (c) aichingert

const V: [char;11] = ['-','0','1','2','3','4','5','6','7','8','9'];

fn part1(ch: &[char]) -> i32 {
    let mut ans = 0i32;
    let mut cur = String::new();
    
    for i in 0..ch.len() {
        if V.contains(&ch[i]) {
            cur.push(ch[i]);
        } else {
            cur.push(' ');
        }
    }

    cur.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).sum::<i32>()
}

fn part2() {}

fn main() {
    let inp = std::fs::read_to_string("../input/12").unwrap();
    let inp = inp.trim().chars().collect::<Vec<char>>();

    println!("Part 1: {}", part1(&inp));
}
