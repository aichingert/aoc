use aoc::read_to_slice;

pub struct Aoc2021_02 {
    i: Vec<(String, i32)>,
    d: i32,
    h: i32
}

impl Aoc2021_02 {
    pub fn new() -> Self {
        Self { 
            i: vec![],
            d: 0,
            h: 0,
        }
    }
}

impl crate::Solution for Aoc2021_02 {
    fn name(&self) -> (usize, usize) {
        (2021, 2)
    }

    fn parse(&mut self) {
        read_to_slice("input/2021/02.txt", " ")
            .iter()
            .for_each(|e| self.i.push((e[0].clone(), e[1].parse::<i32>().expect("invalid input"))));
    }

    fn part1(&mut self) -> Vec<String> {
        for i in 0..self.i.len() {
            match &self.i[i].0 as &str {
                "forward" => self.h += self.i[i].1,
                "up" => self.d -= self.i[i].1,
                "down" => self.d += self.i[i].1,
                _ => panic!("couldn't match {}", self.i[i].0)
            }
        }

        crate::output(self.d * self.h)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut a: i32 = 0;
        self.h = 0;
        self.d = 0;

        for i in 0..self.i.len() {
            match &self.i[i].0 as &str {
                "forward" => {
                    self.h += self.i[i].1;
                    self.d += self.i[i].1 * a;
                },
                "up" => a -= self.i[i].1,
                "down" => a += self.i[i].1,
                _ => panic!("couldn't match {}", self.i[i].0)
            }
        }

        crate::output(self.d * self.h)
    }
}
