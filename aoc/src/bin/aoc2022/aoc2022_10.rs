pub struct Aoc2022_10 {
    d: Vec<i32>
}
        
impl Aoc2022_10 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_10 {
    fn name(&self) -> (usize, usize) {
        (2022, 10)
    }
        
    fn parse(&mut self) {
        self.d = aoc::("input/2022/10.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}