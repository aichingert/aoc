use aoc::read_to_slice;
use std::collections::HashMap;

pub struct Aoc2017_04 {
    d: Vec<Vec<String>>
}

impl Aoc2017_04 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn get_letters(s: &String) -> HashMap<char, i32> {
        let mut h: HashMap<char, i32> = HashMap::new();
        
        for c in s.chars() {
            let o = h.remove(&c);
            
            if let Some(v) = o {
                h.insert(c, v + 1);
            } else {
                h.insert(c, 1);
            }
        }

        h
    }
}

impl crate::Solution for Aoc2017_04 {
    fn name(&self) -> (usize, usize) {
        (2017, 4)
    }

    fn parse(&mut self) {
        self.d = read_to_slice("input/2017/04.txt", " ");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let mut c: Vec<&String> = vec![];
            let mut v: bool = true;
            for j in 0..self.d[i].len() {
                if c.contains(&&self.d[i][j]) {
                    v = false;
                    break;
                }

                c.push(&self.d[i][j]);
            }

            if v {
                s += 1;
            }
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let mut h: Vec<HashMap<char, i32>> = vec![];
            let mut v: bool = false;
            for j in 0..self.d[i].len() {
                h.push(Aoc2017_04::get_letters(&self.d[i][j]));
            }

            'main: for k in 0..h.len() {
                for l in k+1..h.len() {
                    if h[l].len() != h[k].len() {
                        v = true;
                        continue;
                    }
                    v = false;
                    for c in h[k].keys() {
                        if h[l].contains_key(c) {
                            if h[l][c] != h[k][c] {
                                v = true;
                            }
                        } else {
                            v = true;
                        }
                    }

                    if !v {
                        break 'main;
                    }
                }
            }

            if v {
                s += 1;
            }
        }

        crate::output(s)
    }
}