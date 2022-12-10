use std::collections::HashMap;

pub struct Aoc2019_06 {
    map: HashMap<String, Vec<String>>    
}
        
impl Aoc2019_06 {
    pub fn new() -> Self {
        Self {
            map: HashMap::new()
        }
    }
    
    fn orbits(&self, current: &String, depth: i32) -> i32 {
        let mut sum: i32 = 0;
        
        if self.map.contains_key(current) {
            let vec: Vec<String> = self.map[current].clone();
            sum += depth;
            
            for star in vec.iter() {
                sum += self.orbits(star, depth+1);
            }
        } else {
            sum += depth;
        }

        sum
    }

    fn count<'a>(&self, current: &'a str, searching: &String) -> i32 {
        if !self.map.contains_key(current) {
            return 0;
        }
        let mut amount: i32 = 0;
        let paths: &Vec<String> = &self.map[current];

        for i in 0..paths.len() {
            if &paths[i] == searching || amount > 0 {
                return amount + 1;
            }

            amount += self.count(&paths[i], searching);
        }

        if amount > 0 {
            return amount + 1;
        }

        amount
    }
}

impl crate::Solution for Aoc2019_06 {
    fn name(&self) -> (usize, usize) {
        (2019, 06)
    }

    fn parse(&mut self) {
        let inp: Vec<Vec<String>> = aoc::read_to_slice("input/2019/06.txt", ")");

        for i in 0..inp.len() {
            if let Some(mut stars) = self.map.remove(&inp[i][0]) {
                stars.push(inp[i][1].clone());
                self.map.insert(inp[i][0].clone(), stars);
            } else {
                self.map.insert(inp[i][0].clone(), vec![inp[i][1].clone()]);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.orbits(&"COM".to_string(), 0))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut found: bool = false;
        let mut searching: &str = "YOU";

        while !found {
            for k in self.map.keys() {
                if self.map[k].contains(&searching.to_string()) {
                    searching = k;
                    break;
                }
            }

            if self.count(searching, &"SAN".to_string()) > 0 {
                found = true;
            }
        }

        crate::output(
            self.count(searching, &"YOU".to_string()) + 
            self.count(searching, &"SAN".to_string()) - 2)
    }
}