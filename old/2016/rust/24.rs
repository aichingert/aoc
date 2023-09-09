// Advent of Code 2016, day 24
// (c) aichingert

#[path="../../utils/rust/permutations.rs"] mod permutations;
use std::collections::HashMap;

fn solve(map: &Vec<Vec<char>>) -> (u32,u32) {
    let mut steps: u32 = u32::MAX;
    let mut steps_with_return: u32 = u32::MAX;
    let mut positions: HashMap<char, (usize,usize)> = HashMap::new();
    let mut dists: HashMap<(char,char), u32> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' && map[i][j] != '#' {
                positions.insert(map[i][j], (i,j));
            }
        }
    }

    let mut vec = positions.keys().map(|&ch| ch).filter(|&ch| ch != '0').collect::<Vec<char>>();
    let mut perms = permutations::permutations(vec.len(), &mut vec);

    for perm in perms.iter_mut() {
        perm.insert(0, '0');
        steps = steps.min(perm_dist(perm, &mut dists, &positions, map));

        perm.push('0');
        steps_with_return = steps_with_return.min(perm_dist(perm, &mut dists, &positions, map));
    }

    (steps, steps_with_return)
}

fn perm_dist(perm: &Vec<char>, dists: &mut HashMap<(char,char), u32>, positions: &HashMap<char, (usize,usize)>, map: &Vec<Vec<char>>) -> u32 {
    (0..perm.len()-1).map(|i| {
        if !dists.contains_key(&(perm[i],perm[i+1])) {
            let dist = find_dist(perm[i], perm[i+1], &positions, map);
            dists.insert((perm[i],perm[i+1]), dist);
            dists.insert((perm[i+1],perm[i]), dist);
        }

        dists[&(perm[i], perm[i+1])]
    }).sum::<u32>()
}

fn find_dist(start: char, end: char, positions: &HashMap<char, (usize,usize)>, map: &Vec<Vec<char>>) -> u32 {
    let from = positions[&start];
    let to = positions[&end];
    let mut dists: HashMap<(usize,usize), u32> = HashMap::from([(from,0)]);
    let mut queue = vec![(from.0-1,from.1),(from.0+1,from.1),(from.0,from.1-1),(from.0,from.1+1)];

    while let Some((y,x)) = queue.pop() {
        if map[y][x] == '#' {
            continue;
        }

        let mut min = u32::MAX;
        min = min.min(*dists.entry((y + 1, x)).or_insert(u32::MAX));
        min = min.min(*dists.entry((y, x + 1)).or_insert(u32::MAX));
        min = min.min(*dists.entry((y - 1, x)).or_insert(u32::MAX));
        min = min.min(*dists.entry((y, x - 1)).or_insert(u32::MAX));
        min += 1;

        let cur: &mut u32 = dists.entry((y,x)).or_insert(u32::MAX);
        if min < *cur {
            *cur = min;

            queue.push((y-1,x));
            queue.push((y+1,x));
            queue.push((y,x-1));
            queue.push((y,x+1));
        }
    }

    *dists.get(&to).unwrap()
}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (part1,part2) = solve(&inp);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
