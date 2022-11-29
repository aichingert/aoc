pub struct Aoc2015_06 {
    d: Vec<(State, ((usize, usize), (usize, usize)))>,
    m: Vec<Vec<bool>>
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
            m: vec![vec![false; 1000]; 1000]
        }
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
        let mut c: i64 = 0;

        for i in 0..self.d.len() {
            for y in self.d[i].1.0.0..=self.d[i].1.1.0 {
                for x in self.d[i].1.0.1..=self.d[i].1.1.1 {
                    match self.d[i].0 {
                        State::On => self.m[y][x] = true,
                        State::Off => self.m[y][x] = false,
                        State::Toggle => self.m[y][x] = !self.m[y][x],
                    }
                }
            }
        }

        for i in 0..self.m.len() {
            for j in 0..self.m[i].len() {
                if self.m[i][j] {
                    c += 1;
                }
            }
        }

        crate::output(c)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
