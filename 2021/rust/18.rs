#[derive(Clone,Debug)]
enum Snailfish {
    Lit(u8),
    Pair((Box<Snailfish>, Box<Snailfish>)),
}

impl Snailfish {
    fn from_stream(stream: &[char]) -> Snailfish {
        match stream[0] {
            '[' => {
                let mut depth = 1;
                let mut cur = 1;
                let mut mid = 0;

                while cur < stream.len() {
                    match stream[cur] {
                        '[' => depth += 1,
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        },
                        ',' => if depth == 1 { mid = cur },
                        _ => ()
                    }
                    cur += 1;
                }

                Snailfish::Pair((
                    Box::new(Snailfish::from_stream(&stream[1..mid])), 
                    Box::new(Snailfish::from_stream(&stream[mid + 1..cur]))
                ))
            }
            ch => {
                let mut acc = String::new();
                let mut cur = 0;

                while cur < stream.len() && stream[cur].is_digit(10) {
                    acc.push(stream[cur]);
                    cur += 1;
                }
                Snailfish::Lit(acc.parse::<u8>().unwrap())
            }
        }
    }

    fn add(self, other: Self) -> Self {
        Self::Pair((Box::new(self), Box::new(other)))
    }

    fn split(self, is_done: &mut bool) -> Self {
        match self {
            Self::Lit(n) => if n > 9 {
                let lhs = n / 2;
                let rhs = n / 2 + if n & 1 != 0 { 1 } else { 0 };

                *is_done = true;
                Self::Pair((Box::new(Self::Lit(lhs)),Box::new(Self::Lit(rhs))))
            } else {
                self
            }
            Self::Pair((lhs, rhs)) => {
                let lhs = lhs.split(is_done);

                if *is_done {
                    Self::Pair((Box::new(lhs), rhs))
                } else {
                    Self::Pair((Box::new(lhs), Box::new(rhs.split(is_done))))
                }
            }
        }
    }

    fn explode(self, explosion: &mut bool, restored: &mut (Option<u8>, Option<u8>), depth: u8) -> Self {
        self.show();
        println!("           {:?} = {depth}", restored);
        match self {
            Self::Lit(_) => self,
            Self::Pair((lhs, rhs)) => {
                if depth == 4 {
                    match (*lhs, *rhs) {
                        (Self::Lit(l), Self::Lit(r)) => {
                            *explosion = true;
                            *restored = (Some(l), Some(r));
                            return Self::Lit(255);
                        }
                        _ => panic!("to deep"),
                    }    
                } else if *explosion && restored.0.is_none() && restored.1.is_none() {
                    return Self::Pair((lhs, rhs));
                }

                let (lhs, next_to_explosion) = match lhs.explode(explosion, restored, depth + 1) {
                    Self::Lit(n) => if n==255{(Self::Lit(0), true)}else{(Self::Lit(n), false)}
                    p => (p.explode(explosion, restored, depth + 1), false)
                };

                match *restored {
                    (_, Some(x)) => if next_to_explosion {
                        restored.1 = None;
                        let rhs = Box::new(match *rhs {
                            Self::Lit(n) => Self::Lit(x + n),
                            p => p.open(x, false),
                        });
                        return Self::Pair((Box::new(lhs), rhs));
                    }
                    (Some(x), _) => if !next_to_explosion {
                        restored.0 = None;
                        lhs.show();
                        println!("{depth}");
                        let lhs = Box::new(match lhs {
                            Self::Lit(n) => Self::Lit(x + n),
                            p => p.open(x, true),
                        });
                        print!("returning : ");
                        lhs.show();
                        println!();
                        return Self::Pair((lhs, Box::new(*rhs)));
                    }
                    _ => ()
                }

                print!("LHS : ");
                lhs.show();
                println!("");
                let (rhs, next_to_explosion) = match rhs.explode(explosion, restored, depth + 1) {
                    Self::Lit(n) => if n==255{(Self::Lit(0), true)}else{(Self::Lit(n), false)}
                    p => (p.explode(explosion, restored, depth + 1), false)
                };

                match *restored {
                    (Some(x), _) => if next_to_explosion {
                        restored.0 = None;
                        let lhs = Box::new(match lhs {
                            Self::Lit(n) => Self::Lit(n + x),
                            p => p.open(x, true),
                        });
                        return Self::Pair((lhs, Box::new(rhs)));
                    }                     
                    (_, Some(x)) => if !next_to_explosion {
                        restored.1 = None;
                        let rhs = Box::new(match rhs {
                            Self::Lit(n) => Self::Lit(n + x),
                            p => p.open(x, false),
                        });
                        return Self::Pair((Box::new(lhs), rhs));
                    },
                    _ => ()
                }

                Self::Pair((Box::new(lhs), Box::new(rhs)))
            }
        }
    }

    fn open(self, value: u8, right: bool) -> Self {
        match self {
            Self::Lit(n) => Self::Lit(n + value),
            Self::Pair((lhs, rhs)) => {
                if right {
                    Self::Pair((lhs, Box::new(rhs.open(value, right))))
                } else {
                    Self::Pair((Box::new(lhs.open(value, right)), rhs))
                }
            }
        }
    }

    fn magnitude(&self, is_left: bool) -> u32 {
        match self {
            Self::Lit(n) => if is_left { *n as u32 * 3 } else { *n as u32 * 2 },
            Self::Pair((lhs, rhs)) => {
                (match **lhs {
                    Self::Lit(n) => 3 * n as u32,
                    Self::Pair(_) => 3 * lhs.magnitude(true),
                }
                +
                match **rhs {
                    Self::Lit(n) => 2 * n as u32,
                    Self::Pair(_) => 2 * rhs.magnitude(true),
                })
            }
        }
    }

    fn show(&self) {
        match self {
            Self::Lit(n) => print!("{n}"),
            Self::Pair((lhs, rhs)) => {
                print!("[");
                lhs.show();
                print!(",");
                rhs.show();
                print!("]");
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/18").unwrap();
    let input = input.trim().lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let sn = Snailfish::from_stream(&input[0]);

    println!("{:?} ", sn);

    let split = sn.explode(&mut false, &mut (None, None), 0);
    split.show();
    println!();
}
