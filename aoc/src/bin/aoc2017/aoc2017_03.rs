use aoc::read_to_numbers;

pub struct Aoc2017_03 {
    d: usize
}

impl Aoc2017_03 {
    pub fn new() -> Self {
        Self { d: 0 }
    }

    fn distance(&self, n: i32, r: &mut i32, c: &mut i32, m: i32, d: i32, v: &mut usize) -> bool {
        for _ in 0..n {
            *v += 1;
            if *v > self.d {
                return true;
            }

            *r += 1 * m;
            *c += 1 *d;
        }

        false
    }

    fn sum(s: &Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
        let mut v: i32 = 0;

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 || r as i32 + i > s.len() as i32 || 
                c as i32 + j > s.len() as i32 || r as i32 + i < 0 || 
                c as i32 + j < 0 {
                    continue;
                }
    
                v += s[(r as i32 + i) as usize][(c as i32 + j) as usize];
            }
        }
    
        v
    }

    fn spiral(&self, n: i32, r: &mut usize, c: &mut usize, f: i32, m: i32, s: &mut Vec<Vec<i32>>) -> bool {
        for _ in 0..n {
            s[*r][*c] = Aoc2017_03::sum(s, *r, *c);
            if s[*r][*c] > self.d as i32 {
                return true;
            }
            if f < 0  {
                *r-=1;
            } else if f > 0 {
                *r+=1;
            } else if m < 0 {
                *c-=1;
            } else {
                *c+=1;
            }
        }

        false
    }
}

impl crate::Solution for Aoc2017_03 {
    fn name(&self) -> (usize, usize) {
        (2017, 3)
    }

    fn parse(&mut self) {
        self.d = read_to_numbers("input/2017/03.txt")[0];
    }

    fn part1(&mut self) -> Vec<String> {
        let mut n: usize = 1;
        let mut f: bool = false;
        let mut r: i32 = 0;
        let mut c: i32 = 0;
        let mut u: i32 = 1;
        let mut l: i32 = 1;
        let mut d: i32 = 1;
        let mut v: i32 = 1;

        while !f {
            f = self.distance(u, &mut r, &mut c, -1, 0, &mut n);
            if !f {
                f = self.distance(l, &mut r, &mut c, 0, -1, &mut n);
            }
            if !f {
                f = self.distance(d + 1, &mut r, &mut c, 1, 0, &mut n);
            }
            if !f {
                f = self.distance(v+1, &mut r, &mut c, 0, 1, &mut n);
            }

            u+=2;
            l+=2;
            d+=2;
            v=l;
        }

        crate::output(r.abs() + c.abs())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut f: bool = false;
        let mut s = vec![vec![0; self.d / 1000]; self.d / 1000];
        let mut r: usize = s.len() / 2;
        let mut c: usize = s.len() / 2;
        s[r][c] = 1;
        c+=1;
        let mut u: i32 = 1;
        let mut l: i32 = 2;
        let mut d: i32 = 2;
        let mut v: i32 = 2;

        while !f {
            f = self.spiral(u, &mut r, &mut c, -1, 0, &mut s);
            if !f {
                f = self.spiral(l, &mut r, &mut c, 0, -1, &mut s);
            }
            if !f {
                f = self.spiral(d, &mut r, &mut c, 1, 0, &mut s);
            }
            if !f {
                f = self.spiral(v+1, &mut r, &mut c, 0, 1, &mut s);
            }

            u+=2;
            l+=2;
            d+=2;
            v=l;
        }

        crate::output(s[r][c])
    }
}