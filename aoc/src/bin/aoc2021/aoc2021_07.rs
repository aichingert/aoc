pub struct Aoc2021_07 {
    d: Vec<i32>
}
        
impl Aoc2021_07 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2021_07 {
    fn name(&self) -> (usize, usize) {
        (2021, 07)
    }
        
    fn parse(&mut self) {
        self.d = aoc::("input/2021/07.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}