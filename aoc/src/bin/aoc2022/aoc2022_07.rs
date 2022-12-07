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

                if !self.m.contains_key(line[1]) {
                    self.m.insert(line[1].to_string(), 0u64);
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


    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
