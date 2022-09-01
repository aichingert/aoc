use aoc::read_to_numbers;
use std::collections::HashSet;

pub struct Aoc2018_01 {
    d: Vec<i32>
}

impl Aoc2018_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2018_01 {
    fn name(&self) -> (usize, usize) {
        (2018, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_numbers("input/2018/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.d.iter().sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut h: HashSet<i32> = HashSet::new();
        h.insert(0);
        let mut f: i32 = 0;
        
        loop {
            for i in 0..self.d.len() {
                f += self.d[i];

                if h.contains(&f) {
                    return crate::output(f);
                }

                h.insert(f);
            }
        }
    }
}