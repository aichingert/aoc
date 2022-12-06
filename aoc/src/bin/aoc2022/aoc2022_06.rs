pub struct Aoc2022_06 {
    d: Vec<char>
}
        
impl Aoc2022_06 {
    pub fn new() -> Self {
        Self { d: vec![] }
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
        for i in 0..self.d.len() - 3 {
            let mut c: Vec<char> = Vec::new();
            let mut dup: bool = false;
            self.d[i..=i+3].to_vec().iter().for_each(|ch| if c.contains(ch) { dup = true; } else { c.push(*ch); });

            if !dup {
                return crate::output(i+4);
            }
        }
        
        panic!("invalid input!");
    }
        
    fn part2(&mut self) -> Vec<String> {
        for i in 0..self.d.len() - 3 {
            let mut c: Vec<char> = Vec::new();
            let mut dup: bool = false;
            self.d[i..=i+3].to_vec().iter().for_each(|ch| if c.contains(ch) { dup = true; } else { c.push(*ch); });

            if !dup {
                return crate::output(i+4);
            }
        }
        
        panic!("invalid input!");
    }
}