// Advent of Code 2015, day 2
// (c) aichingert

fn part1(p: &[(u32, u32, u32)]) -> u32 {
    p.iter()
        .map(|(l, w, h)| 2 * l * w + 2 * w * h + 2 * h * l + l * w)
        .sum::<u32>()
}

fn part2(p: &[(u32, u32, u32)]) -> u32 {
    p.iter()
        .map(|(l, w, h)| 2 * l + 2 * w + l * w * h)
        .sum::<u32>()
}

fn main() {
    let mut p: Vec<(u32, u32, u32)> = Vec::new();

    for line in std::fs::read_to_string("../input/02").unwrap().lines() {
        let mut vls = line
            .split('x')
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        vls.sort();
        p.push((vls[0], vls[1], vls[2]));
    }

    println!("Part 1: {}", part1(&p));
    println!("Part 2: {}", part2(&p));
}
