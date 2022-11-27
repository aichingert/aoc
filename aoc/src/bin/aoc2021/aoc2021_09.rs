pub struct Aoc2021_09 {
    m: Vec<Vec<u8>>
}
        
impl Aoc2021_09 {
    pub fn new() -> Self {
        Self { m: vec![] }
    }

    fn open(&self, p: &mut Vec<(usize, usize)>, pos: (usize, usize)) -> u32 {
        let mut b: u32 = 1;
        p.push(pos);

        if pos.0 > 0 && !p.contains(&(pos.0-1, pos.1)) && self.m[pos.0 - 1][pos.1] != 9 {
            b += self.open(p, (pos.0-1, pos.1));
        }
        if pos.0 + 1 < self.m.len() && !p.contains(&(pos.0+1,pos.1)) && self.m[pos.0+1][pos.1] != 9 {
            b += self.open(p, (pos.0+1, pos.1));
        }
        if pos.1 > 0 && !p.contains(&(pos.0, pos.1-1)) && self.m[pos.0][pos.1-1] != 9 {
            b += self.open(p, (pos.0, pos.1-1));
        }
        if pos.1 + 1 < self.m[0].len() && !p.contains(&(pos.0, pos.1+1)) && self.m[pos.0][pos.1+1] != 9 {
            b += self.open(p, (pos.0, pos.1+1));
        }
        
        b
    }
}
        
impl crate::Solution for Aoc2021_09 {
    fn name(&self) -> (usize, usize) {
        (2021, 09)
    }
        
    fn parse(&mut self) {
        self.m = aoc::read_to_map("input/2021/09.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut risk: i32 = 0;
        for i in 0..self.m.len() {
            for j in 0..self.m[i].len() {
                if (i + 1 == self.m.len() || self.m[i][j] < self.m[i+1][j])
                && (j + 1 == self.m[i].len() || self.m[i][j] < self.m[i][j+1])
                && (i == 0 || self.m[i][j] < self.m[i-1][j])
                && (j == 0 || self.m[i][j] < self.m[i][j-1]) {
                    risk += self.m[i][j] as i32 +1;
                }
           }
        }

        crate::output(risk)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut p: Vec<(usize, usize)> = Vec::new();
        let mut b: Vec<u32> = Vec::new();

        for i in 0..self.m.len() {
            for j in 0..self.m[i].len() {
                if self.m[i][j] != 9 && !p.contains(&(i, j)) {
                    b.push(self.open(&mut p, (i, j)));
                }
            }
        }

        b.sort();

        crate::output(b[b.len() - 1] * b[b.len() - 2] * b[b.len() - 3])
    }
}
