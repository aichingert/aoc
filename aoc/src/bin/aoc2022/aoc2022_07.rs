use std::collections::HashMap;

pub struct Aoc2022_07 {
    d: Vec<Vec<String>>
}
        
impl Aoc2022_07 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_07 {
    fn name(&self) -> (usize, usize) {
        (2022, 07)
    }
    
    fn parse(&mut self) {
        self.d = aoc::read_to_slice("input/2022/07.txt", " ");
        for i in 0..self.d.len() {
            if self.d[i][0].as_str() == "$" {
                self.d[i].remove(0);
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut tree: HashMap<String, u64> = HashMap::new();
        let mut dir: HashMap<String, Vec<String>> = HashMap::new();
        let mut ls: bool = false;
        let mut curr: String = String::new();

        for i in 0..self.d.len() {
            if ls {
                if self.d[i][0].parse::<u64>().is_ok() {
                    if tree.contains_key(curr.as_str()) {
                        let v = tree.remove(curr.as_str()).unwrap();
                        tree.insert(curr.clone(), v+self.d[i][0].parse::<u64>().unwrap());
                    } else {
                        tree.insert(curr.clone(), self.d[i][0].parse::<u64>().unwrap());
                    }
                    continue;
                } else if self.d[i][0].as_str() == "dir" {
                    if dir.contains_key(curr.as_str()) {
                        let mut c = dir.remove(curr.as_str()).unwrap();
                        c.push(format!("{}-{}", curr,self.d[i][1]));
                        dir.insert(curr.clone(), c);
                    } else {
                        dir.insert(curr.clone(), vec![format!("{}-{}", curr,self.d[i][1])]);
                    }

                    if !tree.contains_key(self.d[i][1].as_str()) {
                        tree.insert(format!("{}-{}",curr, self.d[i][1]), 0); // found it here we insert without the -
                        // quick fix just delete lmao
                    }
                    continue;
                }
                ls = false;
            }

            match self.d[i][0].as_str() {
                "cd" => {
                    match self.d[i][1].as_str() {
                        ".." => {
                            let mut path = curr.split('-').collect::<Vec<&str>>();
                            path.remove(path.len() -1);
                            curr = path.join("-");
                        },
                        _ => {
                            curr.push_str(&format!("-{}", self.d[i][1])); // idk maybe actually not that bad
                            //curr = concat!(curr, "-", self.d[i][1]);
                        }
                    }
                },
                "ls" => {
                    ls = true;
                },
                _ => {
                    println!("{:?}", self.d[i][0]);
                }
            }
        }

        println!("{:?}", dir);
        println!("\n{:?}", tree);

        let mut ked: Vec<&String> = dir.keys().collect();
        ked.sort_by(|a,b| b.len().cmp(&a.len()));

        for k in ked {
            let ke = dir.clone().remove(k).unwrap();
            for i in 0..ke.len() {
                let s = tree.remove(k).unwrap();
                tree.insert(k.clone(), s+tree[ke[i].as_str()]);
            }
        }

        let mut s: u64 = 0;

        for k in tree.keys() {
            if tree[k] <= 100000 {
                s += tree[k];
            }
        }
        crate::output(s)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut tree: HashMap<String, u64> = HashMap::new();
        let mut dir: HashMap<String, Vec<String>> = HashMap::new();
        let mut ls: bool = false;
        let mut curr: String = String::new();

        for i in 0..self.d.len() {
            if ls {
                if self.d[i][0].parse::<u64>().is_ok() {
                    if tree.contains_key(curr.as_str()) {
                        let v = tree.remove(curr.as_str()).unwrap();
                        tree.insert(curr.clone(), v+self.d[i][0].parse::<u64>().unwrap());
                    } else {
                        tree.insert(curr.clone(), self.d[i][0].parse::<u64>().unwrap());
                    }
                    continue;
                } else if self.d[i][0].as_str() == "dir" {
                    if dir.contains_key(curr.as_str()) {
                        let mut c = dir.remove(curr.as_str()).unwrap();
                        c.push(format!("{}-{}", curr,self.d[i][1]));
                        dir.insert(curr.clone(), c);
                    } else {
                        dir.insert(curr.clone(), vec![format!("{}-{}", curr,self.d[i][1])]);
                    }

                    if !tree.contains_key(self.d[i][1].as_str()) {
                        tree.insert(format!("{}-{}",curr, self.d[i][1]), 0); // found it here we insert without the -
                        // quick fix just delete lmao
                    }
                    continue;
                }
                ls = false;
            }

            match self.d[i][0].as_str() {
                "cd" => {
                    match self.d[i][1].as_str() {
                        ".." => {
                            let mut path = curr.split('-').collect::<Vec<&str>>();
                            path.remove(path.len() -1);
                            curr = path.join("-");
                        },
                        _ => {
                            curr.push_str(&format!("-{}", self.d[i][1])); // idk maybe actually not that bad
                            //curr = concat!(curr, "-", self.d[i][1]);
                        }
                    }
                },
                "ls" => {
                    ls = true;
                },
                _ => {
                    println!("{:?}", self.d[i][0]);
                }
            }
        }

        println!("{:?}", dir);
        println!("\n{:?}", tree);

        let mut ked: Vec<&String> = dir.keys().collect();
        ked.sort_by(|a,b| b.len().cmp(&a.len()));

        for k in ked {
            let ke = dir.clone().remove(k).unwrap();
            for i in 0..ke.len() {
                let s = tree.remove(k).unwrap();
                tree.insert(k.clone(), s+tree[ke[i].as_str()]);
            }
        }

        let mut m: u64 = u64::MAX;

        let u: u64 = 30000000 - (70000000-tree["-/"]);

        for k in tree.keys() {
            if tree[k] > u && m > tree[k] {
                m = tree[k];
            }
        }

        crate::output(m)
    }
}