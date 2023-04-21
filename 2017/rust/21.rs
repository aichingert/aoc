// Advent of Code 2017, day 21
// (c) aichingert

use std::collections::{HashMap,HashSet};

#[derive(Clone)]
struct Grid {
    points: HashSet<(i32,i32)>,
    size: i32,
}

impl Grid {
    fn new(points: HashSet<(i32,i32)>, size: i32) -> Self {
        Self { points, size }
    }

    fn sub_grids(&mut self) {
        let lines = self.to_string();
        let lines = lines.split('/').collect::<Vec<&str>>();
        println!("{:?}", lines);

        let len = if self.size & 1 == 0 {
            2
        } else {
            3
        };

        let mut sub_grids = vec![Grid::new(HashSet::new(),len);lines.len() / (len as usize)];
        println!("{:?}", sub_grids.len());
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
    println!("{:?}", grid.to_vec());
    grid.sub_grids();
}
