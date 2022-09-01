use aoc::read_to_numbers;

pub struct Aoc2021_01 {
    d: Vec<i32>
}

impl Aoc2021_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2021_01 {
    fn name(&self) -> (usize, usize) {
        (2021, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_numbers("input/2021/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 1..self.d.len() {
            if self.d[i] > self.d[i - 1] {
                s += 1;
            }
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 3..self.d.len() {
            let a = self.d[i] + self.d[i - 1] + self.d[i - 2];
            let b = self.d[i - 1] + self.d[i - 2] + self.d[i - 3];
            if a > b {
                s+=1;
            }
        }

        crate::output(s)
    }
}