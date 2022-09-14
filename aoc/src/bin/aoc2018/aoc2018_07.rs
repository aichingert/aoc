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
        let mut nodes = self.nodes.clone();

        while nodes.len() > 0 {
            let mut free_nodes: Vec<&String> = Vec::new();
            let current_nodes = nodes.clone();

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
                nodes.remove(free_nodes.remove(0));
            }
        }

        crate::output(ans)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut nodes: HashMap<String, (Vec<String>, i32)> = HashMap::new();
        let mut workes: [i32; 5] = [0; 5];
        let mut seconds: i32 = 0;

        for key in self.nodes.keys() {
            nodes.insert(key.clone(), (self.nodes[key].clone(), (key.chars().next().expect("string is empty") as u8 - 'A' as u8 + 1) as i32));
        }

        while nodes.len() > 0 {
            let mut free_nodes: Vec<(&String, i32)> = Vec::new();
            let current_nodes = nodes.clone();

            'outer: for search_key in current_nodes.iter() {
                for key in current_nodes.iter() {
                    if self.nodes[key.0].contains(search_key.0) {
                        continue 'outer;
                    }
                }

                if !free_nodes.contains(&(search_key.0, search_key.1.1)) {
                    free_nodes.push((search_key.0, search_key.1.1));
                }
            }

            for i in 0..workes.len() {
                if free_nodes.len() > i {
                    if workes[i] == 0 {
                        let values = nodes.remove(free_nodes[i].0).unwrap();
                        if values.1 - 1 != 0 {
                            nodes.insert(free_nodes[i].0.clone(), (values.0, values.1 - 1));
                        }
                        workes[i] = values.1 - 1;
                    } else {
                        if let Some(values) = nodes.remove(free_nodes[i].0) {
                            if values.1 - 1 != 0 {
                                nodes.insert(free_nodes[i].0.clone(), (values.0, values.1 - 1));
                            }
                            workes[i] = values.1 - 1;
                        }
                    }
                } else {
                    break;
                }
            }

            println!("{seconds} {:?} {:?}", free_nodes, workes);
            seconds += 1;
        }
        
        crate::output(seconds)
    }
}