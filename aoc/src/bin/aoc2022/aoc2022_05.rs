pub struct Aoc2022_05 {
    r: Vec<(usize, usize, usize)>,
    s: Vec<Vec<String>>
}
        
impl Aoc2022_05 {
    pub fn new() -> Self {
        Self { r: vec![], s: Vec::new() }
    }
}
        
impl crate::Solution for Aoc2022_05 {
    fn name(&self) -> (usize, usize) {
        (2022, 05)
    }
        
    fn parse(&mut self) {
        let input = std::fs::read_to_string("input/2022/05.txt").expect("unable to open file!");
        let mut p: bool = false;

        for line in input.lines() {
            if !p {
                if !line.contains('[') {
                    p = true;
                    continue;
                }
                let v: Vec<&str> = line.split(' ').collect();
                let (mut i, mut j): (usize, usize) = (0,0);

                while i < v.len() {
                    if v[i] == "" {
                        // "P" "" "" "" "C"
                        i+=4;
                        j+=1;
                        if self.s.len() <= j {
                            self.s.push(vec![]);
                        }
                    } else {
                        if self.s.len() > j {
                            self.s[j].push(v[i][1..=1].to_string());
                        } else {
                            self.s.push(vec![v[i][1..=1].to_string()]);
                        }
                        i+=1;
                        j+=1;
                    }
                }
            } else {
                let splt: Vec<usize> = line.split(' ').filter(|s| s.parse::<usize>().is_ok()).map(|s| s.parse::<usize>().unwrap()).collect();
                if splt.len() < 1 {
                    continue;
                }
                self.r.push((splt[0], splt[1], splt[2]));
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut sol: String = String::new();

        for i in 0..self.r.len() {
            for _amt in 0..self.r[i].0 {
                let v = self.s[self.r[i].1-1].remove(0);
                self.s[self.r[i].2-1].insert(0, v);
            }
        }

        for i in 0..self.s.len() {
            if self.s[i].len() > 0 {
                sol.push_str(self.s[i][0].as_str());
            }
        }

        crate::output(sol)
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.r.clear();
        self.s.clear();
        self.parse();
        let mut sol: String = String::new();

        for i in 0..self.r.len() {
            let mut strg: Vec<String> = Vec::new();
            for _amt in 0..self.r[i].0 {
                let v = self.s[self.r[i].1-1].remove(0);
                strg.push(v);
            }
            for j in 0..strg.len() {
                self.s[self.r[i].2-1].insert(0, strg[strg.len() - j - 1].clone());
            }
        }

        for i in 0..self.s.len() {
            if self.s[i].len() > 0 {
                sol.push_str(self.s[i][0].as_str());
            }
        }

        crate::output(sol)
    }
}
