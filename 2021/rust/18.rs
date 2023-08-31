#[derive(Clone,Debug)]
enum Snailfish {
    Lit(u32),
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
            _ => {
                let mut acc = String::new();
                let mut cur = 0;

                while cur < stream.len() && stream[cur].is_digit(10) {
                    acc.push(stream[cur]);
                    cur += 1;
                }
                Snailfish::Lit(acc.parse::<u32>().unwrap())
            }
        }
    }

    fn add(self, other: Self) -> Self {
        Self::make_pair(self, other)
    }

    fn split(self, splitted: &mut bool) -> Self {
        match self {
            Self::Lit(n) => if n > 9 {
                let lhs = n / 2;
                let rhs = n / 2 + if n & 1 != 0 { 1 } else { 0 };

                *splitted = true;
                Self::Pair((Box::new(Self::Lit(lhs)),Box::new(Self::Lit(rhs))))
            } else { self },
            Self::Pair((lhs, rhs)) => {
                let lhs = lhs.split(splitted);

                if *splitted {
                    Self::Pair((Box::new(lhs), rhs))
                } else {
                    Self::Pair((Box::new(lhs), Box::new(rhs.split(splitted))))
                }
            }
        }
    }

    fn explode(self, depth: u8) -> (bool, Self, Option<u32>, Option<u32>) {
        match self {
            Self::Lit(_) => (false, self, None, None),
            Self::Pair((lhs, rhs)) => {
                if depth >= 4 && lhs.is_lit() && rhs.is_lit() {
                    return (
                        true, Self::Lit(0), Some(lhs.magnitude()), Some(rhs.magnitude())
                    );
                }

                let (did_explode, new_lhs, ll, lr) = lhs.explode(depth + 1);
                if did_explode {
                    return (
                        true, Self::make_pair(new_lhs, rhs.add_left(lr)), ll, None
                    );
                }

                let (did_explode, new_rhs, rl, rr) = rhs.explode(depth + 1);
                if did_explode {
                    return (
                        true, Self::make_pair(new_lhs.add_right(rl), new_rhs), None, rr 
                    );
                }

                (false, Self::make_pair(new_lhs, new_rhs), None, None)
            }
         }
    }

    fn add_left(self, value: Option<u32>) -> Self {
        match value {
            None => self,
            Some(x) => match self {
                Self::Lit(n) => Self::Lit(x + n),
                Self::Pair((lhs, rhs)) => Self::make_pair(lhs.add_left(value), *rhs),
            }
        }
    }

    fn add_right(self, value: Option<u32>) -> Self {
        match value {
            None => self,
            Some(x) => match self {
                Self::Lit(n) => Self::Lit(x + n),
                Self::Pair((lhs, rhs)) => Self::make_pair(*lhs, rhs.add_right(value)),
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Lit(n) => *n as u32,
            Self::Pair((lhs, rhs)) => {
                3 * lhs.magnitude() + 2 * rhs.magnitude()
            }
        }
    }

    fn make_pair(lhs: Self, rhs: Self) -> Self {
        Self::Pair((Box::new(lhs), Box::new(rhs)))
    }
    
    fn is_lit(&self) -> bool {
        match self {
            Self::Lit(_) => true,
            _ => false,
        }
    }

    fn reduce(self) -> Self {
        let mut sn = self;

        loop {
            let (did_explode, next, _, _) = sn.explode(0);
            sn = next;

            if did_explode {
                continue;
            }

            let mut did_split = false;
            sn = sn.split(&mut did_split);

            if !did_split {
                break;
            }
        }

        sn
    }
}

fn main() {
    let input = std::fs::read_to_string("../input/18").unwrap();
    let input = input.trim().lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut current = Snailfish::from_stream(&input[0]);

    for i in 1..input.len() {
        current = current.add(Snailfish::from_stream(&input[i]));
        current = current.reduce();
    }

    let mut part_two: u32 = u32::MIN;
    
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let a = Snailfish::from_stream(&input[i]);
            let b = Snailfish::from_stream(&input[j]);

            part_two = part_two.max(a.clone().add(b.clone()).reduce().magnitude());
            part_two = part_two.max(b.add(a).reduce().magnitude());
        }
    }

    println!("Part 1: {}", current.magnitude());
    println!("Part 2: {}", part_two);
}
