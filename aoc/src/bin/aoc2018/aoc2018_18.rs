#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Open,
    Tree,
    Lumberyard
    
}

pub struct Aoc2018_18 {
    map: Vec<Vec<State>>
}

impl Aoc2018_18 {
    pub fn new() -> Self {
        Self {
            map: Vec::new()
        }
    }
    
    fn next_state(&self, x: usize, y: usize) -> State {
        let mut c_t: i32 = 0;
        let mut c_l: i32 = 0;
        
        for i in -1..2 {
            for j in -1..2 {
                if !(i == 0 && j == 0) && self.bounds(x as i32, j, y as i32, i) {
                    match self.map[(y as i32 + i) as usize][(x as i32 + j) as usize] {
                        State::Tree => c_t += 1,
                        State::Lumberyard => c_l += 1,
                        State::Open => ()
                    }
                }
            }
        }
        
        match self.map[y][x] {
            State::Open => {
                if c_t >= 3 {
                    return State::Tree;
                }
            },
            State::Tree => {
                if c_l >= 3 {
                    return State::Lumberyard;
                }
            },
            State::Lumberyard => {
                if c_l < 1 || c_t < 1 {
                    return State::Open;
                }
            }
        }
        
        self.map[y][x]
    }
    
    fn bounds(&self, x: i32, j: i32, y: i32, i: i32) -> bool {
        x+j > -1 && y+i > -1 && x+j < self.map[0].len() as i32 && y+i < self.map.len() as i32
    }
    
    fn count(&self) -> i32 {
        let mut c_t: i32 = 0;
        let mut c_l: i32 = 0;
        
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                match self.map[y][x] {
                    State::Tree => c_t += 1,
                    State::Lumberyard => c_l += 1,
                    State::Open => ()
                }
            }
        }

        c_t * c_l
    }
}


impl crate::Solution for Aoc2018_18 {
    fn name(&self) -> (usize, usize) {
        (2018, 18)
    }
    
    fn parse(&mut self) {     
        let inp: Vec<Vec<char>> = aoc::get_chars("input/2018/18.txt");
        
        for i in 0..inp.len() {
            let mut states: Vec<State> = Vec::new();
            
            for j in 0..inp[i].len() {
                match inp[i][j] {
                    '.' => states.push(State::Open),
                    '#' => states.push(State::Lumberyard),
                    '|' => states.push(State::Tree),
                    _ => ()
                }
            }
            self.map.push(states);
        }
    }
    
    fn part1(&mut self) -> Vec<String> {
        for _ in 0..10 {
            let mut n_m: Vec<Vec<State>> = self.map.clone();
            
            for y in 0..self.map.len() {
                for x in 0..self.map[y].len() {
                    n_m[y][x] = self.next_state(x,y);
                }
            }
            self.map = n_m;
        }
        crate::output(self.count())
        
    }
    
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
        
    }
}