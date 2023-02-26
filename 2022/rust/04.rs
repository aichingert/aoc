// Advent of Code 2022, day 4
// (c) aichingert

type loc = (u32, u32);

fn part1(inp: &Vec<(loc, loc)>) -> usize {
    inp.iter().filter(|r| r.0.0 <= r.1.0 && r.1.1 <= r.0.1 || r.1.0 <= r.0.0 && r.0.1 <= r.1.1).count()
}

fn part2(inp: &Vec<(loc, loc)>) -> usize {
    inp.iter().filter(|r| !(r.0.1 < r.1.0 || r.0.0 > r.1.1)).count()
}

fn parse() -> Vec<(loc, loc)> {
    let mut vec = Vec::new();

    for l in std::fs::read_to_string("../input/04").unwrap().lines() {
        let (a,b) = l.split_once(',').unwrap();
        let (x,y) = a.split_once('-').unwrap();   
        let (sx,sy) = b.split_once('-').unwrap();
        vec.push(((x.parse().unwrap(),y.parse().unwrap()),(sx.parse().unwrap(),sy.parse().unwrap())));
    }

    vec
}

fn main() {
    let inp = parse();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
