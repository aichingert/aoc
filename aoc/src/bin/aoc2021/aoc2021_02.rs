use aoc::read_to_slice;

pub struct Aoc2021_02 {
    d: Vec<Vec<String>>
}

impl Aoc2021_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2021_02 {
    fn name(&self) -> (usize, usize) {
        (2021, 2)
    }

    fn parse(&mut self) {
        self.d = read_to_slice("input/2021/02.txt", " ");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut d: i32 = 0;
        let mut h: i32 = 0;

        for i in 0..self.d.len() {
            let n: i32 = self.d[i][1].parse::<i32>().expect("invalid input");

            match &self.d[i][0] as &str {
                "forward" => h += n,
                "up" => d -= n,
                "down" => d += n,
                _ => panic!("couldn't match {}", self.d[i][0])
            }
        }

        crate::output(d * h)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut d: i32 = 0;
        let mut h: i32 = 0;
        let mut a: i32 = 0;

        for i in 0..self.d.len() {
            let n: i32 = self.d[i][1].parse::<i32>().expect("invalid input");

            match &self.d[i][0] as &str {
                "forward" => {
                    h += n;
                    d += n * a;
                },
                "up" => a -= n,
                "down" => a += n,
                _ => panic!("couldn't match {}", self.d[i][0])
            }
        }

        crate::output(d * h)
    }
}