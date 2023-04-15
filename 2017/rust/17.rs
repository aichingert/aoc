// Advent of Code 2017, day 17
// (c) aichingert

fn part1(steps: usize) -> u32 {
    let mut buf = vec![0u32];
    let mut loc = 0usize;

    for _ in 0..2017 {
        loc = (loc + steps) % buf.len() + 1;
        buf.insert(loc, buf.len() as u32);
    }

    buf[loc+1]
}

fn part2(steps: usize) -> usize {
    let mut loc = 0usize;
    let mut ans = 0;

    for i in 1..50000000 {
        loc = (loc + steps) % i + 1;

        if loc == 1 {
            ans = i;
        }
    }

    ans
}

fn main() {
    let steps = std::fs::read_to_string("../input/17").unwrap().trim().parse::<usize>().unwrap();

    println!("Part 1: {}", part1(steps));
    println!("Part 2: {}", part2(steps));
}
