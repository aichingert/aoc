// Advent of Code 2022, day 1
// (c) aichingert

fn solve(cal: &Vec<u32>, to: usize) -> u32 {
    (1..=to).map(|i| cal[cal.len() - i]).sum::<u32>()
}

fn main() {
    let mut inp = std::fs::read_to_string("../input/01").unwrap().trim().split("\n\n").map(|s| s.split('\n').map(|n| n.parse::<u32>().unwrap()).sum::<u32>()).collect::<Vec<u32>>();
    inp.sort();

    println!("Part 1: {}", solve(&inp,1));
    println!("Part 2: {}", solve(&inp,3));
}
