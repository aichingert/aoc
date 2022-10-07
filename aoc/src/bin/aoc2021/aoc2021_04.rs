use aoc::slice;

pub struct Aoc2021_04 {
    d: Vec<i32>,
    b: Vec<Board>
}

#[derive(Debug)]
struct Board {
    b: [[(i32, bool); 5]; 5]
}

impl Aoc2021_04 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
            b: vec![] 
        }
    }
}

impl Board {
    fn new(b: [[(i32, bool);5];5]) -> Self {
        Self { b }
    }

    fn bingo(&self) -> bool {
        let mut h: bool;
        let mut d: bool;

        for i in 0..self.b.len() {
            h = true;
            d = true;
            for j in 0..self.b[i].len() {
                if !self.b[i][j].1 {
                    h = false;
                }

                if !self.b[j][i].1 {
                    d = false;
                }

                if !h && !d {
                    break;
                }
            }

            if h || d {
                return true;
            }
        }

        false
    }
}

impl crate::Solution for Aoc2021_04 {
    fn name(&self) -> (usize, usize) {
        (2021, 4)
    }

    fn parse(&mut self) {
        let mut p = slice("input/2021/04.txt", "\n");
        self.d = p.remove(0).split(',').map( | v | v.parse::<i32>().expect("invalid input")).collect();
        p.remove(0);

        let mut a: [[(i32, bool); 5]; 5] = [[(0, false); 5]; 5];
        let mut y: usize = 0;
    
        for i in 0..p.len() {
            if p[i] == "" {
                self.b.push(Board::new(a));
                y = 0;
                continue;
            }
    
    
            let t: Vec<&str> = p[i].split(' ').collect();
            let mut v: Vec<i32> = vec![];
    
            for j in 0..t.len() {
                if t[j] != "" {
                    v.push(t[j].parse::<i32>().unwrap());
                }
            }
    
            for j in 0..v.len() {
                a[y][j].0 = v[j];
            }
    
            y += 1;
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut r: i32 = 0;

        'main: for i in 0..self.d.len() {
            for j in 0..self.b.len() {
                for k in 0..self.b[j].b.len() {
                    for l in 0..self.b[j].b[k].len() {
                        if self.b[j].b[k][l].0 == self.d[i] {
                            self.b[j].b[k][l].1 = true;
                        }
                    }
                }
    
                if Board::bingo(&self.b[j]) {
                    for k in 0..self.b[j].b.len() {
                        for l in 0..self.b[j].b[k].len() {
                            if !self.b[j].b[k][l].1 {
                                r += self.b[j].b[k][l].0;
                            }
                        }
                    }
                    r *= self.d[i];
                    break 'main;
                }
            }
        }

        crate::output(r)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut r: i32 = 0;

        'main: for i in 0..self.d.len() {
            let mut j = 0;

            while j < self.b.len() {
                for k in 0..self.b[j].b.len() {
                    for l in 0..self.b[j].b[k].len() {
                        if self.b[j].b[k][l].0 == self.d[i] {
                            self.b[j].b[k][l].1 = true;
                        }
                    }
                }
        
                if Board::bingo(&self.b[j]) {
                    if self.b.len() > 1 {
                        self.b.remove(j);
                        continue;
                    } else {
                        for k in 0..self.b[j].b.len() {
                            for l in 0..self.b[j].b[k].len() {
                                if !self.b[j].b[k][l].1 {
                                    r += self.b[j].b[k][l].0;
                                }
                            }
                        }

                        r *= self.d[i];
                        break 'main;
                    }
                }

                j+=1;
            }
        }
        crate::output(r)
    }
}
