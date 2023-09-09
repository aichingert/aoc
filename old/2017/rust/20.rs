// Advent of Code 2017, day 20
// (c) aichingert

const ESTIMATION: usize = 500;
use std::collections::HashSet;

#[derive(Clone)]
struct Particle {
    loc: [i64;3],
    vel: [i64;3],
    acc: [i64;3],
}

impl Particle {
    fn new(loc: &[i64], vel: &[i64], acc: &[i64]) -> Self {
        Self {
            loc: [loc[0],loc[1],loc[2]],
            vel: [vel[0],vel[1],vel[2]],
            acc: [acc[0],acc[1],acc[2]],
        }
    }

    fn update(&mut self) {
        for i in 0..3 {
            self.vel[i] += self.acc[i];
            self.loc[i] += self.vel[i];
        }
    }
}

fn part1(particles: &mut Vec<Particle>) -> usize {
    for _ in 0..ESTIMATION {
        particles.iter_mut().for_each(|p| p.update());
    }

    let mut cur = i64::MAX;
    let mut ans = 0;

    for i in 0..particles.len() {
        let sum = particles[i].loc.iter().map(|n| n.abs()).sum::<i64>();

        if sum < cur {
            ans = i;
            cur = sum;
        }
    }

    ans
}

fn part2(particles: &mut Vec<Particle>) -> usize {
    for _ in 0..ESTIMATION {
        let mut remove = HashSet::new();

        for i in 0..particles.len()-1 {
            for j in i+1..particles.len() {
                if particles[i].loc == particles[j].loc {
                    remove.insert(i);
                    remove.insert(j);
                }
            }
        }

        let mut offset = 0;
        let mut v = remove.iter().map(|&i| i).collect::<Vec<usize>>();
        v.sort();
        for loc in v.iter() {
            particles.remove(loc - offset);
            offset += 1;
        }

        particles.iter_mut().for_each(|p| p.update());
    }

    particles.len()
}

fn parse() -> Vec<Particle> {
    std::fs::read_to_string("../input/20").unwrap()
        .lines()
        .map(|l| {
            let mut values = Vec::<Vec<i64>>::new();
            let sp = format!("{l}, ");
            let vls = sp.split(">, ").collect::<Vec<&str>>();

            for i in 0..vls.len()-1 {
                let inner = vls[i].split("=<").collect::<Vec<&str>>();
                let nums = inner[1].split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();
                values.push(nums);
            }
            Particle::new(&values[0],&values[1],&values[2])
        }).collect::<Vec<Particle>>()
}

fn main() {
    let mut particles = parse();

    println!("Part 1: {}", part1(&mut particles.clone()));
    println!("Part 2: {}", part2(&mut particles));
}
