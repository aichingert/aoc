// Advent of Code 2022, day 2
// (c) aichingert

fn part1(n: &Vec<(i32,i32)>) -> i32 {
    n.iter().map(|(lhs,rhs)| rhs + (3 - (2 + lhs - rhs) % 3) % 3 * 3).sum::<i32>()
}

fn part2(n: &Vec<(i32,i32)>) -> i32 {
    n.iter().map(|(lhs,rhs)| (lhs+rhs) % 3 + 1 + (rhs - 1) * 3).sum::<i32>()
}

fn parse() -> Vec<(i32,i32)> {
    std::fs::read_to_string("../input/02").unwrap().trim().lines().map(|l| {
        let (lhs,rhs) = l.split_once(' ').unwrap();
        (map_value(lhs),map_value(rhs))
    }).collect::<Vec<(i32,i32)>>()
}

fn map_value(val: &str) -> i32 {
    match val {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => panic!("Invalid input!"),
    }
}

fn main() {
    let n = parse();

    println!("Part 1: {}", part1(&n));
    println!("Part 2: {}", part2(&n));
}
