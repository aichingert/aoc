pub struct Aoc2022_03 {
    d: Vec<String>
}
        
impl Aoc2022_03 {
    pub fn new() -> Self {
        Self { d: Vec::new() }
    }
}
        
impl crate::Solution for Aoc2022_03 {
    fn name(&self) -> (usize, usize) {
        (2022, 03)
    }
        
    fn parse(&mut self) {
        self.d = aoc::slice("input/2022/03.txt", "\n");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut score: u32 = 0;

        for i in 0..self.d.len() {
            let f: &str = &self.d[i][..self.d[i].len() / 2];
            let s: &str = &self.d[i][self.d[i].len() / 2..];

            let c: char = f.chars().find(|ch| s.contains(*ch)).unwrap();
            score += if c.is_lowercase() {
                ((c as u8)  - ('a' as u8 - 1u8)) as u32
            } else {
                ((c as u8)  - ('A' as u8 - 1u8)) as u32 + 26u32
            }
        }

        crate::output(score)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut score: u32 = 0;

        for i in 0..self.d.len()/3 {
            let f: &str = &self.d[i*3+0].as_str();
            let s: &str = &self.d[i*3+1].as_str();
            let t: &str = &self.d[i*3+2].as_str();
 
            let c: char = f.chars().find(|ch| s.contains(*ch) && t.contains(*ch)).unwrap();
            score += if c.is_lowercase() {
                ((c as u8)  - ('a' as u8 - 1u8)) as u32
            } else {
                ((c as u8)  - ('A' as u8 - 1u8)) as u32 + 26u32
            }
        }

        crate::output(score)
    }
}
