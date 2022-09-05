use aoc::read_to_slice;
use std::collections::HashMap;

pub struct Aoc2016_04 {
    d: Vec<Vec<String>>,
    v: Vec<String>
}

impl Aoc2016_04 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
            v: vec![]
        }
    }
}

impl crate::Solution for Aoc2016_04 {
    fn name(&self) -> (usize, usize) {
        (2016, 4)
    }

    fn parse(&mut self) {
        self.d = read_to_slice("input/2016/04.txt", "-");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let c: Vec<&str> = self.d[i][self.d[i].len() - 1].split('[').collect();
            let mut l: HashMap<char, i32> = HashMap::new();
            let mut v: Vec<char> = vec![];

            for j in 0..self.d[i].len() - 1 {
                for c in self.d[i][j].chars() {
                    if l.contains_key(&c) {
                        let num: i32 = l.remove(&c).unwrap();
                        l.insert(c, num + 1);
                    } else {
                        l.insert(c, 1);
                        v.push(c);
                    }
                }
            }

            let mut p: usize = 0;
            let mut t: bool = true;

            for c in c[1].chars() {
                if t {
                    if l.contains_key(&c) {
                        for j in 0..v.len() {
                            if !(l[&c] >= l[&v[j]]) && v[j] != c {
                                t = false;
                                break;
                            } else if v[j] == c {
                                p = j;
                            }
                        }
                    } else if c != ']' {
                        t = false;
                    }

                    if t && c != ']' {
                        v.remove(p);
                        l.remove(&c);
                    }
                } else {
                    break;
                }
            }

            if t {
                s += c[0].parse::<i32>().unwrap();
                self.v.push(self.d[i].iter().cloned().collect::<String>());
            }
        }
        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}