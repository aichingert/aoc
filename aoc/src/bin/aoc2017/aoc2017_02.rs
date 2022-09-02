use aoc::numbers;

pub struct Aoc2017_02 {
    d: Vec<Vec<i32>>
}

impl Aoc2017_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2017_02 {
    fn name(&self) -> (usize, usize) {
        (2017, 2)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2017/02.txt", "\t");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let mut min: i32 = 10000;
            let mut max: i32 = 0;
            for j in 0..self.d[i].len() {
                min = std::cmp::min(min, self.d[i][j]);
                max = std::cmp::max(max, self.d[i][j]);
            }

            s += max - min;
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            'row: for j in 0..self.d[i].len() {
                for k in j+1..self.d[i].len() {
                    if self.d[i][j] % self.d[i][k] == 0 {
                        s += self.d[i][j] / self.d[i][k];
                        break 'row;
                    }
                    
                    if self.d[i][k] % self.d[i][j] == 0 {
                        s += self.d[i][k] / self.d[i][j];
                        break 'row;
                    }
                }                
            }
        }

        crate::output(s)
    }
}