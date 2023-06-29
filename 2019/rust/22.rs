const LEN: usize = 10007;
const PART_TWO: usize = 119315717514047;

enum Shuffle {
    NewStack,
    Increment(usize),
    Cut(i32),
}

impl Shuffle {
    fn execute(&self, cards: &mut Vec<u64>) {
        match self {
            Shuffle::NewStack => cards.reverse(),
            Shuffle::Increment(n) => Shuffle::shuffle_with_increment(*n, cards),
            Shuffle::Cut(n) => {
                if *n > 0 {
                    for _ in 0..*n {
                        let cur = cards.remove(0);
                        cards.push(cur);
                    }
                } else {
                    for _ in 0..n.abs() {
                        let cur = cards.pop().unwrap();
                        cards.insert(0, cur);
                    }
                }
            }
        }
    }

    fn shuffle_with_increment(n: usize, cards: &mut Vec<u64>) {
        let mut new = vec![0; LEN];
        let mut index: usize = 0;

        while cards.len() > 0 {
            let cur = cards.remove(0);
            new[index] = cur;

            index = (index + n) % LEN;
        }

        cards.append(&mut new);
    }
}

fn parse() -> Vec<Shuffle> {
    let input = std::fs::read_to_string("../input/22").unwrap();
    let mut shuffle_instr = Vec::new();

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();

        shuffle_instr.push(match (values[0], values[1]) {
            ("deal", "into") => Shuffle::NewStack,
            ("deal", "with") => Shuffle::Increment(values[3].parse().unwrap()),
            ("cut", _) => Shuffle::Cut(values[1].parse().unwrap()),
            _ => panic!("invalid shuffle instr"),
        });
    }

    shuffle_instr
}

fn part_one(instr: &Vec<Shuffle>) -> usize {
    let mut cards = (0..LEN as u64).collect();

    for i in 0..instr.len() {
        instr[i].execute(&mut cards);
    }

    cards.iter().position(|n| *n == 2019).unwrap()
}

/*
fn part_two(instr: &Vec<Shuffle>) -> u64 {
    let mut cards = (0..PART_TWO as u64).collect::<Vec<u64>>();

    for i in 0..instr.len() {
        instr[i].execute(&mut cards);
    }

    cards[2020]
}
*/

fn main() {
    let instr = parse();

    println!("Part 1: {}", part_one(&instr));
}
