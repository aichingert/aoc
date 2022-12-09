use std::collections::HashSet;

pub struct Aoc2022_09 {
    d: Vec<(char, i32)>
}
        
impl Aoc2022_09 {
    pub fn new() -> Self {
        Self { d: Vec::new() }
    }

    /*
        x:4 y:5 h
        x:3 y:5 t

        x: // 
    */
    fn simulate(&self, knots: &mut Vec<(i32, i32)>, p1: usize, p2: usize) {
        if knots[p1].1.abs_diff(knots[p2].1) > 1 || knots[p1].0.abs_diff(knots[p2].0) > 1 {
            knots[p2].1 += (knots[p1].1 - knots[p2].1).signum();
            knots[p2].0 += (knots[p1].0 - knots[p2].0).signum();
        }
    }
}
        
impl crate::Solution for Aoc2022_09 {
    fn name(&self) -> (usize, usize) {
        (2022, 09)
    }
        
    fn parse(&mut self) {
        let input: String = std::fs::read_to_string("input/2022/09.txt").expect("unable to open file!");
        
        for line in input.lines() {
            let s: Vec<_> = line.split(' ').collect();

            self.d.push((s[0].chars().next().unwrap(), s[1].parse::<i32>().unwrap()));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut positions: HashSet<(i32, i32)> = HashSet::new();
        let mut knots: Vec<(i32, i32)> = vec![(0,0);2];

        for i in 0..self.d.len() {
            for _ in 0..self.d[i].1 {
                positions.insert(knots[knots.len()-1]);
                match self.d[i].0 {
                    'R' => knots[0].0 += 1,
                    'L' => knots[0].0 -= 1,
                    'U' => knots[0].1 += 1,
                    'D' => knots[0].1 -= 1,
                    _ => panic!()
                };
                self.simulate(&mut knots, 0, 1);
            }
        }

        crate::output(positions.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut positions: HashSet<(i32, i32)> = HashSet::new();
        let mut knots: Vec<(i32, i32)> = vec![(0,0);10];
        positions.insert(knots[knots.len()-1]);

        for i in 0..self.d.len() {
            for _ in 0..self.d[i].1 {
                match self.d[i].0 {
                    'R' => knots[0].0 += 1,
                    'L' => knots[0].0 -= 1,
                    'U' => knots[0].1 += 1,
                    'D' => knots[0].1 -= 1,
                    _ => panic!()
                };
                for j in 0..knots.len()-1 {
                    self.simulate(&mut knots, j, j+1);
                }
                positions.insert(knots[knots.len()-1]);
            }
        }

        crate::output(positions.len())
    }
}