// Advent of Code 2015, day 17
// (c) aichingert

#[path = "../../utils/rust/combinations.rs"]
mod combinations;
use combinations::combinations;

fn solve(cont: &Vec<i32>, part: bool) -> u32 {
    let mut ans = 0;

    for s in 1..cont.len() {
        if part && ans != 0 {
            return ans;
        }

        for comb in combinations(s, cont) {
            if comb.iter().sum::<i32>() == 150 {
                ans += 1;
            }
        }
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/17").unwrap();
    let inp = inp
        .lines()
        .map(|n| n.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("Part 1: {}", solve(&inp, false));
    println!("Part 2: {}", solve(&inp, true));
}
