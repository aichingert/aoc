pub struct Aoc2021_08 {
    d: Vec<i32>
}
        
impl Aoc2021_08 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2021_08 {
    fn name(&self) -> (usize, usize) {
        (2021, 08)
    }
        
    fn parse(&mut self) {
        self.d = aoc::read_to_numbers("input/2021/08.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
