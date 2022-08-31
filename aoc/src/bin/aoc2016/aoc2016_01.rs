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

fn contains_0(m: &mut HashMap<(i32, i32), bool>, p: &mut (i32, i32), n: i32) -> (i32, i32, bool) {
    for i in 0..n {
        if m.contains_key(&(p.0, p.1 - i)) {
            return (p.0, p.1 - i, true);
        } else {
            m.insert((p.0, p.1 - i), true);
        }
    }

    p.1 -= n;
    (0, 0, false)
}

fn contains_90(m: &mut HashMap<(i32, i32), bool>, p: &mut (i32, i32), n: i32) -> (i32, i32, bool) {
    for i in 0..n {
        if m.contains_key(&(p.0 + i, p.1)) {
            return (p.0 + i, p.1, true);
        } else {
            m.insert((p.0 + i, p.1), true);
        }
    }

    p.0 += n;
    (0, 0, false)
}

fn contains_180(m: &mut HashMap<(i32, i32), bool>, p: &mut (i32, i32), n: i32) -> (i32, i32, bool) {
    for i in 0..n {
        if m.contains_key(&(p.0, p.1 + i)) {
            return (p.0, p.1 + i, true);
        } else {
            m.insert((p.0, p.1 + i), true);
        }
    }

    p.1 += n;
    (0, 0, false)
}

fn contains_270(m: &mut HashMap<(i32, i32), bool>, p: &mut (i32, i32), n: i32) -> (i32, i32, bool) {
    for i in 0..n {
        if m.contains_key(&(p.0 - i, p.1)) {
            return (p.0 - i, p.1, true);
        } else {
            m.insert((p.0 - i, p.1), true);
        }
    }

    p.0 -= n;
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
                "R" => {
                    d = (d + 90) % 360;
                },
                "L" => {
                    d = ((d - 90) + 360) % 360;
                }
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
                    "R" => {
                        d = (d + 90) % 360;
                    },
                    "L" => {
                        d = ((d - 90) + 360) % 360;
                    }
                    _ => panic!("invalid direction")
                }

                match d {
                    0 => s = contains_0(&mut m, &mut p, n),
                    90 => s = contains_90(&mut m, &mut p, n),
                    180 => s = contains_180(&mut m, &mut p, n),
                    270 => s = contains_270(&mut m, &mut p, n),
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