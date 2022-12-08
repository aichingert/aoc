use aoc::read_to_chars;
use std::collections::HashSet;

pub struct Aoc2015_03 {
    d: Vec<char>
}

impl Aoc2015_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn solve(&self, p2: bool) -> usize {
        let mut set: HashSet<(i32, i32)> = HashSet::new();
        let mut x: [i32; 2] = [0,0];
        let mut y: [i32; 2] = [0,0];
        let mut idx: usize = 0;

        for i in 0..self.d.len() {
            set.insert((x[idx], y[idx]));
            match self.d[i] {
                '<' => x[idx] -= 1,
                '>' => x[idx] += 1,
                '^' => y[idx] += 1,
                'v' => y[idx] -= 1,
                _ => panic!("invalid char in input")
            }

            if p2 {
                idx = 1 - idx;
            }
        }
        set.insert((x[idx], y[idx]));
        set.len()
    }
}

impl crate::Solution for Aoc2015_03 {
    fn name(&self) -> (usize, usize) {
        (2015, 3)
    }

    fn parse(&mut self) {
        self.d = read_to_chars("input/2015/03.txt")
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.solve(false))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(true))
    }
}