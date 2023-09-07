// Advent of Code 2018, day 22
// (c) aichingert

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

fn geologic_index(gi: &mut HashMap<(i64, i64), i64>, x: i64, y: i64, depth: i64) -> i64 {
    if gi.contains_key(&(x, y)) {
        return gi[&(x, y)];
    }

    let geoi = match (x, y) {
        (0, _) => y * 48271,
        (_, 0) => x * 16807,
        (_, _) => {
            let a = (geologic_index(gi, x - 1, y, depth) + depth) % 20183;
            let b = (geologic_index(gi, x, y - 1, depth) + depth) % 20183;

            a * b
        }
    };
    gi.insert((x, y), geoi);

    gi[&(x, y)]
}

fn part1(depth: i64, x: i64, y: i64, gi: &mut HashMap<(i64, i64), i64>) -> i64 {
    (0..=y)
        .map(|i| (0..=x)
             .map(|j| (geologic_index(gi, j, i, depth) + depth) % 20183 % 3)
             .sum::<i64>())
        .sum::<i64>()
}

struct State {
    cost: u32,
    position: (i64, i64),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbours(p: (i64, i64), dist: &HashMap<(i64, i64), u32>, gi: &mut HashMap<(i64, i64), i64>) -> Vec<State> {
    Vec::new()
}

fn part_two(depth: i64, goal: (i64, i64), gi: &mut HashMap<(i64, i64), i64>) -> u32 {
    let mut dist = HashMap::new();
    let mut heap = BinaryHead::new();

    dist.insert((0,0), 0);
    heap.push(State { cost: 0, (0,0)});

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return cost;
        }

        if cost > dist.get(&position).or_insert(u32::MAX) { continue; }

    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/22").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();
    let depth: i64 = inp[0][7..].parse().unwrap();
    let (x, y): (&str, &str) = inp[1].split_once(',').unwrap();
    let (x, y): (i64, i64) = (x[8..].parse().unwrap(), y.parse().unwrap());
    let mut geologic_index: HashMap<(i64, i64), i64> = HashMap::from([((0, 0), 0), ((x, y), 0)]);

    println!("Part 1: {}", part1(depth, x, y, &mut geologic_index));
    println!("Part 2: {}", 10);
}
