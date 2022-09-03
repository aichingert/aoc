use aoc::get_chars;

pub struct Aoc2020_03 {
    d: Vec<Vec<char>>
}

impl Aoc2020_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn count(&self, s: &[usize]) -> usize {
        let mut x: usize = s[0];
        let mut y: usize = s[1];
        let mut c: usize = 0;
        while y < self.d.len() {
            if self.d[y][x] == '#' {
                c += 1;
            }
            x = (x + s[0]) % self.d[y].len();
            y += s[1];
        }
        c
    }
}

impl crate::Solution for Aoc2020_03 {
    fn name(&self) -> (usize, usize) {
        (2020, 3)
    }

    fn parse(&mut self) {
        self.d = get_chars("input/2020/03.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.count(&[3, 1]))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: usize = self.count(&[1, 1]);
        s *= self.count(&[3,1]);
        s *= self.count(&[5,1]);
        s *= self.count(&[7,1]);
        s *= self.count(&[1,2]);

        crate::output(s)
    }
}