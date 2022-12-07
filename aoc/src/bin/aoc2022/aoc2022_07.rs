use std::collections::HashMap;

pub struct Aoc2022_07 {
    m: HashMap<String, u64>,
    d: HashMap<String, Vec<String>>
}
        
impl Aoc2022_07 {
    pub fn new() -> Self {
        Self { 
            m: HashMap::new(),
            d: HashMap::new()
        }
    }

    fn cmd<'a>(&mut self, pwd: &mut Vec<&'a str>, line: &Vec<&'a str>) {
        match line[1] {
            "cd" => {
                match line[2] {
                    ".." => {
                        pwd.pop();
                    },
                    _ => {
                        pwd.push(line[2]);
                    }
                }
            },
            "ls" => {}, // doesn't do anything
            _ => panic!("invalid input!")
        }
    }

    fn update<'a>(&mut self, pwd: &Vec<&'a str>, line: &Vec<&'a str>) {
        let path: String = pwd.iter().map(|slice| slice.to_string()).collect::<String>();

        match line[0] {
            "dir" => {
                let folder_path: String = format!("{}{}", &path, line[1]);

                if !self.m.contains_key(&folder_path) {
                    self.m.insert(folder_path.clone(), 0u64);
                }

                if let Some(mut folders) = self.d.remove(&path) {
                    folders.push(folder_path);
                    self.d.insert(path, folders);
                } else {
                    self.d.insert(path, vec![folder_path]);
                }
            },
            _ => {
                let file_size: u64 = line[0].parse::<u64>().expect("invalid input: expected file size!");

                if let Some(folder_size) = self.m.remove(&path) {
                    self.m.insert(path, folder_size + file_size);
                } else {
                    self.m.insert(path, file_size);
                }
            }
        }
    }

    fn size<'a>(&mut self) {
        let mut keys: Vec<&String> = self.d.keys().collect();
        keys.sort_by(|a,b| b.len().cmp(&a.len()));

        for key in keys {
            let folders: &Vec<String> = &self.d[key];

            for i in 0..folders.len() {
                let current_size = self.m[key];
                self.m.insert(key.to_string(), current_size + self.m[&folders[i]]);
            }
        }
    }
}
        
impl crate::Solution for Aoc2022_07 {
    fn name(&self) -> (usize, usize) {
        (2022, 07)
    }
    
    fn parse(&mut self) {
        let mut pwd: Vec<&str> = Vec::new();

        for l in std::fs::read_to_string("input/2022/07.txt").expect("unable to open file!").lines() {
            let line: Vec<&str> = l.split(' ').collect();

            match line[0] {
                "$" => {
                    self.cmd(&mut pwd, &line);
                },
                _ => {
                    self.update(&pwd, &line);
                }
            }
        }

        self.size();
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.m.iter().filter(|(_,size)| **size <= 100_000).map(|(_,size)| size).sum::<u64>())
    }
        
    fn part2(&mut self) -> Vec<String> {
        let (mut max, unused): (u64, u64) = (u64::MAX, 30_000_000 - (70_000_000 - self.m["/"]));
        self.m.iter().for_each(|(_,size)| if *size > unused && *size < max { max = *size });
        crate::output(max)
    }
}
