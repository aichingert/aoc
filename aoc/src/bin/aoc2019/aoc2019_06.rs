use std::collections::HashMap;

pub struct Aoc2019_06 {
    map: HashMap<String, Vec<String>>    
}
        
impl Aoc2019_06 {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }
    
    fn orbits(&self, current: &String, depth: i32) -> i32 {
        let mut sum: i32 = 0;
        
        if self.map.contains_key(current) {
            let vec: Vec<String> = self.map[current].clone();
            sum += depth;
            
            for star in vec.iter() {
                sum += self.orbits(star, depth+1);
            }
        } else {
            sum += depth;
        }
        
        sum
    }
}
        
impl crate::Solution for Aoc2019_06 {
    fn name(&self) -> (usize, usize) {
        (2019, 06)
    }
        
    fn parse(&mut self) {
        let inp: Vec<Vec<String>> = aoc::read_to_slice("input/2019/06.txt", ")");
        
        for i in 0..inp.len() {
            if self.map.contains_key(&inp[i][0]) {
                let mut stars: Vec<String> = self.map.remove(&inp[i][0]).unwrap();
                stars.push(inp[i][1].clone());
                self.map.insert(inp[i][0].clone(), stars);
            } else {
                self.map.insert(inp[i][0].clone(), vec![inp[i][1].clone()]);
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.orbits(&"COM".to_string(), 0))
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}