use std::collections::HashMap;

pub struct Aoc2021_22 {
    cords: Vec<((i64, i64), (i64, i64), (i64, i64), bool)>
}
        
impl Aoc2021_22 {
    pub fn new() -> Self {
        Self { 
            cords: Vec::new()
        }
    }
}
        
impl crate::Solution for Aoc2021_22 {
    fn name(&self) -> (usize, usize) {
        (2021, 22)
    }
        
    fn parse(&mut self) {
        let inp: String = std::fs::read_to_string("input/2021/22.txt").expect("unable to open file");

        for line in inp.lines() {
            let p: Vec<&str> = line.trim().split(' ').collect();
            let v: Vec<&str> = p[1].split(',').collect();
            let mut f: Vec<&str> = v[0].split('=').collect();
            let mut s: Vec<i64> = f[1].split("..").map(|x| x.parse::<i64>().unwrap()).collect();
            
            let x: (i64, i64) = (s[0], s[1]);

            f = v[1].split('=').collect();
            s = f[1].split("..").map(|x| x.parse::<i64>().unwrap()).collect();
            
            let y: (i64, i64) = (s[0], s[1]);

            f = v[2].split('=').collect();
            s = f[1].split("..").map(|x| x.parse::<i64>().unwrap()).collect();
            
            let z: (i64, i64) = (s[0], s[1]);

            self.cords.push((x,y,z, p[0] == "on"));
        }

    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut m: Box::<HashMap<(i64, i64, i64), bool>> = Box::new(HashMap::new());

        for cord in &self.cords {
            for x in cord.0.0..=cord.0.1 {
                if x >= -50 && x <= 50 {
                    for y in cord.1.0..=cord.1.1 {
                        if y >= -50 && y <= 50 {
                            for z in cord.2.0..=cord.2.1 {
                                if z >= -50 && z <= 50 {
                                    m.insert((x, y, z), cord.3);
                                }
                            }
                        }
                    }
                }
            }
        }

        crate::output(m.iter().map(|(_, b)| if *b {1}else{0}).sum::<usize>())
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
