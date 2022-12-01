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
        let input: String = std::fs::read_to_string("input/2022/01.txt").unwrap();
        self.d.push(0);

        for line in input.lines() {
            if line.is_empty() {
                self.d.push(0);
            } else {
                let len = self.d.len()-1;
                self.d[len] += line.parse::<i32>().unwrap();
            }
        }

        self.d.sort();
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.d[self.d.len()-1])
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}