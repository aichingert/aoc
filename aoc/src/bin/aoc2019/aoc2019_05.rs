use aoc::read_number_stream;

pub struct Aoc2019_05 {
    d: Vec<i32>
}
        
impl Aoc2019_05 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2019_05 {
    fn name(&self) -> (usize, usize) {
        (2019, 05)
    }
        
    fn parse(&mut self) {
        self.d = read_number_stream("input/2019/05.txt", ",");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut idx: usize = 0;
        let mut s: i32 = 4;

        loop {
            let inst: usize = self.d[idx] as usize;
            let mut j: usize = 0;

            if inst == 99 {
                break;
            }

            match inst {
                1 => {
                    let f: usize = self.d[idx + 1] as usize;
                    let s: usize = self.d[idx + 2] as usize;
                    let t: usize = self.d[idx + 3] as usize;

                    self.d[t] = self.d[f] + self.d[s];
                },
                2 => {
                    let f: usize = self.d[idx + 1] as usize;
                    let s: usize = self.d[idx + 2] as usize;
                    let t: usize = self.d[idx + 3] as usize;

                    self.d[t] = self.d[f] * self.d[s];
                },
                3 => {
                    let f: usize = self.d[idx + 1] as usize;
                    self.d[f] = 1;
                    j = 2;
                },
                4 => {
                    let f: usize = self.d[idx + 1] as usize;
                    s = self.d[f];
                    j = 2;
                }
                _ => {
                    let v: String = inst.to_string();
                    let c: usize = v[v.len()-2..v.len()].parse::<usize>().unwrap();
                    let mut h: usize = 0;
                    let mut t: usize = 0;

                    if v.len() > 2 {
                        h = v[v.len()-3..=v.len() - 3].parse::<usize>().unwrap();
                    }

                    if v.len() > 3 {
                        t = v[v.len()-4..=v.len() - 4].parse::<usize>().unwrap();
                    }


                    let mut d: bool = false;

                    match c {
                        3 => {
                            self.d[idx + 1] = 1;
                            j = 2;
                            d = true;
                        },
                        4 => {
                            s = self.d[idx + 1];
                            j = 2;
                            d = true;
                        },
                        _ => {}
                    }

                    if !d {

                        let f: i32;
                        let s: i32;
                        let th: i32;
        
                        if h == 1 {
                            f = self.d[idx + 1];
                        } else {
                            f = self.d[self.d[idx + 1] as usize];
                        }
        
                        if t == 1 {
                            s = self.d[idx + 2];
                        } else {
                            s = self.d[self.d[idx + 2] as usize];
                        }
        
                        th = self.d[idx + 3];
        
                        match c {
                            99 => {
                                break;
                            },
                            1 => {
                                self.d[th as usize] = f + s;
                            },
                            2 => {
                                self.d[th as usize] = f * s;
                            }
                            _ => {
                                println!("oh no, {}", c)
                            }
                        }
                    }
                }
            }

            idx += j;
        }

        crate::output(s)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}