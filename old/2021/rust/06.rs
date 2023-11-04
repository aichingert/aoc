use std::collections::VecDeque;

fn solve(limit: u16, dp: &mut VecDeque<u64>) -> u64 {
    for _ in 0..limit {
        let cur = dp.pop_front().unwrap();
        dp.push_back(cur);
        dp[6] += cur;
    }

    dp.iter().sum()
}

fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap().trim().to_string();
    let inp = inp.split(',').map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut dp = VecDeque::from(vec![0,0,0,0,0,0,0,0,0]);

    for i in 0..inp.len() {
        dp[inp[i]] += 1;
    }

    println!("Part one: {}", solve(80, &mut dp));
    println!("Part two: {}", solve(256 - 80, &mut dp));
}
