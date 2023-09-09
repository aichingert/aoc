// Advent of Code 2015, day 24
// (c) aichingert

#[path = "../../utils/rust/combinations.rs"]
mod combinations;
use combinations::combinations;

fn solve(n: &Vec<u64>, target: u64) -> u64 {
    let mut quant: Vec<u64> = Vec::new();

    for s in 1..n.len() {
        for comb in combinations(s, n) {
            if comb.iter().sum::<u64>() != target {
                continue;
            }

            if quant.len() == 0
                || comb.len() < quant.len()
                || comb.len() == quant.len()
                    && comb.iter().fold(1, |p, c| p * c) < quant.iter().fold(1, |p, c| p * c)
            {
                quant = comb;
            }
        }
    }

    quant.iter().fold(1, |p, c| p * c)
}

fn main() {
    let inp = std::fs::read_to_string("../input/24")
        .unwrap()
        .lines()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("Part 1: {}", solve(&inp, inp.iter().sum::<u64>() / 3));
    println!("Part 2: {}", solve(&inp, inp.iter().sum::<u64>() / 4));
}
