use aoc::slice;

use std::collections::HashMap;

pub struct Aoc2016_01 {
    d: Vec<String>
}

impl Aoc2016_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

fn contains(m: &mut HashMap<(i32, i32), bool>, p: &mut (i32, i32), n: i32, r: i32, c: i32) -> (i32, i32, bool) {
    for i in 0..n {
        if m.contains_key(&(p.0 + (i * r), p.1 + (i * c))) {
            return (p.0 + (i * r), p.1 + (i * c), true);
        } else {
            m.insert((p.0 + (i * r), p.1 + (i * c)), true);
        }
    }

    p.0 += n * r;
    p.1 += n * c;
    (0, 0, false)
}

impl crate::Solution for Aoc2016_01 {
    fn name(&self) -> (usize, usize) {
        (2016, 1)
    }

    fn parse(&mut self) {
        self.d = slice("input/2016/01.txt", ", ");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut p = [0, 0];
        let mut d = 0;

        for i in 0..self.d.len() {
            let n: i32 = self.d[i][1..self.d[i].len()].parse::<i32>().unwrap();

            match &self.d[i][0..1] {
                "R" => d = (d + 90) % 360,
                "L" => d = ((d - 90) + 360) % 360,
                _ => panic!("invalid direction")
            }

            match d {
                0 => p[1] -= n,
                90 => p[0] += n,
                180 => p[1] += n,
                270 => p[0] -= n,
                _ => panic!("invalid degree value"),
            }
        }

        crate::output(p[0].abs() + p[1].abs())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut p = (0, 0);
        let mut d = 0;
        let mut m: HashMap<(i32, i32), bool> = HashMap::new();
        let mut s = (0, 0, false);

        while !s.2 {
            for i in 0..self.d.len() {
                let n: i32 = self.d[i][1..self.d[i].len()].parse::<i32>().unwrap();

                match &self.d[i][0..1] {
                    "R" => d = (d + 90) % 360,
                    "L" => d = ((d - 90) + 360) % 360,
                    _ => panic!("invalid direction")
                }

                match d {
                    0 => s = contains(&mut m, &mut p, n, 0, -1),
                    90 => s = contains(&mut m, &mut p, n, 1, 0),
                    180 => s = contains(&mut m, &mut p, n, 0, 1),
                    270 => s = contains(&mut m, &mut p, n, -1, 0),
                    _ => panic!("invalid degree value"),
                }

                if s.2 {
                    break;
                }
            }
        }

        crate::output(s.0.abs() + s.1.abs())
    }
}
