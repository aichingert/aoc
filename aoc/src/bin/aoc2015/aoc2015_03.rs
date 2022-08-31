use aoc::read_to_chars;
use std::collections::HashSet;

pub struct Aoc2015_03 {
    d: Vec<char>
}

impl Aoc2015_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
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
        let mut m: HashSet<(i32, i32)> = HashSet::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;

        for i in 0..self.d.len() {
            m.insert((x, y));
            match self.d[i] {
                '<' => x -= 1,
                '>' => x += 1,
                '^' => y += 1,
                'v' => y -= 1,
                _ => panic!("invalid char in input")
            }
        }
        m.insert((x, y));

        crate::output(m.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut m: HashSet<(i32, i32)> = HashSet::new();
        let mut x = [0, 0];
        let mut y = [0, 0];
        let mut w: usize = 0;
        for i in 0..self.d.len() {
            m.insert((x[w], y[w]));
            match self.d[i] {
                '<' => x[w] -= 1,
                '>' => x[w] += 1,
                '^' => y[w] += 1,
                'v' => y[w] -= 1,
                _ => panic!("invalid char in input")
            }
            w = 1 - w;
        }
        m.insert((x[w], y[w]));

        crate::output(m.len())
    }
}