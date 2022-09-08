use aoc::read_to_chars;

pub struct Aoc2017_09 {
    d: Vec<char>,
    g: i32
}
        
impl Aoc2017_09 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
            g: 0 
        }
    }
}
        
impl crate::Solution for Aoc2017_09 {
    fn name(&self) -> (usize, usize) {
        (2017, 09)
    }
        
    fn parse(&mut self) {
        self.d = read_to_chars("input/2017/09.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;
        let mut c: i32 = 1;
        let mut j: bool = false;
        let mut g: bool = false;

        for i in 0..self.d.len() {
            if j {
                j = false;
                continue;
            }

            if g {
                match self.d[i] {
                    '>' => {
                        g = false;
                    },
                    '!' => {
                        j = true;
                    },
                    _ => {
                        self.g += 1;
                    }
                }
            }
            else {
                match self.d[i] {
                    '{' => {
                        s += c;
                        c += 1;
                    },
                    '}' => {
                        c -= 1;
                    },
                    '<' => {
                        g = true;
                    }
                    '!' => {
                        j = true;
                    },
                    _ => {}
                }
            }
        }

        crate::output(s)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}