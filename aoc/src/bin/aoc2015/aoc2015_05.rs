use aoc::slice;

pub struct Aoc2015_05 {
    d: Vec<String>
}
        
impl Aoc2015_05 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2015_05 {
    fn name(&self) -> (usize, usize) {
        (2015, 05)
    }
        
    fn parse(&mut self) {
        self.d = slice("input/2015/05.txt", "\r\n");
        self.d.remove(self.d.len() - 1);
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let mut c: i32 = 0;
            let mut v: bool = true;
            let mut h: bool = false;
            for j in 0..self.d[i].len() {
                match &self.d[i][j..=j] {
                    "a" | "e" | "i" | "o" | "u" => c += 1, 
                    _ => {}
                }

                if j < self.d[i].len() - 1 {
                    let d: String = self.d[i][j..=j].to_owned() + &self.d[i][j+1..=j+1];

                    match &d as &str {
                        "ab" | "cd" | "pq" | "xy" => v = false,
                        _ => {}
                    }

                    if d[0..=0] == d[1..=1] {
                        h = true;
                    }
                }
            }

            if h && v && c >= 3 {
                s += 1;
            }
        }

        crate::output(s)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            let mut v: bool = false;
            let mut c: bool = false;
            for j in 0..self.d[i].len() - 1 {
                if !c {
                    let d = self.d[i][j..=j].to_owned() + &self.d[i][j+1..=j+1];

                    for k in j+2..self.d[i].len() - 1 {
                        if d == self.d[i][k..=k].to_owned() + &self.d[i][k+1..=k+1] {
                            c = true;
                            break;
                        }
                    }
                }

                if j < self.d[i].len() - 2 {
                    if self.d[i][j..=j] == self.d[i][j+2..=j+2] {
                        v = true;
                        if c {
                            break;
                        }
                    }
                }
            }

            if v && c {
                s += 1;
            }
        }

        crate::output(s)
    }
}