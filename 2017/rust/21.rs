// Advent of Code 2017, day 21
// (c) aichingert

use std::collections::{HashMap,HashSet};

trait A {
    fn flip(&self) -> Self;
    fn rotate(&self) -> Self;
}

impl A for Vec<Vec<char>> {
    fn flip(&self) -> Vec<Vec<char>> {
        let mut vec = vec![vec!['.';self.len()];self.len()];

        Vec::new()
    }
    fn rotate(&self) -> Vec<Vec<char>> {
        Vec::new()
    }
}

#[derive(Clone)]
struct Grid {
    points: HashSet<(i32,i32)>,
    size: i32,
}

impl Grid {
    fn new(points: HashSet<(i32,i32)>, size: i32) -> Self {
        Self { points, size }
    }

    fn sub_grids(&mut self) -> Vec<Self> {
        let mut sub_grids = Vec::<Self>::new();
        let lines = self.to_string();
        let lines = lines.split('/')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let len = if self.size & 1 == 0 {
            2
        } else {
            3
        };

        for row in (0..self.size).step_by(len) {
            for col in (0..self.size).step_by(len) {
                let mut points: HashSet<(i32,i32)> = HashSet::new();

                for i in 0..len as i32 {
                    for j in 0..len as i32 {
                        if lines[(row + i) as usize][(col + j) as usize] == '#' {
                            points.insert((i,j));
                        }
                    }
                }

                sub_grids.push(Self::new(points, len as i32));
            }
        }

        sub_grids
    }

    fn to_vec(&self) -> Vec<Vec<char>> {
        let mut vec = Vec::new();

        for i in 0..self.size {
            let mut col = Vec::new();
            for j in 0..self.size {
                col.push(if self.points.contains(&(i,j)) {
                    '#'
                } else {
                    '.'
                });
            }
            vec.push(col);
        }

        vec
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for i in 0..self.size {
            for j in 0..self.size {
                s.push(if self.points.contains(&(i,j)) {
                    '#'
                } else {
                    '.'
                });
            }
            s.push('/');
        }
        s.pop();

        s
    }
}

fn parse() -> HashMap<String,String> {
    let mut rules: HashMap<String,String> = HashMap::new();

    for line in std::fs::read_to_string("../input/21").unwrap().lines() {
        let (from,to) = line.split_once(" => ").unwrap();
        rules.insert(from.to_string(), to.to_string());
    }

    rules
}

fn main() {
    let rules = parse();
    let mut grid = Grid::new(HashSet::from([(0,0),(0,3),(3,0),(3,3)]), 4);

    let s = grid.sub_grids();
    for i in 0..s.len() {
        println!("{:?}", s[i].points);
    }
}
