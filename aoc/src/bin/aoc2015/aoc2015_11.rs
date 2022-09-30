pub struct Aoc2015_11 {
    d: Vec<u8>
}
        
impl Aoc2015_11 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn tick(&mut self) {
        let len: usize = self.d.len();

        for i in 1..len {
            self.d[len - i] += 1;

            if self.d[len - i] > 122 {
                self.d[len -i] = 97;
            } else {
                break;
            }
        }
    }

    fn is_valid(&mut self)-> bool {
        let mut is_valid: bool = false;
    
        for i in 0..self.d.len() - 2 {
            if self.d[i] + 1 == self.d[i + 1] && self.d[i] + 2 == self.d[i + 2] {
                is_valid = true;
                break;
            }
        }
    
        if !is_valid {
            return is_valid;
        }
    
        for i in 0..self.d.len() {
            if self.d[i] == 105 || self.d[i] == 111 || self.d[i] == 108 {
                return false;
            }
        }
    
        is_valid = false;
        let mut appear_byte: u8 = 0;
    
        for i in 0..self.d.len() - 1 {
            if self.d[i] == self.d[i + 1] {
                is_valid = true;
                appear_byte = self.d[i];
                break;
            }
        }
    
        if !is_valid {
            return false;
        }
    
        is_valid = false;
    
        for i in 0..self.d.len() - 1 {
            if self.d[i] == self.d[i + 1] && self.d[i] != appear_byte {
                is_valid = true;
            }
        }
    
        is_valid
    }

    fn solve(&mut self) -> String {
        while !self.is_valid() {
            self.tick();
        }

        self.d.iter().map(|c| *c as char).collect::<String>()
    }
}
        
impl crate::Solution for Aoc2015_11 {
    fn name(&self) -> (usize, usize) {
        (2015, 11)
    }
        
    fn parse(&mut self) {
        for c in aoc::read_to_chars("input/2015/11.txt") {
            self.d.push(c as u8);
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.solve())
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.tick();
        crate::output(self.solve())
    }
}