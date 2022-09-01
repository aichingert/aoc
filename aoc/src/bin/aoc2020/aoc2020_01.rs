use aoc::read_to_numbers;

pub struct Aoc2020_01 {
    d: Vec<i32>
}

impl Aoc2020_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2020_01 {
    fn name(&self) -> (usize, usize) {
        (2020, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_numbers("input/2020/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        for i in 0..self.d.len() {
            for j in i+1..self.d.len() {
                if i != j && self.d[i] + self.d[j] == 2020 {
                    return crate::output(self.d[i] * self.d[j])
                }
            }
        }

        panic!("no solution found")
    }

    fn part2(&mut self) -> Vec<String> {
        for i in 0..self.d.len() {
            for j in i+1..self.d.len() {
                for k in j+1..self.d.len() {
                    if i != j && j != k && self.d[i] + self.d[j] + self.d[k] == 2020 {
                        return crate::output(self.d[i] * self.d[j] * self.d[k])
                    }
                }
            }
        }

        panic!("no solution found")
    }
}