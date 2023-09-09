// Advent of Code 2017, day 14
// (c) aichingert

#[path="lib.rs"] mod lib;
use std::collections::HashSet;

fn solve(inp: &String) -> (u32, u32) {
    let mut grid = Vec::<Vec<char>>::new();
    let mut seen: HashSet<(usize,usize)> = HashSet::new();
    let mut regions: u32 = 0;

    for i in 0..128 {
        grid.push((&format!("{:#b}", u128::from_str_radix(&lib::knot_hash(&format!("{inp}-{i}")), 16).unwrap())[2..]).chars().rev().collect::<Vec<char>>());

        let len = grid.len()-1;
        for _ in grid[len].len()..128 {
            grid[len].push('0');
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if region(&mut seen, (i, j), &grid) {
                regions += 1;
            }
        }
    }

    (grid.iter().map(|g| g.iter().map(|ch| (*ch as u8 - '0' as u8) as u32).sum::<u32>()).sum::<u32>(), regions)
}

fn region(seen: &mut HashSet<(usize,usize)>, cur: (usize,usize), map: &Vec<Vec<char>>) -> bool {
    if seen.contains(&cur) || map[cur.0][cur.1] == '0' {
        return false;
    }
    seen.insert(cur);

    for dirs in [(0,1),(1,0),(0,-1),(-1,0)] {
        let y = (cur.0 as i32) + dirs.0;
        let x = (cur.1 as i32) + dirs.1;

        if !(x < 0 || x > (map.len() - 1) as i32 || y < 0 || y > (map.len() - 1) as i32) {
            region(seen, (y as usize, x as usize), map);
        }
    }

    true
}

fn main() {
    let inp = std::fs::read_to_string("../input/14").unwrap().trim().to_string();
    let (part1, part2) = solve(&inp);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
