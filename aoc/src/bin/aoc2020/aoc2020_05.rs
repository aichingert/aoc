use aoc::get_chars;

pub struct Aoc2020_05 {
    d: Vec<Vec<char>>,
    s: Vec<(usize, usize)>
}

impl Aoc2020_05 {
    pub fn new() -> Self {
        Self { 
            d: vec![], 
            s: vec![]
        }
    }
}

impl crate::Solution for Aoc2020_05 {
    fn name(&self) -> (usize, usize) {
        (2020, 5)
    }

    fn parse(&mut self) {
        self.d = get_chars("input/2020/05.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut s: usize = 0;

        for i in 0..self.d.len() {
            let x: usize;
            let y: usize;

            let mut min: usize = 0;
            let mut max: usize = 127;

            for j in 0..6 {
                let mut n: usize;

                if self.d[i][j] == 'F' {
                    n = max - min;

                if n % 2 != 0 {
                    n = n / 2 + 1;
                } else {
                    n = n / 2;
                }

                max -= n;
                } else {
                    n = max - min;

                    if n % 2 != 0 {
                        n = n / 2 + 1;
                    } else {
                        n = n / 2;
                    }
                    min += n;
                }
            }

            if self.d[i][6] == 'F' {
                y = min;
            } else {
                y = max;
            }
            
            min = 0;
            max = 7;

            for j in 7..self.d[i].len() - 1 {
                let mut new_val: usize;
    
                if self.d[i][j] == 'L' {
                    new_val = max - min;
    
                    if new_val % 2 != 0 {
                        new_val = new_val / 2 + 1;
                    } else {
                        new_val = new_val / 2;
                    }
    
                    max -= new_val;
                } else {
                    new_val = max - min;
    
                    if new_val % 2 != 0 {
                        new_val = new_val / 2 + 1;
                    } else {
                        new_val = new_val / 2;
                    }
    
                    min += new_val;
                }
            }
    
            if self.d[i][self.d[i].len() - 1] == 'L' {
                x = min;
            } else {
                x = max;
            }
    
            if y * 8 + x > s {
                s = y * 8 + x;
            }
    
            self.s.push((y, x));
        }
        
        crate::output(s)
    }

    fn part2(&mut self) -> Vec<String> {
        todo!()
    }
}