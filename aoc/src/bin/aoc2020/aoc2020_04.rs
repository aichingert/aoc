use aoc::read_to_slice;

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

    fn check_height(&self) -> bool {
        let mut s: String = String::new();
        let mut f: String = String::new();
    
        for c in self.hgt.chars() {
            if c.is_numeric() {
                s.push(c)
            } else {
                f.push(c);
            }
        }
    
        let n: i32 = s.parse().unwrap();
    
        match &f as &str {
            "in" => {
                if n >= 59 && n <= 76 {
                    return true;
                }
            },
            "cm" => {
                if n >= 150 && n <= 193 {
                    return true;
                }
            },
            _ => {}
        }
    
        false
    }

    fn check_hair_color(&self) -> bool {
        let mut h: String = self.hcl.clone();
        let mut v: bool = false;
    
        if h.remove(0) != '#' {
            return false;
        }
    
        if h.len() != 6 {
            return false;
        }
    
        for c in h.chars() {
            for s in 'a'..='f' {
                if c == s {
                    v = true;
                }
            }
    
            if c.is_numeric() {
                v = true;
            }
        }
    
        v
    }
    
    fn check_eye_color(&self) -> bool {
        match &self.ecl as &str {
            "amb" => return true,
            "blu" => return true,
            "brn" => return true,
            "gry" => return true,
            "grn" => return true,
            "hzl" => return true,
            "oth" => return true,
            _ => return false
        }
    }

    fn check_pid(&self) -> bool {
        if self.pid.len() != 9 {
            return false;
        }
    
        for c in self.pid.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
    
        true
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
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            if self.d[i - o].byr == -1 || self.d[i - o].iyr == -1 
            || self.d[i - o].eyr == -1 || self.d[i - o].hgt == "".to_string() 
            || self.d[i - o].hcl == "".to_string() || self.d[i - o].ecl == "".to_string() 
            || self.d[i - o].pid == "".to_string() {
                self.d.remove(i - o);
                o += 1;
            } else {
                s += 1;
            }
        }

        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            if self.d[i].byr.to_string().len() == 4 && self.d[i].byr >= 1920 && self.d[i].byr <= 2002
            && self.d[i].iyr.to_string().len() == 4 && self.d[i].iyr >= 2010 && self.d[i].iyr <= 2020
            && self.d[i].eyr.to_string().len() == 4 && self.d[i].eyr >= 2020 && self.d[i].eyr <= 2030
            && self.d[i].check_height() && self.d[i].check_hair_color()
            && self.d[i].check_eye_color() && self.d[i].check_pid() {
                s += 1;
            } 
        }

        crate::output(s)
    }
}
