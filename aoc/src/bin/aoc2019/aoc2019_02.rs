use aoc::numbers;

pub struct Aoc2019_02 {
    d: Vec<usize>
}

impl Aoc2019_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn solve(data: &mut Vec<usize>) -> usize {
        let mut i: usize = 0;

        loop {
            let inst = data[i];
            let f: usize = data[i + 1];
            let s: usize = data[i + 2];
            let t: usize = data[i + 3];

            match inst {
                1 => data[t] = data[f] + data[s],
                2 => data[t] = data[f] * data[s],
                99 => break,
                _ => panic!("invalid instruction")
            }
            i+=4;
        } 
        data[0]   
    }
}

impl crate::Solution for Aoc2019_02 {
    fn name(&self) -> (usize, usize) {
        (2019, 2)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2019/02.txt", ",")[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut cl: Vec<usize> = self.d.clone();
        cl[1] = 12;
        cl[2] = 2;
        crate::output(Aoc2019_02::solve(&mut cl))
    }

    fn part2(&mut self) -> Vec<String> {
        for n in 0..100 {
            for v in 0..100 {
                let mut c = self.d.clone();
                c[1] = n;
                c[2] = v;

                if Aoc2019_02::solve(&mut c) == 19690720 {
                    return crate::output(100 * n + v);
                }
            }
        }

        panic!("invalid input")
    }
}