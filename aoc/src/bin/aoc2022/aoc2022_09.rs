use std::collections::HashSet;

pub struct Aoc2022_09 {
    instr: Vec<(char, i32)>
}
        
impl Aoc2022_09 {
    pub fn new() -> Self {
        Self { instr: Vec::new() }
    }

    fn simulate(&self, knots: &mut Vec<(i32, i32)>, p1: usize, p2: usize) {
        if knots[p1].1.abs_diff(knots[p2].1) > 1 || knots[p1].0.abs_diff(knots[p2].0) > 1 {
            knots[p2].1 += (knots[p1].1 - knots[p2].1).signum();
            knots[p2].0 += (knots[p1].0 - knots[p2].0).signum();
        }
    }

    fn solve(&self, part: bool) -> usize {
        let mut positions: HashSet<(i32, i32)> = HashSet::from([(0,0)]);
        let mut knots: Vec<(i32, i32)> = if part { vec![(0,0);2] } else { vec![(0,0);10] };

        for i in 0..self.instr.len() {
            for _ in 0..self.instr[i].1 {
                match self.instr[i].0 {
                    'R' => knots[0].0 += 1,
                    'L' => knots[0].0 -= 1,
                    'U' => knots[0].1 += 1,
                    'D' => knots[0].1 -= 1,
                    _ => panic!()
                };
                for j in 1..knots.len() {
                    self.simulate(&mut knots, j-1, j);
                }
                positions.insert(knots[knots.len()-1]);
            }
        }

        positions.len()
    }
}
        
impl crate::Solution for Aoc2022_09 {
    fn name(&self) -> (usize, usize) {
        (2022, 09)
    }
        
    fn parse(&mut self) {
        for line in aoc::read_to_slice("input/2022/09.txt", " ") {
            self.instr.push((line[0].chars().next().unwrap(), line[1].parse::<i32>().unwrap()));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.solve(true))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(false))
    }
}