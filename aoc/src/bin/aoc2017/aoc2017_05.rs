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
        let mut r = (self.d.clone(), vec![0; self.d.len()]);
        let mut p: i32 = 0;
        let mut s: i32 = 0;

        loop {
            if p >= r.0.len() as i32 {
                break;
            }
            let check: bool = if r.1[p as usize] >= 3 {
                true
            } else { 
                false
            };

            r.1[p as usize] += if check { -1 } else { 1 };
            p += r.1[p as usize] + r.0[p as usize] + if check { -1 } else { 1 };
            s += 1;

            if s % 10000 == 0 {
                println!("{s}")
            }
        }

        crate::output(s)
    }
}