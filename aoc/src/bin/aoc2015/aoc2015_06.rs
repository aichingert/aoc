pub struct Aoc2015_06 {
    d: Vec<(State, ((usize, usize), (usize, usize)))>,
    m: Vec<Vec<u8>>
}

enum State {
    On,
    Off,
    Toggle
}
        
impl Aoc2015_06 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
            m: vec![vec![0; 1000]; 1000]
        }
    }

    fn count(&self) -> i64 {
        let mut c: i64 = 0;
        for y in 0..self.m.len() {
            for x in 0..self.m[y].len() {
                c += self.m[y][x] as i64;
            }
        }
        c
    }
}
        
impl crate::Solution for Aoc2015_06 {
    fn name(&self) -> (usize, usize) {
        (2015, 06)
    }
        
    fn parse(&mut self) {
        let input = aoc::read_to_slice("input/2015/06.txt", " ");

        for i in 0..input.len() {
            let mut tuple: ((usize,usize),(usize,usize)) = ((0,0),(0,0));
            let mut state: State = State::Toggle;
            if &input[i][0] != "toggle" {
                let values: Vec<usize> = input[i][2].split(',').map(|s| s.parse().expect("invalid input")).collect();
                tuple.0 = (values[0], values[1]);
                let values: Vec<usize> = input[i][4].split(',').map(|s| s.parse().expect("invalid input")).collect();
                tuple.1 = (values[0], values[1]);

                if &input[i][1] == "on" {
                    state = State::On;
                } else {
                    state = State::Off;
                }
            } else {
                let values: Vec<usize> = input[i][1].split(',').map(|s| s.parse().expect("invalid input")).collect();
                tuple.0 = (values[0], values[1]);
                let values: Vec<usize> = input[i][3].split(',').map(|s| s.parse().expect("invalid input")).collect();
                tuple.1 = (values[0], values[1]);
            }

            self.d.push((state, tuple));
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        for i in 0..self.d.len() {
            for y in self.d[i].1.0.0..=self.d[i].1.1.0 {
                for x in self.d[i].1.0.1..=self.d[i].1.1.1 {
                    match self.d[i].0 {
                        State::On => self.m[y][x] = 1,
                        State::Off => self.m[y][x] = 0,
                        State::Toggle => self.m[y][x] = if self.m[y][x] == 1 { 0 } else { 1 },
                    }
                }
            }
        }

        crate::output(self.count())
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.m = vec![vec![0; 1000];1000];
        for i in 0..self.d.len() {
            for y in self.d[i].1.0.0..=self.d[i].1.1.0 {
                for x in self.d[i].1.0.1..=self.d[i].1.1.1 {
                    match self.d[i].0 {
                        State::On => self.m[y][x] += 1,
                        State::Off => self.m[y][x] -= if self.m[y][x] > 0 { 1 } else { 0 },
                        State::Toggle => self.m[y][x] += 2,
                    }
                }
            }
        }

        crate::output(self.count())
    }
}
