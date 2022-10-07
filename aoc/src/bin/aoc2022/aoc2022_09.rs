pub struct Aoc2022_09 {
    d: Vec<i32>
}
        
impl Aoc2022_09 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_09 {
    fn name(&self) -> (usize, usize) {
        (2022, 09)
    }
        
    fn parse(&mut self) {
        self.d = aoc::("input/2022/09.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}