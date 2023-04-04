// Advent of Code 2016, day 24
// (c) aichingert

#[path="../../utils/rust/permutations.rs"] mod permutations;
use std::collections::HashMap;

fn part1(map: &Vec<Vec<char>>) -> u32 {
    let mut steps: u32 = u32::MAX;
    let mut positions: HashMap<char, (usize,usize)> = HashMap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' && map[i][j] != '#' {
                positions.insert(map[i][j], (i,j));
            }
        }
    }

    let mut vec = positions.keys().map(|&ch| ch).filter(|&ch| ch != '0').collect::<Vec<char>>();
    let mut perms = permutations::permutations(vec.len(), &mut vec);
    let mut i = 0;

    for perm in perms.iter_mut() {
        println!("{i}");
        perm.insert(0, '0');
        steps = steps.min(find_dist(perm, &positions, map));
        i += 1;
    }

    steps
}

fn find_dist(points: &Vec<char>, positions: &HashMap<char, (usize,usize)>, map: &Vec<Vec<char>>) -> u32 {
    let mut dist = 0;

    for i in 0..points.len()-1 {
        let from = positions[&points[i]];
        let to = positions[&points[i+1]];
        let mut path: HashMap<(usize,usize), u32> = HashMap::from([(from,0)]);
        let mut queue = vec![
            (from.0 - 1, from.1),
            (from.0 + 1, from.1),
            (from.0, from.1 - 1),
            (from.0, from.1 + 1),
        ];

        while let Some((y,x)) = queue.pop() {
            if map[y][x] == '#' {
                continue;
            }

            let mut min = u32::MAX;
            min = min.min(*path.entry((y + 1, x)).or_insert(u32::MAX));
            min = min.min(*path.entry((y, x + 1)).or_insert(u32::MAX));
            min = min.min(*path.entry((y - 1, x)).or_insert(u32::MAX));
            min = min.min(*path.entry((y, x - 1)).or_insert(u32::MAX));
            min += 1;

            let cur: &mut u32 = path.entry((y,x)).or_insert(u32::MAX);
            if min < *cur {
                *cur = min;

                queue.push((y-1,x));
                queue.push((y+1,x));
                queue.push((y,x-1));
                queue.push((y,x+1));
            }

            if path.contains_key(&to) && path[&to] < u32::MAX {
                break;
            }
        }

        dist += path.get(&to).unwrap();
    }

    dist
}

fn part2() {}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", part1(&inp));
    //println!("Part 2: {}", part2());
}
