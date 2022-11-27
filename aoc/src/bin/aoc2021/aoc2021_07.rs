pub struct Aoc2021_07 {
    d: Vec<i32>
}
        
impl Aoc2021_07 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn allign(&self, height: i32) -> i32 {
        let mut fuel: i32 = 0;

        for i in 0..self.d.len() {
            fuel += (self.d[i] - height).abs();
        }

        fuel
    }
}
        
impl crate::Solution for Aoc2021_07 {
    fn name(&self) -> (usize, usize) {
        (2021, 07)
    }
        
    fn parse(&mut self) {
        self.d = aoc::read_number_stream("input/2021/07.txt", ",");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut m: i32 = i32::MAX;

        for i in 0..500 {
            m = std::cmp::min(m, self.allign(i));
        }

        crate::output(m)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
