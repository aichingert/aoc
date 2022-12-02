pub struct Aoc2022_02 {
    d: Vec<i32>,
    rules: Vec<Vec<String>>
}
        
impl Aoc2022_02 {
    pub fn new() -> Self {
        Self { d: vec![], rules: Vec::new() }
    }
}
        
impl crate::Solution for Aoc2022_02 {
    fn name(&self) -> (usize, usize) {
        (2022, 02)
    }
        
    fn parse(&mut self) {
        let input = aoc::read_to_slice("input/2022/02.txt", " ");

        self.rules = input;
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut score: u32 = 0;

        for i in 0..self.rules.len() {
            match self.rules[i][0].as_str() {
                "A" => {
                    match self.rules[i][1].as_str() {
                        "X" => score += 4,
                        "Y" => score += 8,
                        "Z" => score += 3,
                        _ => panic!()
                    }
                },
                "B" => {
                    match self.rules[i][1].as_str() {
                        "X" => score += 1,
                        "Y" => score += 5,
                        "Z" => score += 9,
                        _ => panic!()
                    }
                },
                "C" => {
                    match self.rules[i][1].as_str() {
                        "X" => score += 7,
                        "Y" => score += 2,
                        "Z" => score += 6,
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        }
        crate::output(score)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut score: u32 = 0;
        for i in 0..self.rules.len() {
            match self.rules[i][0].as_str() {
                "A" => {
                    match self.rules[i][1].as_str() {
                        "X" => score += 3,
                        "Y" => score += 4,
                        "Z" => score += 8,
                        _ => panic!()
                    }
                },
                "B" => {
                    match self.rules[i][1].as_str() {
                        "X" => score += 1,
                        "Y" => score += 5,
                        "Z" => score += 9,
                        _ => panic!()
                    }
                },
                "C" => {
                    match self.rules[i][1].as_str() {
                        "X" => score += 2,
                        "Y" => score += 6,
                        "Z" => score += 7,
                        _ => panic!()
                    }
                }
                _ => panic!()
            }
        }
        crate::output(score)
    }
}
