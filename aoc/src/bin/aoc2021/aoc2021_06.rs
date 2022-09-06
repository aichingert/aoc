use aoc::numbers;

pub struct Aoc2021_06 {
    d: Vec<Vec<i32>>
}

impl Aoc2021_06 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2021_06 {
    fn name(&self) -> (usize, usize) {
        (2021, 6)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2021/06.txt", ",");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}