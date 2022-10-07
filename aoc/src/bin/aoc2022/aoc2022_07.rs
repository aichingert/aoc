pub struct Aoc2022_07 {
    d: Vec<i32>
}
        
impl Aoc2022_07 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_07 {
    fn name(&self) -> (usize, usize) {
        (2022, 07)
    }
        
    fn parse(&mut self) {
        self.d = aoc::("input/2022/07.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}