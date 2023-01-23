// Advent of Code 2015, day 10
// (c) aichingert

fn solve(ans: &mut Vec<char>, steps: u32) -> usize {
    for _ in 0..steps {
        let mut i = 0;
        let mut n = Vec::new();

        while i < ans.len() {
            let mut amt: u8 = 1;
            for j in i+1..ans.len() {
                if ans[i] != ans[j] { break; }
                amt += 1;
                i += 1;
            }
            i += 1;

            n.append(&mut amt.to_string().chars().collect::<Vec<char>>());
            n.push(ans[i-1]);
        }

        *ans = n;
    }

    ans.len()
}

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap();
    let mut cur = inp.trim().chars().collect::<Vec<char>>();

    println!("Part 1: {}", solve(&mut cur, 40));
    println!("Part 2: {}", solve(&mut cur, 10)); 
}
