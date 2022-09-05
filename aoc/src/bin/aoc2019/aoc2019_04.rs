use aoc::slice;

pub struct Aoc2019_04 {
    d: Vec<String>
}

impl Aoc2019_04 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2019_04 {
    fn name(&self) -> (usize, usize) {
        (2019, 4)
    }

    fn parse(&mut self) {
        self.d = slice("input/2019/04.txt", "-");
    }

    fn part1(&mut self) -> Vec<String> {
        let b: Vec<usize> = self.d.iter().map( | x | x.parse::<usize>().expect("invalid input")).collect();
        let mut n: i32 = 0;

        for i in b[0]..b[1] {
            let c: Vec<char> = i.to_string().chars().collect();
        let mut d: bool = false;

        for j in 0..c.len() -1 {
            if c[j] <= c[j + 1] {
                if c[j] == c[j + 1] {
                    d = true;
                }
            } else {
                d = false;
                break;
            }
        }

        if d {
            n += 1;
        }
        }

        crate::output(n)
    }

    fn part2(&mut self) -> Vec<String> {
        let b: Vec<usize> = self.d.iter().map( | x | x.parse::<usize>().expect("invalid input")).collect();
        let mut n: i32 = 0;

        for i in b[0]..b[1] {
            let c: Vec<char> = i.to_string().chars().collect();
            let mut d: bool = false;

            for j in 0..c.len() -1 {
                if c[j] <= c[j + 1] {
                    if c[j] == c[j + 1] {
                        d = true;
                    }
                } else {
                    d = false;
                    break;
                }
            }

            let mut dd: bool = false;

            if d {
                let mut j: usize = 0;
                while j < c.len() {
                    let mut l: usize = 1;

                    for k in j + 1..c.len() {
                        if c[j] == c[k] {
                            l += 1;
                        } else {
                            break;
                        }
                    }

                    j += l;

                    if l == 2 {
                        dd = true;
                        break;
                    }
                }
        
                if dd {
                    n += 1;
                }
            }
        }
        crate::output(n)
    }
}