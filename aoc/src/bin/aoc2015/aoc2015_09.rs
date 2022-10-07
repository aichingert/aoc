use aoc::{slice ,Permutations};
use std::collections::{HashMap, HashSet};

pub struct Aoc2015_09 {
    dist: HashMap<(String, String), u64>,
    cities: HashSet<String>
}
        
impl Aoc2015_09 {
    pub fn new() -> Self {
        Self { 
            dist: HashMap::new(),
            cities: HashSet::new()
        }
    }
}
        
impl crate::Solution for Aoc2015_09 {
    fn name(&self) -> (usize, usize) {
        (2015, 09)
    }
        
    fn parse(&mut self) {
        for l in slice("input/2015/09.txt", "\n") {
            let line = l.split(' ').collect::<Vec<&str>>();
            let from = line[0];
            let to = line[2];
            let distance = line[4].parse().expect("invalid input");

            self.dist.insert((from.to_string(), to.to_string()), distance);
            self.dist.insert((to.to_string(), from.to_string()), distance);
            self.cities.insert(from.to_string());
            self.cities.insert(to.to_string());            
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut list = self.cities
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut perms = Permutations::new();
        perms.generate(&mut list, self.cities.len());
        let mut shortest: u64 = u64::MAX;

        for p in perms.list.iter() {
            let mut sum: u64 = 0;
            for pair in p.windows(2) {
                sum += self.dist[&(pair[0].clone(), pair[1].clone())]
            }

            shortest = shortest.min(sum);
        }

        crate::output(shortest)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut list = self.cities
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut perms = Permutations::new();
        perms.generate(&mut list, self.cities.len());
        let mut longest: u64 = 0;

        for p in perms.list.iter() {
            let mut sum: u64 = 0;
            for pair in p.windows(2) {
                sum += self.dist[&(pair[0].clone(), pair[1].clone())];
            }

            longest = longest.max(sum);
        }

        crate::output(longest)
    }
}
