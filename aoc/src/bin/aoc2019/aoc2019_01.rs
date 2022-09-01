use aoc::read_to_numbers;

pub struct Aoc2019_01 {
    d: Vec<i32>
}

impl Aoc2019_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2019_01 {
    fn name(&self) -> (usize, usize) {
        (2019, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_numbers("input/2019/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.d.iter().map( | i | i / 3 - 2).sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for v in self.d.iter_mut() {
            while *v > 0 {
                *v = *v / 3 - 2;
                if *v > 0 {
                    s += *v;
                }
            }
        }
        
        crate::output(s)
    }
}