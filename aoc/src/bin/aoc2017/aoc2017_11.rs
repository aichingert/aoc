use aoc::slice;

pub struct Aoc2017_11 {
    d: Vec<String>,
    f: (f32, f32)
}
        
impl Aoc2017_11 {
    pub fn new() -> Self {
        Self { 
            d: vec![],
            f: (0.0, 0.0) 
        }
    }

    fn get_steps(c: &mut (f32, f32)) -> i32 {
        let mut s: i32 = 0;
    
        while c.0 != 0. || c.1 != 0. {
            if c.0 > 0. && c.1 > 0. {
                while c.0 != 0. {
                    c.0 -= 0.5;
                    c.1 -= 0.5;
                    s += 1;
                }
            } else if c.0 > 0. && c.1 < 0. {
                while c.0 != 0. {
                    c.0 -= 0.5;
                    c.1 += 0.5;
                    s += 1;
                }
            } else if c.0 < 0. && c.1 > 0. {
                while c.0 != 0. {
                    c.0 += 0.5;
                    c.1 -= 0.5;
                    s += 1;
                }
            } else if c.0 < 0. && c.1 < 0. {
                while c.0 != 0. {
                    c.0 += 0.5;
                    c.1 += 0.5;
                    s += 1;
                }
            } else if c.1 > 0. {
                while c.1 != 0. {
                    c.1 -= 1.;
                    s += 1;
                }
            } else {
                while c.1 != 0. {
                    c.1 += 1.;
                    s += 1;
                }
            }
        }
    
        s
    }
}
        
impl crate::Solution for Aoc2017_11 {
    fn name(&self) -> (usize, usize) {
        (2017, 11)
    }
        
    fn parse(&mut self) {
        self.d = slice("input/2017/11.txt", ",");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut c: (f32, f32) = (0., 0.);
        let mut f: (f32, f32) = (0., 0.);

        for i in 0..self.d.len() {
            match &self.d[i] as &str {
                "nw" => {
                    c.0 -= 0.5;
                    c.1 += 0.5;
                },
                "n" => {
                    c.1 += 1.;
                },
                "ne" => {
                    c.0 += 0.5;
                    c.1 += 0.5;
                },
                "se" => {
                    c.0 += 0.5;
                    c.1 -= 0.5;
                },
                "s" => {
                    c.1 -= 1.;
                },
                "sw" => {
                    c.0 -= 0.5;
                    c.1 -= 0.5;
                },
                _ => {
                    println!("{}", self.d[i])
                }
            }
    
            let current: f32 = c.0.abs() + c.1.abs();
            let check: f32 = f.0.abs() + f.1.abs();
    
            if current > check {
                f = c;
            }
        }

        self.f = f;
        crate::output(Aoc2017_11::get_steps(&mut c))
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output(Aoc2017_11::get_steps(&mut self.f))
    }
}