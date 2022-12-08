pub struct Aoc2022_08 {
    d: Vec<Vec<u8>>
}
        
impl Aoc2022_08 {
    pub fn new() -> Self {
        Self { 
            d: Vec::new()
        }
    }
}

impl crate::Solution for Aoc2022_08 {
    fn name(&self) -> (usize, usize) {
        (2022, 08)
    }

    fn parse(&mut self) {
        self.d = aoc::read_to_map("input/2022/08.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut c: i32 = 0;

        for i in 1..self.d.len()-1 {
            for j in 1..self.d.len()-1 {
                let mut r: bool = true;
                for k in 0..i {
                    if self.d[i][j] <= self.d[k][j] {
                        r = false;
                    }
                }

                if r {
                    c += 1;
                    continue;
                }
                r = true;

                for k in i+1..self.d.len() {
                    if self.d[i][j] <= self.d[k][j] {
                        r = false;
                    }
                }

                if r {
                    c += 1;
                    continue;
                }
                r = true;
                for k in 0..j {
                    if self.d[i][j] <= self.d[i][k] {
                        r = false;
                    }
                }

                if r {
                    c += 1;
                    continue;
                }
                r = true;
                for k in j+1..self.d[i].len() {
                    if self.d[i][j] <= self.d[i][k] {
                        r = false;
                    }
                }

                if r {
                    c += 1;
                    continue;
                }
            }
        }


        c += 2 * self.d.len() as i32 + 2 * (self.d[0].len() - 2) as i32;
        crate::output(c)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut c: i32 = 0;

        for i in 1..self.d.len()-1 {
            for j in 1..self.d.len()-1 {
                let mut u = 0;
                let mut d = 0;
                let mut l = 0;
                let mut r = 0;
                for k in 0..i-1 {
                    if self.d[i][j] > self.d[i-k-1][j] {
                        l += 1;
                    } else {
                        break;
                    }
                }

                for k in i+1..self.d.len()-1 {
                    if self.d[i][j] > self.d[k][j] {
                        r += 1;
                    } else {
                        break;
                    }
                }

                for k in 0..j-1 {
                    if self.d[i][j] > self.d[i][j-k-1] {
                        u += 1;
                    } else {
                        break;
                    }
                }

                for k in j+1..self.d[i].len()-1 {
                    if self.d[i][j] > self.d[i][k] {
                        d += 1;
                    } else {
                        break;
                    }
                }


                l += 1;
                r += 1;
                u += 1;
                d += 1;
                let s = l * r * u * d;
                if s > c {
                    c = s;
                }
            }
        }

        crate::output(c)
    }
}