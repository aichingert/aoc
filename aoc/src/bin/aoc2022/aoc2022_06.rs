pub struct Aoc2022_06 {
    d: Vec<char>
}
        
impl Aoc2022_06 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn solve(&self, range: usize) -> usize {
        for i in 0..self.d.len() - range {
            let mut c: Vec<char> = Vec::new();
            let mut dup: bool = false;
            self.d[i..i+range].to_vec().iter().for_each(|ch| if c.contains(ch) { dup = true; } else { c.push(*ch); });

            if !dup {
                return i+range;
            }
        }
        
        panic!("invalid input!");
    }
}
        
impl crate::Solution for Aoc2022_06 {
    fn name(&self) -> (usize, usize) {
        (2022, 06)
    }
        
    fn parse(&mut self) {
        self.d = aoc::read_to_chars("input/2022/06.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.solve(4))
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(14))
    }
}