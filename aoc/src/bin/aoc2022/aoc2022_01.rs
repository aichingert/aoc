pub struct Aoc2022_01 {
    d: Vec<i32>
}
        
impl Aoc2022_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_01 {
    fn name(&self) -> (usize, usize) {
        (2022, 01)
    }
        
    fn parse(&mut self) {
        let input: Vec<Vec<i32>> = aoc::numbers("input/2022/01.txt", " ");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}