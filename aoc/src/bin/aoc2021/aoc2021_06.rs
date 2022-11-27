use std::collections::VecDeque;

pub struct Aoc2021_06 {
    l: VecDeque<u64>
}

impl Aoc2021_06 {
    pub fn new() -> Self {
        Self { l: VecDeque::from(vec![0; 9]) }
    }

    fn solve(&mut self, steps: u32) -> u64 {
        for _ in 0..steps {
            let n: u64 = self.l.pop_front().unwrap();
            self.l[6] += n;
            self.l.push_back(n);
        }
        
        self.l.iter().sum::<u64>()
    }
}

impl crate::Solution for Aoc2021_06 {
    fn name(&self) -> (usize, usize) {
        (2021, 6)
    }

    fn parse(&mut self) {
        aoc::read_number_stream::<&str, usize>("input/2021/06.txt", ",")
            .iter().for_each(|i| self.l[*i] += 1);
    }

    fn part1(&mut self) -> Vec<String> {
       crate::output(self.solve(80))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(/* 256 - 80 = */176))
    }
}
