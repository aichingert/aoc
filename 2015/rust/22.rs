// Advent of Code 2015, day 22
// (c) aichingert

#[derive(Debug, Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn duration(&self) -> i32 {
        match self {
            Spell::MagicMissile => 1,
            Spell::Drain => 1,
            Spell::Shield => 6,
            Spell::Poison => 6,
            Spell::Recharge => 5,
        }
    }
}

#[derive(Debug, Clone)]
struct Effect {
    timer: i32,
    spell: Spell,
}

impl Effect {
    fn new(spell: Spell) -> Self {
        Self { timer: spell.duration(), spell }
    }
}

#[derive(Debug, Clone)]
struct ActiveEffects {
    effects: Vec<Effect>,
}

impl ActiveEffects {
    fn new() -> Self {
        Self { effects: Vec::new() }
    }

    fn update(&mut self, player: &mut Player, boss: &mut Boss) {
        player.mana -= self.effects[0].spell.cost();

        for effect in self.effects.iter() {
            match effect.spell {
                Spell::MagicMissile => boss.hp -= 4,
                Spell::Drain => {
                    player.hp += 2;
                    boss.hp -= 2;
                },
                Spell::Shield => player.shield = 7,
                Spell::Poison => boss.hp -= 3,
                Spell::Recharge => player.mana += 101,
            };
        }

        let mut i: usize = 0;
        while i < self.effects.len() {
            if self.effects[i].timer == 1 {
                self.effects.remove(i);
            } else {
                self.effects[i].timer -= 1;
                i += 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    hp: i32,
    mana: i32,
    shield: i32,
}

const SPELLS: [Spell;5] = [Spell::MagicMissile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge];

fn part1(player: &mut Player, boss: &mut Boss) -> i32 {
    let mut amt: i32 = i32::MAX;
    let mut effects = ActiveEffects::new();

    for spell in SPELLS {
        effects.effects.push(Effect::new(spell));
        if let Some(mana) = simulation(&mut player.clone(), &mut boss.clone(), &mut effects) {
            amt = amt.min(mana);
        }
        println!("{:?}, {:?}, {:?}", spell, amt, effects.effects);
        effects.effects.clear();
    }

    amt
}

fn simulation(player: &mut Player, boss: &mut Boss, real: &mut ActiveEffects) -> Option<i32> {
    let mut m = 0;
    real.update(player, boss);

    if boss.hp <= 0 && player.mana >= 0 {
        return Some(player.mana);
    } else if player.mana <= 0 || player.hp <= 0 {
        return None;
    } 
    
    for spell in SPELLS {
        let mut fake_p = player.clone();
        let mut fake = real.clone();
        fake_p.hp -= (boss.damage - player.shield).max(1);

        fake.effects.push(Effect::new(spell));

        if let Some(mana) = simulation(&mut fake_p, &mut boss.clone(), &mut fake) {
            m = m.max(mana);
        }
    }


    Some(m)
}

fn part2() {}

fn parse() -> (Player, Boss) {
    let inp = std::fs::read_to_string("../input/22").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();
    let hp = inp[0].split(": ").collect::<Vec<&str>>();
    let dmg = inp[1].split(": ").collect::<Vec<&str>>();

    (Player { hp: 50, mana: 500, shield: 0 }, Boss { hp: hp[1].parse::<i32>().unwrap(), damage: dmg[1].parse::<i32>().unwrap() })
}

fn main() {
    let (mut player, mut boss) = parse();
    let (mut p, mut b): (Player, Boss) = (Player { hp: 10, shield: 0, mana: 250 }, Boss { hp: 13, damage: 8 });

    println!("Part 1: {:?}", part1(&mut p, &mut b));
    //println!("Part 2: {}", part2());
}
