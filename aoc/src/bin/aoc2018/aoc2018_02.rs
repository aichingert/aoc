use aoc::slice;
use std::collections::HashMap;

pub struct Aoc2018_02 {
    d: Vec<String>
}

impl Aoc2018_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn find_diff(s1: &String, s2: &String) -> usize {
        if s1.len() != s2.len() {
            return 100;
        }

        let mut d: usize = 0;

        for i in 0..s1.len() {
            if s1[i..=i] != s2[i..=i] {
                d += 1;
            }
        }

        d
    }
}

impl crate::Solution for Aoc2018_02 {
    fn name(&self) -> (usize, usize) {
        (2018, 2)
    }

    fn parse(&mut self) {
        self.d = slice("input/2018/02.txt", "\r\n");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut v: HashMap<char, i32> = HashMap::new();
        let mut d: i32 = 0;
        let mut t: i32 = 0;

        for i in 0..self.d.len() {
            let mut cd = false;
            let mut ct = false;

            for c in self.d[i].chars() {
                if v.contains_key(&c) {
                    let n = v.remove(&c).unwrap();
                    v.insert(c, n + 1);
                } else {
                    v.insert(c, 1);
                }
            }

            for k in v.values() {
                if *k == 2 && !cd {
                    d += 1;
                    cd = true;
                } else if *k == 3 && !ct {
                    t += 1;
                    ct = true;
                }
            }

            v.clear();
        }

        crate::output(d * t)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: String = String::from("invalid input");
        let mut m: usize = 100;

        for i in 0..self.d.len() - 1 {
            for j in i+1..self.d.len() {
                if Aoc2018_02::find_diff(&self.d[i], &self.d[j]) <= m {
                    s.clear();

                    for k in 0..self.d[i].len() {
                        if &self.d[i][k..=k] == &self.d[j][k..=k] {
                            s.push_str(&self.d[i][k..=k]);
                        }
                    }
                    m = self.d[i].len() - s.len();
                }
            }
        }

        return crate::output(s);
    }
}