// Advent of Code 2017, day 14
// (c) aichingert

#[path="./knot_hash.rs"] mod knot_hash;
use std::collections::HashSet;

fn solve(inp: &String) -> u32 {
    let mut grid = Vec::<Vec<char>>::new();
    let mut seen: HashSet<(usize,usize)> = HashSet::new();

    for i in 0..128 {
        grid.push((&format!("{:#b}", u128::from_str_radix(&knot_hash::knot_hash(&format!("{inp}-{i}")), 16).unwrap())[2..]).chars().collect::<Vec<char>>());
    }

    grid.iter().map(|g| g.iter().map(|ch| (*ch as u8 - '0' as u8) as u32).sum::<u32>()).sum::<u32>()
}

//fn region(seen: &mut HashSet<(usize,usize)>, cur: (usize,usize), map: &Vec<vec<char>>)

fn main() {
    let inp = std::fs::read_to_string("../input/14").unwrap().trim().to_string();

    println!("Part 1: {}", solve(&inp));
}
