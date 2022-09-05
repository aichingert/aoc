use aoc::read_to_slice;

use crate::aoc2015::P;

pub struct Aoc2020_04 {
    d: Vec<Passport>
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: i32,
}

impl Aoc2020_04 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl Passport {
    fn clear() -> Self {
        Self { 
            byr: -1,
            iyr: -1, 
            eyr: -1, 
            hgt: "".to_string(), 
            hcl: "".to_string(), 
            ecl: "".to_string(), 
            pid: "".to_string(),
            cid: -1,
        }
    }
}

impl crate::Solution for Aoc2020_04 {
    fn name(&self) -> (usize, usize) {
        (2020, 4)
    }

    fn parse(&mut self) {
        let v = read_to_slice("input/2020/04.txt", " ");
        let mut p: Passport = Passport::clear();

        for i in 0..v.len() {
            if &v[i][0] == "" {
                self.d.push(p.clone());
                p = Passport::clear();
                continue;
            }

            for s in &v[i] {
                let m: Vec<&str> = s.split(':').collect();

                match m[0] {
                    "byr" => p.byr = m[1].parse().unwrap(),
                    "iyr" => p.iyr = m[1].parse().unwrap(),
                    "eyr" => p.eyr = m[1].parse().unwrap(),
                    "hgt" => p.hgt = m[1].to_string(),
                    "hcl" => p.hcl = m[1].to_string(),
                    "ecl" => p.ecl = m[1].to_string(),
                    "pid" => p.pid = m[1].to_string(),
                    "cid" => p.cid = m[1].parse().unwrap(),
                    _ => panic!("invalid input")
                }
            }
        }
        self.d.push(p.clone());
    }

    fn part1(&mut self) -> Vec<String> {
        let mut o: usize = 0;
        let mut v: bool = false;
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            if self.d[i - o].byr == -1 || self.d[i - o].iyr == -1 
            || self.d[i - o].eyr == -1 || self.d[i - o].hgt == "".to_string() 
            || self.d[i - o].hcl == "".to_string() || self.d[i - o].ecl == "".to_string() 
            || self.d[i - o].pid == "".to_string() {
                v = false;
            } else {
                v = true;
            }
    
            if v {
                s += 1;
            } else {
                self.d.remove(i - o);
                o += 1;
            }
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        todo!()
    }
}