use aoc::numbers;

pub struct Aoc2015_02 {
    d: Vec<P>
}

pub struct P([u32; 3]);

impl P {
    fn new(v: &[u32]) -> Self {
        Self([v[0], v[1], v[2]])
    }

    fn area(&self) -> u32 {
        2 * self.0[0] * self.0[1] + 2 * self.0[0] * self.0[2] + 2 * self.0[1] * self.0[2]
    }

    fn slack(&self) -> u32 {
        self.0[0] * self.0[1]
    }

    fn ribbon(&self) -> u32 {
        2 * self.0[0] + 2 * self.0[1] + self.0[0] * self.0[1] * self.0[2]
    }
}

impl Aoc2015_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2015_02 {
    fn name(&self) -> (usize, usize) {
        (2015, 2)
    }

    fn parse(&mut self) {
        let mut d: Vec<Vec<u32>> = numbers("input/2015/02.txt", 'x');
        for i in 0..d.len() {
            d[i].sort();
            self.d.push(P::new(&d[i]));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            self.d
                .iter()
                .map(| p | p.area() + p.slack())
                .sum::<u32>()
        )
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(
            self.d
                .iter()
                .map( | p | p.ribbon())
                .sum::<u32>()   
        )
    }
}