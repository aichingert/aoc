use aoc::slice;
use std::collections::HashMap;

pub struct Aoc2018_07 {
    nodes: HashMap<String, Vec<String>>
}
        
impl Aoc2018_07 {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }
}
        
impl crate::Solution for Aoc2018_07 {
    fn name(&self) -> (usize, usize) {
        (2018, 07)
    }
        
    fn parse(&mut self) {
        for l in slice("input/2018/07.txt", "\r\n") {
            let line = l.split(' ').collect::<Vec<&str>>();
            if let Some(mut values) = self.nodes.remove(line[1]) {
                values.push(line[7].to_string());
                self.nodes.insert(line[1].to_string(), values);

                if !self.nodes.contains_key(line[7]) {
                    self.nodes.insert(line[7].to_string(), vec![]);
                }
            } else {
                self.nodes.insert(line[1].to_string(), vec![line[7].to_string()]);
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut ans: String = String::new();

        while self.nodes.len() > 0 {
            let mut free_nodes: Vec<&String> = Vec::new();
            let current_nodes = self.nodes.clone();

            'outer: for search_key in current_nodes.iter() {
                for key in current_nodes.iter() {
                    if self.nodes[key.0].contains(search_key.0) {
                        continue 'outer;
                    }
                }

                if !free_nodes.contains(&search_key.0) {
                    free_nodes.push(search_key.0);
                }
            }

            if free_nodes.len() > 0 {
                free_nodes.sort();
                ans.push_str(&free_nodes[0].clone());
                self.nodes.remove(free_nodes.remove(0));
            }
        }

        crate::output(ans)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}