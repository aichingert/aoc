use aoc::slice;

pub struct Aoc2019_04 {
    d: Vec<String>
}

impl Aoc2019_04 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2019_04 {
    fn name(&self) -> (usize, usize) {
        (2019, 4)
    }

    fn parse(&mut self) {
        self.d = slice("input/2019/04.txt", "-");
    }

    fn part1(&mut self) -> Vec<String> {
        let b: Vec<usize> = self.d.iter().map( | x | x.parse::<usize>().expect("invalid input")).collect();
        let mut n: i32 = 0;

        for i in b[0]..b[1] {
            let c: Vec<char> = i.to_string().chars().collect();
        let mut d: bool = false;

        for j in 0..c.len() -1 {
            if c[j] <= c[j + 1] {
                if c[j] == c[j + 1] {
                    d = true;
                }
            } else {
                d = false;
                break;
            }
        }

        if d {
            n += 1;
        }
        }

        crate::output(n)
    }

    fn part2(&mut self) -> Vec<String> {
        todo!()
    }
}