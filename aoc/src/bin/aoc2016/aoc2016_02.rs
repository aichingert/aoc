use aoc::slice;

pub struct Aoc2016_02 {
    d: Vec<String>
}

impl Aoc2016_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2016_02 {
    fn name(&self) -> (usize, usize) {
        (2016, 2)
    }

    fn parse(&mut self) {
        self.d = slice("input/2016/02.txt", "\r\n");
    }

    fn part1(&mut self) -> Vec<String> {
        let m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let mut o = String::new();

        for i in 0..self.d.len() {
            
        }


        crate::output(o)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}