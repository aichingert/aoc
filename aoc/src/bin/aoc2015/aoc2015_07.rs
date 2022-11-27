use std::collections::HashMap;

pub struct Aoc2015_07 {
    c: HashMap<String, Command>,
    r: HashMap<String, u32>,
    p: u32
}

#[derive(Debug, Clone)]
enum Command {
    Assign(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u32),
    RShift(String, u32)
}
        
impl Aoc2015_07 {
    pub fn new() -> Self {
        Self { 
            c: HashMap::new(),
            r: HashMap::new(),
            p: 0
        }
    }

    pub fn solve(&mut self, cur: &str) -> u32 {
        let result: Result<u32, _> = cur.parse::<u32>();

        match result {
            Ok(value) => return value,
            Err(_) => {}
        }

        if self.r.contains_key(cur) {
            return self.r[cur];
        }

        let command: Command = self.c[cur].clone();

        let result = match command {
            Command::Assign(x) => self.solve(&x),
            Command::Not(x) => !self.solve(&x),
            Command::And(x, y) => self.solve(&x) & self.solve(&y),
            Command::Or(x, y) => self.solve(&x) | self.solve(&y),
            Command::LShift(x, y) => self.solve(&x) << y,
            Command::RShift(x, y) => self.solve(&x) >> y
        };

        self.r.insert(cur.to_string(), result);
        result
    }
}
        
impl crate::Solution for Aoc2015_07 {
    fn name(&self) -> (usize, usize) {
        (2015, 07)
    }
        
    fn parse(&mut self) {
        let mut lines: Vec<Vec<String>> = aoc::read_to_slice("input/2015/07.txt", " -> ");

        for i in 0..lines.len() {
            let to: String = lines[i][1].clone();
            let from: Vec<&str> = lines[i][0].split(' ').collect::<Vec<&str>>();

            match from.len() {
                1 => self.c.insert(to, Command::Assign(from[0].to_string())),
                2 => self.c.insert(to, Command::Not(from[1].to_string())),
                _ => match from[1] {
                    "AND" => self.c.insert(to, Command::And(from[0].to_string(), from[2].to_string())),
                    "OR" => self.c.insert(to, Command::Or(from[0].to_string(), from[2].to_string())),
                    "LSHIFT" => self.c.insert(to, Command::LShift(from[0].to_string(), from[2].parse::<u32>().expect("invalid input"))),
                    "RSHIFT" => self.c.insert(to, Command::RShift(from[0].to_string(), from[2].parse::<u32>().expect("invalid input"))),
                    _ => panic!("Invalid input {:?}", lines[i])
                }
            };
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        self.p = self.solve("a");
        crate::output(self.p)
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.r = HashMap::from([("b".to_string(), self.p)]);
        crate::output(self.solve("a"))
    }
}
