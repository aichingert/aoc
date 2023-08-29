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

    fn explode(self, explosion: &mut bool, remainding: &mut (Option<u8>, Option<u8>), depth: u8) -> Self {
        match self {
            Self::Lit(_) => self,
            Self::Pair((lhs, rhs)) => {
                if depth < 4 {
                    let lhs = lhs.explode(explosion, remainding, depth + 1);

                    if *explosion {
                        match *remainding {
                            (None, None) => return Self::Pair((Box::new(lhs), rhs)),
                            (Some(x), None) | (Some(x), Some(_)) => if let Some(y) = rhs.is_literal() {
                                remainding.0 = None;
                                return Self::Pair((Box::new(lhs), Box::new(Self::Lit(x + y))));
                            } else {
                                return Self::Pair((Box::new(lhs), rhs));
                            }
                            _ => {},
                        }
                    }

                    return Self::Pair((Box::new(lhs), rhs));
                }             

                match (*lhs, *rhs) {
                    (Self::Lit(lhs), Self::Lit(rhs)) => println!("{:?} - {:?}", lhs, rhs),
                    _ => panic!("should not happen :("),
                }

                Self::Lit(0)
            }
        }
    }

    fn is_literal(&self) -> Option<u8> {
        match self {
            Self::Lit(n) => Some(*n),
            Self::Pair(_) => None,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/18").unwrap();
    let input = input.trim().lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let sn = Snailfish::from_stream(&input[0]);

    println!("{:?} \n", sn);

    let split = sn.split(&mut false);
    println!("{:?} \n", split);

    let split = split.split(&mut false);
    println!("{:?} \n", split);

    split.explode(&mut false, &mut (None, None), 0);
}
