pub struct Aoc2022_01 {
    d: [u32;3]
}
        
impl Aoc2022_01 {
    pub fn new() -> Self {
        Self { d: [0;3] }
    }
}
        
impl crate::Solution for Aoc2022_01 {
    fn name(&self) -> (usize, usize) {
        (2022, 01)
    }
        
    fn parse(&mut self) {
        let input: String = std::fs::read_to_string("input/2022/01.txt").unwrap();

        for block in input.split("\n\n") {
            let s: u32 = block.lines().map(|v| v.parse::<u32>().expect("invalid input")).sum::<u32>();
            if s > self.d[0] {
                self.d[2] = self.d[1];
                self.d[1] = self.d[0];
                self.d[0] = s;
            } else if s > self.d[1] {
                self.d[2] = self.d[1];
                self.d[1] = s;
            } else if s > self.d[2] {
                self.d[2] = s;
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.d[0])
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output(self.d[..].iter().sum::<u32>())
    }
}
