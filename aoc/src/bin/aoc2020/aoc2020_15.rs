use std::collections::HashMap;

pub struct Aoc2020_15 {
    m: HashMap<u32,u32>
}

impl Aoc2020_15 {
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn solve(&self, s: u32, m: &mut HashMap<u32,u32>) -> u32 {
        let mut l: u32 = 0;

        for i in m.len() as u32 + 2u32..=s {
            if m.contains_key(&l) {
                let h: u32 = m.remove(&l).unwrap();
                let v: u32 = i - 1;
                m.insert(l, v);

                l = v - h;
            } else {
                m.insert(l, i - 1);
                l = 0;
            }
        }

        l
    }
}

impl crate::Solution for Aoc2020_15 {
    fn name(&self) -> (usize, usize) {
        (2020, 15)
    }

    fn parse(&mut self) {
        self.m = HashMap::from_iter(
            aoc::read_number_stream("input/2020/15.txt", ",")
                .iter()
                .enumerate()
                .map(|(p,v)| (*v, p as u32 + 1)));
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.solve(2020, &mut self.m.clone()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(30000000, &mut self.m.clone()))
    }
}