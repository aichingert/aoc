pub struct Aoc2022_02 {
    s: Vec<Vec<String>>
}
        
impl Aoc2022_02 {
    pub fn new() -> Self {
        Self { s: Vec::new() }
    }
}
        
impl crate::Solution for Aoc2022_02 {
    fn name(&self) -> (usize, usize) {
        (2022, 02)
    }
        
    fn parse(&mut self) {
        self.s = aoc::read_to_slice("input/2022/02.txt", " ");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut score: u32 = 0;

        for i in 0..self.s.len() {
            match self.s[i][0].as_str() {
                "A" => {
                    match self.s[i][1].as_str() {
                        "X" => score += 4,
                        "Y" => score += 8,
                        "Z" => score += 3,
                        _ => panic!("Invalid input, {:?}", self.s[i][1]),
                    }
                },
                "B" => {
                    match self.s[i][1].as_str() {
                        "X" => score += 1,
                        "Y" => score += 5,
                        "Z" => score += 9,
                        _ => panic!("Invalid input, {:?}", self.s[i][1]),
                    }
                },
                "C" => {
                    match self.s[i][1].as_str() {
                        "X" => score += 7,
                        "Y" => score += 2,
                        "Z" => score += 6,
                        _ => panic!("Invalid input, {:?}", self.s[i][1]),
                    }
                },
                _ => panic!("Invalid input, {:?}", self.s[i][1]),
            }
        }
        crate::output(score)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut score: u32 = 0;
        for i in 0..self.s.len() {
            match self.s[i][0].as_str() {
                "A" => {
                    match self.s[i][1].as_str() {
                        "X" => score += 3,
                        "Y" => score += 4,
                        "Z" => score += 8,
                        _ => panic!("Invalid input, {:?}", self.s[i][1]),
                    }
                },
                "B" => {
                    match self.s[i][1].as_str() {
                        "X" => score += 1,
                        "Y" => score += 5,
                        "Z" => score += 9,
                        _ => panic!("Invalid input, {:?}", self.s[i][1]),
                    }
                },
                "C" => {
                    match self.s[i][1].as_str() {
                        "X" => score += 2,
                        "Y" => score += 6,
                        "Z" => score += 7,
                        _ => panic!("Invalid input, {:?}", self.s[i][1]),
                    }
                }
                _ => panic!("Invalid input, {:?}", self.s[i][1]),
            }
        }
        crate::output(score)
    }
}
