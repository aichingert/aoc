use aoc::slice;

pub struct Aoc2015_15 {
    values: Vec<[i64; 4]>,
    calories: Vec<i64>,
    perms: Vec<[i64; 4]>,
}
        
impl Aoc2015_15 {
    pub fn new() -> Self {
        Self { 
            values: vec![],
            calories: vec![],
            perms: vec![] 
        }
    }
}
        
impl crate::Solution for Aoc2015_15 {
    fn name(&self) -> (usize, usize) {
        (2015, 15)
    }
        
    fn parse(&mut self) {
        for l in slice("input/2015/15.txt", "\n") {
            let line = l.split(' ').collect::<Vec<&str>>();
            self.values.push([
                line[2][0..line[2].len() - 1].parse().unwrap(), 
                line[4][0..line[4].len() - 1].parse().unwrap(), 
                line[6][0..line[6].len() - 1].parse().unwrap(), 
                line[8][0..line[8].len() - 1].parse().unwrap(),
            ]);
            self.calories.push(line[10].parse().unwrap());
        }

        for i in 0..=100 {
            for j in 0..=100 {
                if i + j > 100 {
                    break;
                }
                for k in 0..=100 {
                    if i + j + k > 100 {
                        break;
                    }
                    for l in 0..=100 {
                        if i + j + k + l > 100 {
                            break;
                        }
                        if i + j + k + l == 100 {
                            self.perms.push([i, j, k, l]);
                        }
                    }
                }
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut max: i64 = 0;

        for i in 0..self.perms.len() {
            let mut storage: [i64; 4] = [0; 4];
            for j in 0..self.values.len() {
                for k in 0..self.values[j].len() {
                    storage[k] += self.values[j][k] * self.perms[i][j];
                }
            }

            let mut total: i64 = 1;
            for j in 0..storage.len() {
                if storage[j] < 0 { 
                    storage[j] = 0; 
                }
                total *= storage[j];
            }

            max = max.max(total);
        }

        crate::output(max)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut max: i64 = 0;

        for i in 0..self.perms.len() {
            let mut storage: [i64; 4] = [0; 4];
            let mut calories: i64 = 0;
            
            for j in 0..self.values.len() {
                for k in 0..self.values[j].len() {
                    storage[k] += self.values[j][k] * self.perms[i][j];
                }
                calories += self.calories[j] * self.perms[i][j];
            }

            if calories != 500 {
                continue;
            }

            let mut total: i64 = 1;
            for j in 0..storage.len() {
                if storage[j] < 0 { 
                    storage[j] = 0; 
                }
                total *= storage[j];
            }

            max = max.max(total);
        }

        crate::output(max)
    }
}
