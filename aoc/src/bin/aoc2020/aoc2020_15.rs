use aoc::numbers;
use std::collections::HashMap;

pub struct Aoc2020_15 {
    d: Vec<Vec<i32>>
}

impl Aoc2020_15 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2020_15 {
    fn name(&self) -> (usize, usize) {
        (2020, 15)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2020/15.txt", ",");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut l: i32 = 0;
        let s: i32 = self.d[0].len() as i32 + 2;

        for i in 0..self.d[0].len() {
            m.insert(self.d[0][i], i as i32 + 1);
        }

        for i in s..=2020 {
            if m.contains_key(&l) {
                let h: i32 = m.remove(&l).unwrap();
                let v: i32 = i - 1;
                m.insert(l, v);

                l = v - h;
            } else {
                m.insert(l, i - 1);
                l = 0;
            }
        }

        crate::output(l)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        let mut l: i32 = 0;
        let s: i32 = self.d[0].len() as i32 + 2;

        for i in 0..self.d[0].len() {
            m.insert(self.d[0][i], i as i32 + 1);
        }

        for i in s..=30000000 {
            if m.contains_key(&l) {
                let h: i32 = m.remove(&l).unwrap();
                let v: i32 = i - 1;
                m.insert(l, v);

                l = v - h;
            } else {
                m.insert(l, i - 1);
                l = 0;
            }
        }

        crate::output(l)
    }
}