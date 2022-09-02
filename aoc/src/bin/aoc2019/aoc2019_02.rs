use aoc::numbers;

pub struct Aoc2019_02 {
    d: Vec<usize>
}

impl Aoc2019_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2019_02 {
    fn name(&self) -> (usize, usize) {
        (2019, 2)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2019/02.txt", ',')[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut i: usize = 0;

        loop {
            let inst = self.d[i];
            let f: usize = self.d[i + 1];
            let s: usize = self.d[i + 2];
            let t: usize = self.d[i + 3];

            match inst {
                1 => self.d[t] = self.d[f] + self.d[s],
                2 => self.d[t] = self.d[f] * self.d[s],
                99 => break,
                _ => panic!("invalid instruction")
            }
            i+=4;
        } 
        crate::output(self.d[0])
    }

    fn part2(&mut self) -> Vec<String> {
        let mut i: usize = 0;
        self.d = numbers("input/2019/02.txt", ',')[0].clone();

        for n in 0..100 {
            for v in 0..100 {
                let mut c = self.d.clone();
                c[1] = n;
                c[2] = v;

                loop {
                    let inst = c[i];
                    let f: usize = c[i + 1];
                    let s: usize = c[i + 2];
                    let t: usize = c[i + 3];
        
                    match inst {
                        1 => c[t] = c[f] + c[s],
                        2 => c[t] = c[f] * c[s],
                        99 => break,
                        _ => panic!("invalid instruction {inst}")
                    }
                    i+=4;
                }
                if c[0] == 19690720 {
                    return crate::output(100 * n + v);
                }
                i = 0;
            }
        }

        panic!("invalid input")
    }
}