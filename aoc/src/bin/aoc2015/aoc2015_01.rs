use aoc::read_to_chars;

#[derive(Debug)]
pub struct Aoc2015_01 {
    pub d: Vec<char>,
}

impl Aoc2015_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn solve(&self, pt: bool) -> i32 {
        let mut h: i32 = 0;
        for (p, c) in self.d.iter().enumerate() {
            match c {
                '(' => h+=1,
                ')' => h-=1,
                _ => panic!("invalid input!")
            }

            if pt && h < 0 {
                return p as i32 + 1;
            }
        }

        h
    }
}

impl crate::Solution for Aoc2015_01 {
    fn name(&self) -> (usize, usize) {
        (2015, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_chars("input/2015/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.solve(false))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(true))
    }
}
