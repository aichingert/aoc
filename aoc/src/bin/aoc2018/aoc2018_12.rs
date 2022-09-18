use std::collections::HashMap;

pub struct Aoc2018_12 {
    current: Vec<(i32, char)>,
    rules: HashMap<Vec<char>, char>
}
        
impl Aoc2018_12 {
    pub fn new() -> Self {
        Self { 
            current: vec![],
            rules: HashMap::new()
        }
    }
}
        
impl crate::Solution for Aoc2018_12 {
    fn name(&self) -> (usize, usize) {
        (2018, 12)
    }
        
    fn parse(&mut self) {
        let mut p = aoc::read_to_slice("input/2018/12.txt", " ");
        
        for c in p[0][1].chars().enumerate() {
            self.current.push((c.0 as i32, c.1));
        }
        println!("{:?}", self.current);
    }
        
    fn part1(&mut self) -> Vec<String> {
       crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}