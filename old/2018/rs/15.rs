// Advent of Code 2018, day 15
// (c) aichingert

const HP: i32 = 200;
const DMG: i32 = 3;

#[derive(Debug)]
struct Entity {
    is_elf: bool,
    hp: i32,
    dmg: i32,
    loc: (usize,usize),
}

impl Entity {
    fn new(is_elf: bool, loc: (usize,usize)) -> Self {
        Self { is_elf, hp: HP, dmg: DMG, loc }
    }
}

fn parse() -> (Vec<Entity>, Vec<Vec<char>>) {
    let mut entitys = Vec::<Entity>::new();
    let map = std::fs::read_to_string("../input/15").unwrap();

    for (i,l) in map.lines().enumerate() {
        for (j,c) in l.chars().enumerate() {
            match c {
                'G' | 'E' => entitys.push(Entity::new(c == 'E', (i,j))),
                _ => (),
            }
        }
    }
        
    (entitys, map.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>())
}

fn main() {
    let (mut entitys, map) = parse();
}
