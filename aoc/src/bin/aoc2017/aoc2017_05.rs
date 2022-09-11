use aoc::read_to_numbers;

pub struct Aoc2017_05 {
    d: Vec<i32>
}
        
impl Aoc2017_05 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2017_05 {
    fn name(&self) -> (usize, usize) {
        (2017, 05)
    }
        
    fn parse(&mut self) {
        self.d = read_to_numbers("input/2017/05.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut p: i32 = 0;
        let mut s: i32 = 0;

        loop {
            if p >= self.d.len() as i32 {
                break;
            }

            self.d[p as usize] += 1;
            p += self.d[p as usize] - 1;
            s += 1;
        }

        crate::output(s)
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.parse();
        let mut p: i32 = 0;
        let mut s: i32 = 0;

        loop {
            if p >= self.d.len() as i32 {
                break;
            }

            let l = p;
            p += self.d[p as usize];
            self.d[l as usize] += if self.d[l as usize] >= 3 {
                -1
            } else {
                1
            };

            s += 1;
        }

        crate::output(s)
    }
}