extern crate pathfinding;

use pathfinding::prelude::dijkstra;

pub struct Aoc2022_12 {
    m: Vec<Vec<char>>
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Pos(i32, i32);
        
impl Aoc2022_12 {
    pub fn new() -> Self {
        Self { 
            m: Vec::new()
        }
    }
}

impl Pos {
    fn successors(&self, m: &Vec<Vec<char>>) -> Vec<(Pos, usize)> {
        [(1,0),(0,1),(-1,0),(0,-1)].iter().filter(|vec| {
            self.0 + vec.0 > -1 && ((self.0 + vec.0) as usize) < m.len() && self.1 + vec.1 > -1 && ((self.1 + vec.1) as usize) < m[0].len() && 
            (m[(self.0 + vec.0) as usize][(self.1 + vec.1) as usize]) as i32 - (m[(self.0) as usize][(self.1) as usize]) as i32 >= -1
        }).map(|vec| (Pos(self.0 + vec.0, self.1 + vec.1), 1)).collect()
    }
}
        
impl crate::Solution for Aoc2022_12 {
    fn name(&self) -> (usize, usize) {
        (2022, 12)
    }
        
    fn parse(&mut self) {
        self.m = std::fs::read_to_string("input/2022/12.txt").expect("unable to open file!").lines().map(|s| s.chars().collect()).collect();
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut start: Pos = Pos(0,0);
        let mut dest: Pos = Pos(0,0);

        for (i, r) in self.m.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                match *c {
                    'S' => start = Pos(i as i32, j as i32),
                    'E' => dest = Pos(i as i32, j as i32),
                    _ => ()
                }
            }
        }

        self.m[start.0 as usize][start.1 as usize] = 'a';
        self.m[dest.0 as usize][dest.1 as usize] = 'z';

        if let Some((_, result)) = dijkstra(&dest, |loc| loc.successors(&self.m), |loc| *loc == start) {
            self.m[dest.0 as usize][dest.1 as usize] = 'E';
            return crate::output(result);
        }
        
        panic!("unable to find path")
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut starts: Vec<Pos> = Vec::new();
        let mut dest: Pos = Pos(0,0);
        let mut min: usize = 100000;

        for (i, r) in self.m.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                match *c {
                    'a' => starts.push(Pos(i as i32, j as i32)),
                    'E' => dest = Pos(i as i32, j as i32),
                    _ => ()
                }
            }
        }

        self.m[dest.0 as usize][dest.1 as usize] = 'z';

        for start in starts {
            if let Some((_, result)) = dijkstra(&dest, |loc| loc.successors(&self.m), |loc| *loc == start) {
                min = std::cmp::min(min, result);
            }
        }
        
        crate::output(min)
    }
}
