use std::{collections::HashMap, hash::Hash};

pub struct Aoc2022_03 {
    d: Vec<Vec<char>>
}
        
impl Aoc2022_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_03 {
    fn name(&self) -> (usize, usize) {
        (2022, 03)
    }
        
    fn parse(&mut self) {
        for line in std::fs::read_to_string("../input/2022/03.txt").expect("r").lines() {
            self.d.push(line.chars().collect());
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut score: u32 = 0;
        for i in 0..self.d.len() {
            let mut f: HashMap<char, i32> = HashMap::new();
            let mut s: HashMap<char, i32> = HashMap::new();
            for j in 0..self.d[i].len() / 2 {
                if f.contains_key(&self.d[i][j]) {
                    let v = f.remove(&self.d[i][j]).unwrap();
                    f.insert(self.d[i][j], v+1);
                } else {
                    f.insert(self.d[i][j], 1);
                }

                if s.contains_key(&self.d[i][self.d[i].len() - j - 1]) {
                    let v = s.remove(&self.d[i][self.d[i].len()-j-1]).unwrap();
                    s.insert(self.d[i][self.d[i].len()-j-1], v+1);
                } else {
                    s.insert(self.d[i][self.d[i].len()-j-1], 1);
                }
            }

            let mut highest: char = 'A';
            let mut h: i32 = 0;
            for k in f.keys() {
                if s.contains_key(k) {
                    if s[k] > h {
                        highest = *k;
                    }
                }
            }

            println!("{:?}", highest);

            if (highest as u8) > ('Z' as u8) {
                score += ((highest as u8) - ('a' as u8 - 1u8)) as u32;
            } else {
                score += ((highest as u8) - ('A' as u8 - 1u8)) as u32 + 26u32;
            }
        }

        crate::output(score)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut score: u32 = 0;
        let mut groups: Vec<Vec<Vec<char>>> = Vec::new();

        while self.d.len() > 0 {
            groups.push(vec![self.d.remove(0), self.d.remove(0), self.d.remove(0)]);
        }

        for i in 0..groups.len() {
            let mut f: HashMap<char, i32> = HashMap::new();
            let mut s: HashMap<char, i32> = HashMap::new();
            let mut t: HashMap<char, i32> = HashMap::new();

            for j in 0..groups[i][0].len() {
                if f.contains_key(&groups[i][0][j]) {
                    let v = f.remove(&groups[i][0][j]).unwrap();
                    f.insert(groups[i][0][j], v+1);
                } else {
                    f.insert(groups[i][0][j], 1);
                }
            }

            for j in 0..groups[i][1].len() {
                if s.contains_key(&groups[i][1][j]) {
                    let v = s.remove(&groups[i][1][j]).unwrap();
                    s.insert(groups[i][1][j], v+1);
                } else {
                    s.insert(groups[i][1][j], 1);
                }
            }

            for j in 0..groups[i][2].len() {
                if t.contains_key(&groups[i][2][j]) {
                    let v = t.remove(&groups[i][2][j]).unwrap();
                    t.insert(groups[i][2][j], v+1);
                } else {
                    t.insert(groups[i][2][j], 1);
                }
            }

            let mut highest: char = 'A';
            let mut h: i32 = 0;
            for k in f.keys() {
                if s.contains_key(k) && t.contains_key(k) {
                    if s[k] > h {
                        highest = *k;
                        h = s[k];
                    }
                }
            }

            if (highest as u8) > ('Z' as u8) {
                score += ((highest as u8) - ('a' as u8 - 1u8)) as u32;
            } else {
                score += ((highest as u8) - ('A' as u8 - 1u8)) as u32 + 26u32;
            }
        }

        crate::output(score)
    }
}