// Advent of Code 2017, day 5
// (c) aichingert

fn solve(inp: &mut Vec<i32>, part_two: bool) -> i32 {
    let mut ans = 0i32;
    let mut loc = 0usize;

    while loc < inp.len() {
        let prev = loc;
        loc = (loc as i32 + inp[loc]) as usize;

        if inp[prev] < 3 || !part_two {
            inp[prev] += 1;
        } else {
            inp[prev] -= 1;
        }
        
        ans += 1;
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/05").unwrap();
    let mut inp = inp.lines().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("Part 1: {}", solve(&mut inp.clone(), false));
    println!("Part 2: {}", solve(&mut inp, true));
}
