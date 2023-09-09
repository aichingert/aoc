// Advent of Code 2016, day 13
// (c) aichingert

use std::collections::HashMap;

fn is_wall_at(x: i32, y: i32, fav: i32) -> bool {
    (x*x + 3*x + 2*x*y + y + y*y + fav).count_ones() & 1 == 1
}

fn part1(fav: i32, dist: &mut HashMap<(i32,i32), i32>) -> i32 {
    let mut stack = vec![(1,0),(0,1),(2,1),(1,2)];

    while let Some(loc) = stack.pop() {
        if is_wall_at(loc.0, loc.1, fav) {
            continue;
        }

        let mut min = i32::MAX;
        min = min.min(*dist.entry((loc.0 + 1, loc.1)).or_insert(i32::MAX));
        min = min.min(*dist.entry((loc.0, loc.1 + 1)).or_insert(i32::MAX));
        min = min.min(*dist.entry((loc.0 - 1, loc.1)).or_insert(i32::MAX));
        min = min.min(*dist.entry((loc.0, loc.1 - 1)).or_insert(i32::MAX));

        min += 1;
        let mut cur: &mut i32 = dist.entry(loc).or_insert(i32::MAX);
        if min < *cur {
            *cur = min;

            if loc.0 < 50 {  stack.push((loc.0 + 1, loc.1)); }
            if loc.1 < 50 {  stack.push((loc.0, loc.1 + 1)); }
            if loc.0 > 0 { stack.push((loc.0 - 1, loc.1)); }
            if loc.1 > 0 { stack.push((loc.0, loc.1 - 1));}
        }
    }


    *dist.get(&(31,39)).unwrap()
}
fn part2(dist: &HashMap<(i32,i32), i32>) -> usize {
    dist.values().filter(|&steps| *steps <= 50).count()
}

fn main() {
    let fav: i32 = std::fs::read_to_string("../input/13").unwrap().trim().parse::<i32>().unwrap();
    let mut dist = HashMap::from([((1,1), 0)]);

    println!("Part 1: {}", part1(fav, &mut dist));
    println!("Part 2: {}", part2(&dist));
}
