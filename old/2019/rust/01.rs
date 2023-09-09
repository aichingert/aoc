// Advent of Code 2019, day 1
// (c) aichingert

fn part1(inp: &[u32]) -> u32 {
    inp.iter().map(|n| n / 3 - 2).sum::<u32>()
}

fn part2(inp: &mut [u32]) -> u32 {
    let mut ans = 0;

    inp.iter_mut().for_each(|n| 
        while *n / 3 > 1 {
            *n = *n / 3 - 2;
            ans += *n;
        }
    );

    ans
}

fn main() {
    let mut inp = std::fs::read_to_string("../input/01").unwrap().lines().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&mut inp));
}
