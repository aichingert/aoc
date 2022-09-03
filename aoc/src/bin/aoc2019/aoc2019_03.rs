use aoc::read_to_slice;
use std::collections::HashMap;

pub struct Aoc2019_03 {
    d: Vec<Vec<String>>
}

impl Aoc2019_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn follow_path(&mut self, m: &mut HashMap<(i32, i32), i32>) {
        let mut p = (0,0);
        let mut w: i32 = 0;

        for j in 0..self.d[0].len() {
            let v = self.d[0][j][1..self.d[0][j].len()].parse::<i32>().expect("invalid input");
            let mut f: i32 = 0;
            let mut s: i32 = 0;

            match &self.d[0][j][0..1] {
                "R" => f = 1,
                "L" => f = -1,
                "U" => s = 1,
                "D" => s = -1,
                _ => panic!("invalid input")
            }

            for _ in 0..v {
                p.0 += 1 * f;
                p.1 += 1 * s;
                w+=1;
                m.insert((p.0, p.1), w);
            }
        }
    }

    fn overlapping(&mut self, m: &mut HashMap<(i32, i32), i32>) -> i32 {
        let mut p = (0,0);
        let mut d: i32 = 100000;

        for j in 0..self.d[1].len() {
            let v = self.d[1][j][1..self.d[1][j].len()].parse::<i32>().expect("invalid input");
            let mut f: i32 = 0;
            let mut s: i32 = 0;

            match &self.d[1][j][0..1] {
                "R" => f = 1,
                "L" => f = -1,
                "U" => s = 1,
                "D" => s = -1,
                _ => panic!("invalid input")
            }

            for _ in 0..v {
                p.0 += 1 * f;
                p.1 += 1 * s;
                if m.contains_key(&(p.0, p.1)) {
                    let r: i32 = p.0.abs() + p.1.abs();
                    if d > r {
                        d = r;
                    }
                }
            }
        }
        
        d
    }

    fn steps(&mut self, m: &mut HashMap<(i32, i32), i32>) -> i32 {
        let mut p = (0,0);
        let mut d: i32 = 100000;
        let mut w: i32 = 0;

        for j in 0..self.d[1].len() {
            let v = self.d[1][j][1..self.d[1][j].len()].parse::<i32>().expect("invalid input");
            let mut f: i32 = 0;
            let mut s: i32 = 0;

            match &self.d[1][j][0..1] {
                "R" => f = 1,
                "L" => f = -1,
                "U" => s = 1,
                "D" => s = -1,
                _ => panic!("invalid input")
            }

            for _ in 0..v {
                p.0 += 1 * f;
                p.1 += 1 * s;
                w+=1;
                if m.contains_key(&(p.0, p.1)) {
                    let r: i32 = m[&(p.0, p.1)] + w;
                    if d > r {
                        d = r;
                    }
                }
            }
        }
        
        d
    }
}

impl crate::Solution for Aoc2019_03 {
    fn name(&self) -> (usize, usize) {
        (2019, 3)
    }

    fn parse(&mut self) {
        self.d = read_to_slice("input/2019/03.txt", ",");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut m: HashMap<(i32, i32), i32> = HashMap::new();
        self.follow_path(&mut m);
        crate::output(self.overlapping(&mut m))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut m: HashMap<(i32, i32), i32> = HashMap::new();
        self.follow_path(&mut m);
        crate::output(self.steps(&mut m))
    }
}