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
            let mut j: usize = 4;

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
        self.parse();
        let mut idx: usize = 0;
        let mut s: i32 = 0;

        loop {
            let inst: usize = self.d[idx] as usize;
            let mut j: usize = 4;

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
                    self.d[f] = 5;
                    j = 2;
                },
                4 => {
                    let f: usize = self.d[idx + 1] as usize;
                    s = self.d[f];
                    j = 2;
                },
                5 => {
                    let f: usize = self.d[idx + 1] as usize;
                    let s: usize = self.d[idx + 2] as usize;
                    
                    if self.d[f] != 0 {
                        idx = self.d[s] as usize;
                        continue;
                    }

                    j = 3;
                },
                6 => {
                    let f: usize = self.d[idx + 1] as usize;
                    let s: usize = self.d[idx + 2] as usize;
                    
                    if self.d[f] == 0 {
                        idx = self.d[s] as usize;
                        continue;
                    }

                    j = 3;
                },
                7 => {
                    let f: usize = self.d[idx + 1] as usize;
                    let s: usize = self.d[idx + 2] as usize;
                    let t: usize = self.d[idx + 3] as usize;

                    if self.d[f] < self.d[s] {
                        self.d[t] = 1;
                    } else {
                        self.d[t] = 0;
                    }
                },
                8 => {
                    let f: usize = self.d[idx + 1] as usize;
                    let s: usize = self.d[idx + 2] as usize;
                    let t: usize = self.d[idx + 3] as usize;

                    if self.d[f] == self.d[s] {
                        self.d[t] = 1;
                    } else {
                        self.d[t] = 0;
                    }
                },
                _ => {
                    let v: String = inst.to_string();
                    let mut c: usize = v[v.len()-1..v.len()].parse::<usize>().unwrap();
                    let mut h: usize = 0;
                    let mut t: usize = 0;

                    if v.len() > 1 {
                        c = v[v.len()-2..v.len()].parse::<usize>().unwrap()
                    }

                    if v.len() > 2 {
                        h = v[v.len()-3..=v.len() - 3].parse::<usize>().unwrap();
                    }

                    if v.len() > 3 {
                        t = v[v.len()-4..=v.len() - 4].parse::<usize>().unwrap();
                    }

                    let mut d: bool = false;

                    match c {
                        3 => {
                            self.d[idx + 1] = 5;
                            j = 2;
                            d = true;
                        },
                        4 => {
                            s = self.d[idx + 1];
                            j = 2;
                            d = true;
                        },
                        5 => {
                            if h == 1 {
                                if t == 1 {
                                    if self.d[idx + 1] != 0 {
                                        idx = self.d[idx + 2] as usize;
                                        continue;
                                    }
                                } else {
                                    if self.d[idx + 1] != 0 {
                                        idx = self.d[self.d[idx + 2] as usize] as usize;
                                        continue;
                                    }
                                }
                            } else {
                                if t == 1 {
                                    if self.d[self.d[idx + 1] as usize] != 0 {
                                        idx = self.d[idx + 2] as usize;
                                        continue;
                                    }
                                } else {
                                    if self.d[self.d[idx + 1] as usize] != 0 {
                                        idx = self.d[self.d[idx + 2] as usize] as usize;
                                        continue;
                                    }
                                }
                            }
                            
                            idx += 3;
                            continue;
                        },
                        6 => {
                            if h == 1 {
                                if self.d[idx + 1] == 0 {
                                    if t == 1 {
                                        idx = self.d[idx + 2] as usize;
                                    } else {
                                        idx = self.d[self.d[idx + 2] as usize] as usize;
                                    }
                                    continue;
                                }
                            } else {
                                if self.d[self.d[idx + 1] as usize] == 0 {
                                    if t == 1 {
                                        idx = self.d[idx + 2] as usize;
                                    } else {
                                        idx = self.d[self.d[idx + 2] as usize] as usize;
                                    }
                                    continue;
                                }
                            }
                            
                            idx += 3;
                            continue;
                        },
                        7 => {
                            let q = self.d.clone();
                            if h == 1 {
                                if t == 1 {
                                    if self.d[idx + 1] < self.d[idx + 2] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                } else {
                                    if self.d[idx + 1] < self.d[self.d[idx + 2] as usize] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                }
                            } else {
                                if t == 1 {
                                    if self.d[self.d[idx + 1] as usize] < self.d[idx + 2] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                } else {
                                    if self.d[self.d[idx + 1] as usize] < self.d[self.d[idx + 2] as usize] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                }
                            }
                            d = true;
                        },
                        8 => {
                            let q = self.d.clone();
                            if h == 1 {
                                if t == 1 {
                                    if self.d[idx + 1] == self.d[idx + 2] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                } else {
                                    if self.d[idx + 1] == self.d[self.d[idx + 2] as usize] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                }
                            } else {
                                if t == 1 {
                                    if self.d[self.d[idx + 1] as usize] == self.d[idx + 2] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                } else {
                                    if self.d[self.d[idx + 1] as usize] == self.d[self.d[idx + 2] as usize] {
                                        self.d[q[idx + 3] as usize] = 1;
                                    } else {
                                        self.d[q[idx + 3] as usize] = 0;
                                    }
                                }
                            }
                            d = true;
                        }
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
                            },
                            _ => {
                                println!("oh no, {}", c)
                            }
                        }
        
                        j = 4;
                    }
                }
            }

            idx += j;
        }

        crate::output(s)
    }
}