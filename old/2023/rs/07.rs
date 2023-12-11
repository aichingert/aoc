use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Eq)]
struct Hand {
    card_t: Type,
    cards: Vec<char>,
    bid: u64,
}

impl Hand {
    fn parse(inp: &str) -> Self {
        let (cards, bid) = inp.split_once(" ").unwrap();
        let (cards, bid) = (cards.chars().collect::<Vec<_>>(), bid.parse::<u64>().unwrap());

        Self {
            card_t: Type::parse(&cards),
            cards,
            bid
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.card_t.cmp(&other.card_t) {
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.card_t.cmp(&other.card_t) {
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let (a, b) = (map_to_val(self.cards[i]), map_to_val(other.cards[i]));

                    if a > b {
                        return Some(Ordering::Greater);
                    } else if a < b {
                        return Some(Ordering::Less);
                    }
                }

                Ordering::Equal
            },
            o => o,
        })
    }
}

fn map_to_val(ch: char) -> u32 {
    match ch {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 0,
        'T' => 10,
        _ if ch.is_digit(10) => (ch as u8 - b'0') as u32,
        _ => panic!("{}", ch),
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.card_t == other.card_t && self.cards == other.cards && self.bid == other.bid
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Type {
    FiveOfKind = 13,
    FourOfKind = 11,
    FullHouse = 9,
    ThreeOfKind = 7,
    TwoPair = 5,
    OnePair = 3,
    HighCard = 1,
}

impl Type {
    fn parse(inp: &Vec<char>) -> Self {
        let mut hm = HashMap::new();

        for i in 0..inp.len() {
            hm.entry(inp[i]).and_modify(|c| *c += 1).or_insert(1);
        }

        if inp.contains(&'J') {
            let j = hm.remove(&'J').unwrap();

            let mut v = hm.values().collect::<Vec<_>>();
            v.sort_unstable();
            
            return match v.len() {
                0 | 1 => Self::FiveOfKind,
                2 => {
                    if j == 3 || *v[v.len() - 1] == 3 || j == 2 && *v[v.len() - 1] == 2 {
                        Self::FourOfKind
                    } else {
                        Self::FullHouse
                    }
                }
                3 => {
                    if j == 2 && *v[v.len() - 1] == 2 {
                        Self::FourOfKind
                    } else {
                        Self::ThreeOfKind
                    }
                }
                4 => Self::OnePair,
                _ => panic!("{:?}", v),
            };
        }

        let mut v = hm.values().collect::<Vec<_>>();
        v.sort_unstable();
        
        match v.len() {
            1 => Self::FiveOfKind,
            2 => {
                if *v[0] == 1 {
                    Self::FourOfKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if *v[0] == 1 && *v[1] == 1 {
                    Self::ThreeOfKind
                } else if *v[0] == 1 && *v[1] == 2 {
                    Self::TwoPair
                } else {
                    panic!("{:?}", v);
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("{:?}", v),
        }

    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/07").unwrap().trim().to_string();
    let mut inp = inp.lines().map(Hand::parse).collect::<Vec<_>>();

    inp.sort();

    let mut ans = 0;

    for i in 0..inp.len() {
        ans += inp[i].bid * (i as u64 + 1);
    }

    println!("{:?}", ans);
}
