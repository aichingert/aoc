// Advent of Code 2017, day 6
// (c) aichingert

use std::collections::HashMap;

fn solve(inp: &mut Vec<i32>) -> (usize, usize) {
    let mut storage = HashMap::<Vec<i32>, usize>::new();
    let len = inp.len();
    let mut cur = 0usize;

    while !storage.contains_key(inp) {
        storage.insert(inp.clone(), cur);
        let (mut start,mut count) = (0usize, 0i32);

        for i in 0..len {
            if inp[i] > count {
                start = i;
                count = inp[i];
            }
        }

        inp[start] = 0;
        start = (start+1) % len;

        while count > 0 {
            inp[start % len] += 1;
            start += 1;
            count -= 1;
        }

        cur += 1;
    }
    (cur,cur - storage[inp])
}

fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap();
    let mut inp = inp.trim().split('\t').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (cycles, loop_len) = solve(&mut inp);

    println!("Part 1: {cycles}");
    println!("Part 2: {loop_len}");
}
