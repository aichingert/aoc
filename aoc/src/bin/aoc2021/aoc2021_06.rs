use aoc::numbers;

pub struct Aoc2021_06 {
    d: Vec<Vec<usize>>
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
        let mut l: [i32; 9] = [0; 9];

        for i in 0..self.d[0].len() {
            l[self.d[0][i]] += 1;
        }

        for _ in 0..80 {
            let n = l[0];

            for i in 1..l.len() {
                l[i - 1] = l[i];
            }

            l[6] += n;
            l[8] = n;
        }

        crate::output(l.iter().sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}