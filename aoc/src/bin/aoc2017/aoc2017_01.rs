use aoc::read_to_chars;

pub struct Aoc2017_01 {
    d: Vec<char>
}

impl Aoc2017_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2017_01 {
    fn name(&self) -> (usize, usize) {
        (2017, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_chars("input/2017/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: u32 = 0;

        if self.d[0] == self.d[self.d.len() - 1] {
            let n: u32 = self.d[0].to_digit(10).expect("invalid input");
            s += n;
        }

        for i in 0..self.d.len() - 1 {
            let n: u32 = self.d[i].to_digit(10).expect("invalid input");
            
            if self.d[i] == self.d[i + 1] {
                s += n;
            }
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: u32 = 0;
        let j: usize = self.d.len() / 2;

        for i in 0..self.d.len() - 1 {
            let n: u32 = self.d[i].to_digit(10).expect("invalid input");
            if i + j >= self.d.len() {
                if self.d[i] == self.d[i - j] {
                    s += n;
                }
            } else if self.d[i] == self.d[i + j] {
                s += n;
            }
        }

        crate::output(s)
    }
}