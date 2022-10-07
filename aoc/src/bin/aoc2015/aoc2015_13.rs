use aoc::{slice, Permutations};
use std::collections::{HashMap, HashSet};

pub struct Aoc2015_13 {
    happiness: HashMap<(String, String), i32>,
    people: HashSet<String>,
}
        
impl Aoc2015_13 {
    pub fn new() -> Self {
        Self { 
            happiness: HashMap::new(),
            people: HashSet::new()
        }
    }
}
        
impl crate::Solution for Aoc2015_13 {
    fn name(&self) -> (usize, usize) {
        (2015, 13)
    }
        
    fn parse(&mut self) {
        for l in slice("input/2015/13.txt", "\n") {
            let line = l.split(' ').collect::<Vec<&str>>();
            let from: &str = line[0];
            let to: &str = &line[10][0..line[10].len() - 1];
            let amount: i32;

            if line[2] == "gain" {
                amount = line[3].parse().expect("invalid input");
            } else {
                amount = line[3].parse::<i32>().expect("invalid input") * -1;
            }

            self.happiness.insert((from.to_string(), to.to_string()), amount);          
            self.people.insert(from.to_string());
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut list = self.people
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut perms = Permutations::new();
        perms.generate(&mut list, self.people.len());

        let mut most = -i32::MAX;

        for p in perms.list {
            let mut sum: i32 = 0;
            for pair in p.windows(2) {
                sum += self.happiness[&(pair[0].clone(), pair[1].clone())];
                sum += self.happiness[&(pair[1].clone(), pair[0].clone())]
            }
            sum += self.happiness[&(p[0].clone(), p[p.len() - 1].clone())];
            sum += self.happiness[&(p[p.len() - 1].clone(), p[0].clone())];

            most = most.max(sum);
        }

        crate::output(most)
    }
        
    fn part2(&mut self) -> Vec<String> {
        for p in self.people.iter() {
            self.happiness.insert(("you".to_string(), p.clone()), 0);
            self.happiness.insert((p.clone(), "you".to_string()), 0);
        }
        self.people.insert("you".to_string());
        let mut list = self.people
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut perms = Permutations::new();
        perms.generate(&mut list, self.people.len());

        let mut most = -i32::MAX;

        for p in perms.list {
            let mut sum: i32 = 0;
            for pair in p.windows(2) {
                sum += self.happiness[&(pair[0].clone(), pair[1].clone())];
                sum += self.happiness[&(pair[1].clone(), pair[0].clone())]
            }
            sum += self.happiness[&(p[0].clone(), p[p.len() - 1].clone())];
            sum += self.happiness[&(p[p.len() - 1].clone(), p[0].clone())];

            most = most.max(sum);
        }

        crate::output(most)
    }
}
