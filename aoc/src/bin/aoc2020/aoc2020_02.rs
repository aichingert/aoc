use aoc::read_to_slice;

pub struct Aoc2020_02 {
    d: Vec<Vec<String>>,
}

impl Aoc2020_02 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
        }
    }
}

impl crate::Solution for Aoc2020_02 {
    fn name(&self) -> (usize, usize) {
        (2020, 2)
    }

    fn parse(&mut self) {
        self.d = read_to_slice("input/2020/02.txt", " ");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let n: Vec<i32> = self.d[i][0].split('-').map(| v | v.parse::<i32>().expect("invalid input")).collect();
            let mut c: i32 = 0;

            for j in 0..self.d[i][2].len() {
                if self.d[i][2][j..=j] == self.d[i][1][0..=self.d[i][1].len()-2] {
                    c += 1;
                }
            }

            if c >= n[0] && c <= n[1] {
                s += 1;
            }
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let n: Vec<usize> = self.d[i][0].split('-').map(| v | v.parse::<usize>().expect("invalid input") - 1).collect();
            let l = &self.d[i][1][0..=self.d[i][1].len()-2];

            match (&self.d[i][2][n[0]..=n[0]] == l, &self.d[i][2][n[1]..=n[1]] == l) {
                (true, false) => s += 1,
                (false, true) => s += 1,
                _ => {}
            }
        }

        crate::output(s)
    }
}