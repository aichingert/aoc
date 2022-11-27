use aoc::read_to_numbers;

pub struct Aoc2021_01 {
    d: Vec<u32>
}

impl Aoc2021_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    pub fn solve(&self, size: usize) -> usize {
        // [0, 2, 5, 6, 1]
        // (0, 2, 5, 6)
        // (2, 5, 6, 1)
        // (2, 5, 6 => 0)
        // (0 < 1)
        self.d
            .windows(size)
            .filter(|w| w[0] < w[size - 1])
            .count()
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
        crate::output(self.solve(2))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(4))
    }
}
