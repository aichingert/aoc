use aoc::numbers;

pub struct Aoc2016_03 {
    d: Vec<Vec<i32>>
}

impl Aoc2016_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn check_side_len(n1: i32, n2: i32, n3: i32) -> bool {
        if n1 + n2 > n3 && n2+ n3 > n1 && n1 + n3 > n2 {
            return true;
        }

        false
    }
}

impl crate::Solution for Aoc2016_03 {
    fn name(&self) -> (usize, usize) {
        (2016, 3)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2016/03.txt", " ");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.d.iter().map( | v | if Aoc2016_03::check_side_len(v[0], v[1], v[2]) { 1 } else { 0 }).sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;
        let mut i: usize = 0;
        while i < self.d.len() - 2 {
            if Aoc2016_03::check_side_len(self.d[i][0], self.d[i + 1][0], self.d[i + 2][0]) {
                s += 1;
            }
            if Aoc2016_03::check_side_len(self.d[i][1], self.d[i + 1][1], self.d[i + 2][1]) {
                s += 1;
            }
            if Aoc2016_03::check_side_len(self.d[i][2], self.d[i + 1][2], self.d[i + 2][2]) {
                s += 1;
            }

            i += 3;
        }
        crate::output(s)
    }
}
