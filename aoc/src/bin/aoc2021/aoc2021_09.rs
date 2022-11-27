pub struct Aoc2021_09 {
    m: Vec<Vec<u8>>
}
        
impl Aoc2021_09 {
    pub fn new() -> Self {
        Self { m: vec![] }
    }
}
        
impl crate::Solution for Aoc2021_09 {
    fn name(&self) -> (usize, usize) {
        (2021, 09)
    }
        
    fn parse(&mut self) {
        self.m = aoc::read_to_map("input/2021/09.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut risk: i32 = 0;
        for i in 0..self.m.len() {
            for j in 0..self.m[i].len() {
                if (i + 1 == self.m.len() || self.m[i][j] < self.m[i+1][j])
                && (j + 1 == self.m[i].len() || self.m[i][j] < self.m[i][j+1])
                && (i as i32 - 1 == -1 || self.m[i][j] < self.m[i-1][j])
                && (j as i32 - 1 == -1 || self.m[i][j] < self.m[i][j-1]) {
                    risk += self.m[i][j] as i32 +1;
                }
           }
        }

        crate::output(risk)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
