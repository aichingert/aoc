// Advent of Code 2015, day 25
// (c) aichingert

fn part1(row: u64, col: u64) -> u64 {
    let mut ry = 1u64;
    let mut cx = 1u64;
    let mut cur = 20151125u64;

    while row != ry || col != cx {
        if ry == 1 {
            ry = cx + 1;
            cx = 1;
        } else {
            ry -= 1;
            cx += 1;
        }

        cur = (cur * 252533) % 33554393;
    }

    cur
}

fn part2<'a>() -> &'a str {
    "Merry chirstmas"
}

fn parse() -> (u64, u64) {
    let inp = std::fs::read_to_string("../input/25").unwrap();
    let inp = inp.trim().split(' ').collect::<Vec<&str>>();

    (
        inp[16][..inp[16].len() - 1].parse::<u64>().unwrap(),
        inp[18][..inp[18].len() - 1].parse::<u64>().unwrap(),
    )
}

fn main() {
    let (row, col) = parse();

    println!("Part 1: {}", part1(row, col));
    println!("Part 2: {}", part2());
}
