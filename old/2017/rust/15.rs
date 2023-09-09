// Advent of Code 2017, day 15
// (c) aichingert

fn solve(gens: &Vec<u64>, part_one: bool) -> u32 {
    let (mut a, mut b) = (gens[0], gens[1]);
    let mut ans = 0;
    let times = match part_one {
        true => 40_000_000,
        false => 5_000_000,
    };

    for _ in 0..times {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;

        while !part_one && a % 4 != 0 { a = (a * 16807) % 2147483647; }
        while !part_one && b % 8 != 0 { b = (b * 48271) % 2147483647; }

        if a as u16 == b as u16 {
            ans += 1;
        }
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/15").unwrap()
        .lines()
        .map(|l| { let v = l.split(' ').collect::<Vec<&str>>(); v[4].parse().unwrap()})
        .collect::<Vec<u64>>();

    println!("Part 1: {}", solve(&inp, true));
    println!("Part 2: {}", solve(&inp, false));
}
