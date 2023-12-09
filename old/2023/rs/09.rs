use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let inp = std::fs::read_to_string("../input/09").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{:?}", inp);
    let mut ans = 0;

    for i in 0..inp.len() {
        let mut diff = vec![inp[i].clone()];

        let mut d = 0;
        let mut z = false;

        while !z {
            z = true;
            let mut next = Vec::new();

            for j in 0..diff[d].len() - 1{
                let df = diff[d][j + 1] - diff[d][j];

                if df != 0 { z = false; }

                next.push(df);
            }
            diff.push(next);

            d += 1;
        }

        let mut sum = 0;

        for i in 0..diff.len() {
            sum = *diff[diff.len() - i - 1].first().unwrap() - sum;
        }

        ans += sum;
    }

    println!("ans: {ans}");
}
