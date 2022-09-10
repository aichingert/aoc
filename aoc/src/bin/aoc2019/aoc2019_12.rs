use aoc::read_to_slice;
use std::collections::HashMap;
pub struct Aoc2019_12 {
    d: Vec<[[i32; 3]; 2]>
}
        
impl Aoc2019_12 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2019_12 {
    fn name(&self) -> (usize, usize) {
        (2019, 12)
    }
        
    fn parse(&mut self) {
        let c = read_to_slice("input/2019/12.txt", ", ");

        for i in 0..c.len() {
            let x: i32 = c[i][0][3..c[i][0].len()].parse::<i32>().unwrap();
            let y: i32 = c[i][1][2..c[i][1].len()].parse::<i32>().unwrap();
            let z: i32 = c[i][2][2..c[i][2].len() - 1].parse::<i32>().unwrap();

            self.d.push([[x,y,z], [0,0,0]]);
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let s = 1000;
        for b in 0..=s {
            for i in 0..self.d.len() {
                // apply velocity
                self.d[i][0][0] += self.d[i][1][0];
                self.d[i][0][1] += self.d[i][1][1];
                self.d[i][0][2] += self.d[i][1][2];
            }

            if b < s {
                for i in 0..self.d.len() {
                    // calc velocity
                    for j in 0..self.d.len() {
                        if i == j {
                            continue;
                        }
    
                        if self.d[j][0][0] > self.d[i][0][0] {
                            self.d[i][1][0] += 1;
                        } else if self.d[j][0][0] < self.d[i][0][0] {
                            self.d[i][1][0] -= 1;
                        }
    
                        if self.d[j][0][1] > self.d[i][0][1] {
                            self.d[i][1][1] += 1;
                        } else if self.d[j][0][1] < self.d[i][0][1] {
                            self.d[i][1][1] -= 1;
                        }
    
                        if self.d[j][0][2] > self.d[i][0][2] {
                            self.d[i][1][2] += 1;
                        } else if self.d[j][0][2] < self.d[i][0][2] {
                            self.d[i][1][2] -= 1;
                        }
                    }
                }
            }
        }

        let mut s: i32 = 0;

        for i in 0..self.d.len() {
            s += (self.d[i][0][0].abs() + self.d[i][0][1].abs() + self.d[i][0][2].abs()) * (self.d[i][1][0].abs() + self.d[i][1][1].abs() + self.d[i][1][2].abs());
        }

        crate::output(s)
    }
        
    fn part2(&mut self) -> Vec<String> {
       
        crate::output("")
    }
}