const S: usize = 1000;

pub struct Aoc2021_05 {
    cords: Vec<((usize, usize), (usize, usize))>,
    map: Vec<Vec<i32>>
}
        
impl Aoc2021_05 {
    pub fn new() -> Self {
        Self { 
            cords: vec![],
            map: vec![vec![0; S]; S]
        }
    }

    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for i in 0..self.map.len() {
            for j in 0..self.map[i].len() {
                if self.map[i][j] > 1 {
                    sum += 1;
                }
            }
        }

        sum
    } 
}
        
impl crate::Solution for Aoc2021_05 {
    fn name(&self) -> (usize, usize) {
        (2021, 05)
    }
        
    fn parse(&mut self) {
        let p = aoc::read_to_slice("input/2021/05.txt", " -> ");
        
        for i in 0..p.len() {
            let mut v: Vec<usize> = p[i][0].split(',').map(|n| n.parse().unwrap()).collect();
            let s: (usize, usize) = (v[0], v[1]);
            v.clear();
            v = p[i][1].split(',').map(|n| n.parse().unwrap()).collect();
            self.cords.push((s, (v[0], v[1])));
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        for i in 0..self.cords.len() {
            let ma: usize;
            let mi: usize;

            if self.cords[i].0.0 == self.cords[i].1.0 {
                ma = std::cmp::max(self.cords[i].0.1, self.cords[i].1.1);
                mi = std::cmp::min(self.cords[i].0.1, self.cords[i].1.1);
    
                for j in mi..=ma {
                    self.map[j][self.cords[i].0.0] += 1;
                }
            } else if self.cords[i].0.1 == self.cords[i].1.1 {
                ma = std::cmp::max(self.cords[i].0.0, self.cords[i].1.0);
                mi = std::cmp::min(self.cords[i].0.0, self.cords[i].1.0);
    
                for j in mi..=ma {
                    self.map[self.cords[i].0.1][j] += 1;
                }
            }
            
        }
        crate::output(self.sum())
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}