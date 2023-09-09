// Advent of Code 2019, day 12
// (c) aichingert

#[path="../../utils/rust/math.rs"] mod math;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Momentum {
    pos: i16,
    vel: i16,
}

impl Momentum {
    pub fn new(pos: i16) -> Self {
        Self { pos, vel: 0 }
    }
}

fn step(axis: &mut Vec<Momentum>) {
    for i in 0..axis.len() {
        for j in 0..i {
            let (dvi, dvj) = match axis[i].pos.cmp(&axis[j].pos) {
                Ordering::Less => (1,-1),
                Ordering::Greater => (-1,1),
                Ordering::Equal => (0,0),
            };

            axis[i].vel += dvi;
            axis[j].vel += dvj;
        }
    }

    for momentum in axis.iter_mut() {
        momentum.pos += momentum.vel;
    }
}

fn cycle(axis: &mut Vec<Momentum>) -> usize {
    let mut seen = HashSet::new();

    while !seen.contains(axis) {
        seen.insert(axis.clone());
        step(axis);
    }

    seen.len()
}

fn part1(axes: &mut Vec<Vec<Momentum>>) -> i16 {
    for a in axes.iter_mut() {
        for _ in 0..100 {
            step(a);
        }
    }

    (0..axes[0].len()).fold(0, |acc,i| 
        acc + (0..3).map(|loc| axes[loc][i].pos.abs()).sum::<i16>()
            * (0..3).map(|loc| axes[loc][i].vel.abs()).sum::<i16>())
}

fn part2(axes: &mut Vec<Vec<Momentum>>) -> usize {
    (0..axes.len()).fold(1, |acc,i| math::lcm(acc, cycle(&mut axes[i])))
}

fn input() -> Vec<Vec<Momentum>> {
    let mut axes = vec![vec![], vec![], vec![]];

    for line in std::fs::read_to_string("../input/12").unwrap().lines() {
        let vls = line.split(", ").collect::<Vec<_>>();
        let cords = (0..3).map(|i| vls[i].split_once('=').unwrap().1).collect::<Vec<_>>();

        axes[0].push(Momentum::new(cords[0].parse::<i16>().unwrap()));
        axes[1].push(Momentum::new(cords[1].parse::<i16>().unwrap()));
        axes[2].push(Momentum::new(cords[2][0..cords[2].len()-1].parse::<i16>().unwrap()));

    }

    axes
}

fn main() {
    let mut axes = input();

    println!("Part 1: {}", part1(&mut axes));
    println!("Part 2: {}", part2(&mut axes));
}
