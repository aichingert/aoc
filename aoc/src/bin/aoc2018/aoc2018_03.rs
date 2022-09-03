use aoc::read_to_slice;

pub struct Aoc2018_03 {
    d: Vec<Vec<String>>,
    f: Vec<Vec<(i32, i32)>>
}

impl Aoc2018_03 {
    pub fn new() -> Self {
        Self { d: vec![], f: vec![] }
    }

    fn overlapping(&mut self, r: &[usize], p: &[usize]) -> bool {
        for i in 0..r[1] {
            for j in 0..r[0] {
                if self.f[p[1] + i][p[0] + j].0 > 1 {
                    return true;
                }
            }
        }

        false
    }
}

impl crate::Solution for Aoc2018_03 {
    fn name(&self) -> (usize, usize) {
        (2018, 3)
    }

    fn parse(&mut self) {
        self.d = read_to_slice("input/2018/03.txt", " @ ");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut f: Vec<Vec<(i32, i32)>> = vec![vec![(0, 20); 1000]; 1000];
        for i in 0..self.d.len() {
            let c: Vec<&str> = self.d[i][1].split(": ").collect();
            let p: Vec<usize> = c[0].split(',').map( | v | v.parse::<usize>().expect("invalid input")).collect();
            let r: Vec<usize> = c[1].split('x').map( | v | v.parse::<usize>().expect("invalid input")).collect();

            for i in 0..r[1] {
                for j in 0..r[0] {
                    f[p[1] + i][p[0] + j].0 += 1;
                    f[p[1] + i][p[0] + j].1 = self.d[i][0][1..self.d[i][0].len()].parse::<i32>().expect("invalid input");                 
                }
            }

        }
        self.f = f.clone();
        crate::output(f.iter().map( | y | y.iter().map( | x | if x.0 > 1 {1} else {0}).sum::<i32>()).sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut id: u16 = 0;
        for i in 0..self.d.len() {
            let c: Vec<&str> = self.d[i][1].split(": ").collect();
            let p: Vec<usize> = c[0].split(',').map( | v | v.parse::<usize>().expect("invalid input")).collect();
            let r: Vec<usize> = c[1].split('x').map( | v | v.parse::<usize>().expect("invalid input")).collect();

            if !self.overlapping(&r, &p) {
                id = self.d[i][0][1..self.d[i][0].len()].parse::<u16>().expect("invalid input");
            }
        }
        crate::output(id)
    }
}