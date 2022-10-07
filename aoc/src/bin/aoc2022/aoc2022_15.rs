pub struct Aoc2022_15 {
    d: Vec<i32>
}
        
impl Aoc2022_15 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_15 {
    fn name(&self) -> (usize, usize) {
        (2022, 15)
    }
        
    fn parse(&mut self) {
        self.d = aoc::("input/2022/15.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}