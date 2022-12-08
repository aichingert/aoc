use std::collections::HashMap;

pub struct Aoc2022_07 {
    m: HashMap<String, u64>
}
        
impl Aoc2022_07 {
    pub fn new() -> Self {
        Self { 
            m: HashMap::new(),
        }
    }

    fn cmd<'a>(&mut self, pwd: &mut Vec<&'a str>, line: &Vec<&'a str>) {
        match line[1] {
            "cd" => {
                match line[2] {
                    ".." => {
                        let child_folder: String = pwd.iter().map(|slice| slice.to_string()).collect::<String>();
                        pwd.pop();
                        let parent_folder: String = pwd.iter().map(|slice| slice.to_string()).collect::<String>();
                        if let Some(size) = self.m.remove(&parent_folder) {
                            self.m.insert(parent_folder, size + self.m[&child_folder]);
                        }
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
                    self.m.insert(folder_path, 0u64);
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
}
        
impl crate::Solution for Aoc2022_07 {
    fn name(&self) -> (usize, usize) {
        (2022, 07)
    }
    
    fn parse(&mut self) {
        let mut pwd: Vec<&str> = Vec::new();
        let input: String = std::fs::read_to_string("input/2022/07.txt").expect("unable to open file!");

        for l in input.lines() {
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

        while pwd.len() > 0 {
            self.cmd(&mut pwd, &vec!["$", "cd", ".."]);
        }
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
