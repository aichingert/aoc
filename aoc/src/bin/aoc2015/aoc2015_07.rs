use std::collections::HashMap;

pub struct Aoc2015_07 {
    d: Vec<Vec<String>>,
    v: HashMap<String, i32>
}
        
impl Aoc2015_07 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
            v: HashMap::new()
        }
    }

    fn get_match(&self, op: String, v_o: i32, v_t: i32) -> i32 {
        match op.as_str() {
            "AND" => v_o & v_t,
            "OR" => v_o | v_t,
            "LSHIFT" => v_o << v_t,
            "RSHIFT" => v_o >> v_t,
            "NOT" => !v_o,
            _ => panic!("Invalid argument {}", op)
        }
    }
}
        
impl crate::Solution for Aoc2015_07 {
    fn name(&self) -> (usize, usize) {
        (2015, 07)
    }
        
    fn parse(&mut self) {
        self.d = aoc::read_to_slice("input/2015/07.txt", " -> ");
        let mut idx: Vec<usize> = Vec::new();
        let mut offset: usize = 0;

        for i in 0..self.d.len() {
            if self.d[i][0].parse::<i32>().is_ok() && self.d[i].len() == 2 {
                self.v.insert(self.d[i][1].clone(), self.d[i][0].parse::<i32>().unwrap());
                idx.push(i);
            }
        }

        while idx.len() > 0 {
            self.d.remove(idx[0] - offset);
            offset += 1;
            idx.remove(0);
        }

        for i in 0..self.d.len() {
            let mut f: Vec<String> = self.d[i][0].split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
            f.push(self.d[i][self.d[i].len()-1].clone());
            self.d[i] = f;
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        while self.d.len() > 0 {
            let mut rem: bool = false;
            for i in 0..self.d.len() {
                if self.d[i].len() >= 3 && self.v.contains_key(&self.d[i][0]) && self.d[i][2].parse::<i32>().is_ok() {
                    let value: i32 = self.get_match(self.d[i][1].clone(), self.v[&self.d[i][0]], self.d[i][2].parse::<i32>().unwrap());
                    self.v.insert(self.d[i][3].clone(), value);
                    rem = true;
                } else if self.d[i].len() >= 3 && self.v.contains_key(&self.d[i][0]) && self.v.contains_key(&self.d[i][2]) {
                    let value: i32 = self.get_match(self.d[i][1].clone(), self.v[&self.d[i][0]], self.v[&self.d[i][2]]);
                    self.v.insert(self.d[i][3].clone(), value);
                    rem = true;
                } else if self.d[i].len() >= 3 && self.d[i][0].parse::<i32>().is_ok() && self.v.contains_key(&self.d[i][2]) {
                    let value: i32 = self.get_match(self.d[i][1].clone(), self.d[i][0].parse::<i32>().unwrap(), self.v[&self.d[i][2]]);
                    self.v.insert(self.d[i][3].clone(), value);
                    rem = true;
                } else if self.d[i].len() >= 3 && self.d[i][0].parse::<i32>().is_ok() && self.d[i][2].parse::<i32>().is_ok() {
                    let value: i32 = self.get_match(self.d[i][1].clone(), self.d[i][0].parse::<i32>().unwrap(), self.d[i][2].parse::<i32>().unwrap());
                    self.v.insert(self.d[i][3].clone(), value);
                    rem = true;
                } else if self.d[i].len() >= 3 && self.d[i][0].as_str() == "NOT" && self.d[i][1].parse::<i32>().is_ok() {
                    let value: i32 = self.get_match(self.d[i][0].clone(), self.d[i][1].parse::<i32>().unwrap(), 0);
                    self.v.insert(self.d[i][2].clone(), value);
                    rem = true;
                } else if self.d[i].len() >= 3 && self.d[i][0].as_str() == "NOT" && self.v.contains_key(&self.d[i][1]) {
                    let value: i32 = self.get_match(self.d[i][0].clone(), self.v[&self.d[i][1]], 0);
                    self.v.insert(self.d[i][2].clone(), value);
                    rem = true;
                } else if self.d[i].len() == 2 && self.v.contains_key(&self.d[i][0]) {
                    self.v.insert(self.d[i][1].clone(), self.v[&self.d[i][0]]);
                    rem = true;
                }

                if rem {
                    self.d.remove(i);
                    break;
                }
            }
        }

        crate::output(self.v["a"])
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
