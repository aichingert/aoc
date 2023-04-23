// Advent of Code 2017, day 21
// (c) aichingert

use std::collections::{HashMap,HashSet};

trait A {
    fn flip(&self) -> Self;
    fn rotate(&self) -> Self;
    fn to_string(&self) -> String;
}

impl A for Vec<Vec<char>> {
    fn flip(&self) -> Vec<Vec<char>> {
        let mut vec = self.clone();

        for i in 0..self.len() {
            vec[i][0] = self[i][self.len()-1];
            vec[i][self.len()-1] = self[i][0];
        }

        vec
    }
    
    fn rotate(&self) -> Vec<Vec<char>> {
        let mut vec = self.clone();

        if self.len() == 2 {
            vec[0][0] = self[1][0];
            vec[0][1] = self[0][0];
            vec[1][0] = self[1][1];
            vec[1][1] = self[0][1];
        } else {
            vec[0][0] = self[2][0];
            vec[1][0] = self[2][1];
            vec[2][0] = self[2][2];
            vec[0][1] = self[1][0];
            vec[2][1] = self[1][2];
            vec[0][2] = self[0][0];
            vec[1][2] = self[0][1];
            vec[2][2] = self[0][2];
        }

        vec
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        for i in 0..self.len() {
            for j in 0..self.len() {
                s.push(self[i][j]);
            }
            s.push('/');
        }
        s.pop();

        s
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

    fn from_str(s: &str) -> Self {
        let l = s.split('/')
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut points = HashSet::new();
        let mut size = 0;

        for i in 0..l.len() {
            for j in 0..l.len() {
                if l[i][j] == '#' {
                    points.insert((i as i32,j as i32));
                }
            }
            
            size += 1;
        }

        Self { points, size }
    }

    fn sub_grids(&mut self) -> Vec<Self> {
        let mut sub_grids = Vec::<Self>::new();
        let lines = self.to_vec().to_string();
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

    fn merge(sub_grids: &Vec<Self>) -> Self {
        let mut grid: Self = Self::new(HashSet::new(), sub_grids[0].size * (sub_grids.len() as f64).sqrt() as i32);
        let mut row = 0;
        let mut col = 0;

        for i in 0..sub_grids.len() {
            for r in 0..sub_grids[i].size {
                for c in 0..sub_grids[i].size {
                    if sub_grids[i].points.contains(&(r,c)) {
                        grid.points.insert((row+r,col+c));
                    }
                }
            }
            col += sub_grids[i].size;
            if col >= (sub_grids.len() as f64).sqrt() as i32 * sub_grids[i].size {
                row += sub_grids[i].size;
                col = 0;
            }
        }

        grid
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
}

fn parse() -> HashMap<String,String> {
    let mut rules: HashMap<String,String> = HashMap::new();

    for line in std::fs::read_to_string("../input/21").unwrap().lines() {
        let (from,to) = line.split_once(" => ").unwrap();
        rules.insert(from.to_string(), to.to_string());
    }

    rules
}

fn solve(rules: &HashMap<String,String>, start: &Grid, times: u32) -> usize {
    let mut v = start.to_vec();
    let mut pattern = String::new();

    'outer: for _ in 0..2 {
        for _ in 0..4 {
            let s = v.to_string();
            if rules.contains_key(&s) {
                pattern = rules[&s].clone();
                break 'outer;
            }
            v = v.rotate();
        }
        v = v.flip();
    }

    let mut n = Grid::from_str(&pattern).sub_grids();
    for _ in 0..times-1 {
        for i in 0..n.len() {
            v = n[i].to_vec();
            'outer: for _ in 0..2 {
                for _ in 0..4 {
                    let s = v.to_string();
                    if rules.contains_key(&s) {
                        pattern = rules[&s].clone();
                        break 'outer;
                    }
                    v = v.rotate();
                }
                v = v.flip();
            }
            
            n[i] = Grid::from_str(&pattern);
        }

        n = Grid::merge(&n).sub_grids();
    }
        
    Grid::merge(&n).points.len()
}

fn main() {
    let rules = parse();
    let mut grid = Grid::new(HashSet::from([(0,1),(1,2),(2,0),(2,1),(2,2)]), 3);

    println!("Part 1: {}", solve(&rules, &mut grid, 5));
    println!("Part 2: {}", solve(&rules, &mut grid, 18));
}
