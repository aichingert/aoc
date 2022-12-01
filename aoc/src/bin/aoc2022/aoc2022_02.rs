pub struct Aoc2022_02 {
    d: Vec<i32>
}
        
impl Aoc2022_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_02 {
    fn name(&self) -> (usize, usize) {
        (2022, 02)
    }
        
    fn parse(&mut self) {
        self.d = aoc::read_number_stream("input/2022/02.txt", "");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
