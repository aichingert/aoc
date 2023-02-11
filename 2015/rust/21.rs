// Advent of Code 2015, day 21
// (c) aichingert

const WEAPONS: [(i32,i32);5] = [(8,4),(10,5),(25,6),(40,7),(74,8)];
const ARMORS: [(i32,i32);6] = [(0,0),(13,1),(31,2),(53,3),(75,4),(102,5)];
const RINGS: [(i32,i32,i32);6] = [(25,1,0),(50,2,0),(100,3,0),(20,0,1),(40,0,2),(80,0,3)];

struct Entity {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Entity {
    fn new(hp:i32, damage:i32, armor:i32) -> Self {
        Self { hp, damage, armor }
    }
    
    fn fight(&self, other: &Self) -> bool {
        let player_hits: f32 = (other.hp as f32 / (self.damage as f32 - other.armor as f32).max(1.)).ceil();
        let boss_hits: f32 = (self.hp as f32 / (other.damage as f32 - self.armor as f32).max(1.)).ceil();

        player_hits <= boss_hits
    }
}

fn part1(player: &mut Entity, boss: &Entity) -> i32 {
    let mut ans = i32::MAX;

    for i in 0..WEAPONS.len() {
        for j in 0..ARMORS.len() {
            for k in 0..RINGS.len() {
                for l in 0..RINGS.len() {
                    player.damage = WEAPONS[i].1;
                    player.armor = ARMORS[j].1;
                    
                    let cost = if k != l {
                        player.damage += RINGS[k].1;
                        player.armor += RINGS[k].2;
                        RINGS[k].0
                    } else {
                        0
                    };
                    player.damage += RINGS[l].1;
                    player.armor += RINGS[l].2;
                    if player.fight(boss) {
                        ans = ans.min(cost + WEAPONS[i].0 + ARMORS[j].0 + RINGS[l].0);
                    }
                }
            }
        }
    }

    ans
}

fn part2() {}

fn parse() -> (Entity, Entity) {
    let inp = std::fs::read_to_string("../input/21").unwrap();
    let mut values: Vec<i32> = Vec::new();

    for l in inp.lines() {
        let vls: Vec<&str> = l.split(": ").collect();
        values.push(vls[1].parse::<i32>().unwrap());
    }

    (Entity::new(values[0], values[1], values[2]), Entity::new(100,0,0))
}

fn main() {
    let (boss, mut player) = parse();

    println!("Part 1: {}", part1(&mut player, &boss));
    //println!("Part 2: {}", part2());
}
