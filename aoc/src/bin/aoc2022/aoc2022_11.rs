pub struct Aoc2022_11 {
    d: Vec<i32>
}
        
impl Aoc2022_11 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_11 {
    fn name(&self) -> (usize, usize) {
        (2022, 11)
    }
        
    fn parse(&mut self) {
        self.d = aoc::("input/2022/11.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}