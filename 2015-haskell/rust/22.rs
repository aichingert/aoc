// Advent of Code 2015, day 22
// (c) aichingert

const SPELLS: [Spell; 5] = [
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
    Spell::Recharge,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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

    fn consume(&self, player: &mut Player, boss: &mut Boss) {
        match self {
            Spell::MagicMissile => {
                boss.hp -= 4;
            }
            Spell::Drain => {
                player.hp += 2;
                boss.hp -= 2;
            }
            Spell::Shield => {
                player.armor = 7;
            }
            Spell::Poison => {
                boss.hp -= 3;
            }
            Spell::Recharge => {
                player.mana += 101;
            }
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
        Self {
            timer: spell.duration(),
            spell,
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
    armor: i32,
}

fn update(player: &mut Player, boss: &mut Boss, effects: &mut Vec<Effect>) {
    let mut i = 0;

    while i < effects.len() {
        effects[i].spell.consume(player, boss);

        if effects[i].timer == 1 {
            let effect = effects.remove(i);

            if effect.spell == Spell::Shield {
                player.armor = 0;
            }
        } else {
            effects[i].timer -= 1;
            i += 1;
        }
    }
}

fn contains_spell(effects: &Vec<Effect>, spell: &Spell) -> bool {
    for i in 0..effects.len() {
        if effects[i].spell == *spell {
            return true;
        }
    }
    false
}

fn solve(
    player: &Player,
    boss: &Boss,
    effects: &Vec<Effect>,
    cur: i32,
    hard_mode: bool,
) -> Option<i32> {
    let mut wins: Vec<i32> = Vec::new();

    for spell in SPELLS.iter() {
        let mut sim_player = player.clone();
        let mut sim_boss = boss.clone();
        let mut sim_effects = effects.clone();

        if hard_mode {
            sim_player.hp -= 1;
            if sim_player.hp <= 0 {
                return None;
            }
        }

        update(&mut sim_player, &mut sim_boss, &mut sim_effects);

        if sim_boss.hp <= 0 {
            return Some(cur);
        }
        if sim_player.mana - spell.cost() < 0 || contains_spell(&sim_effects, spell) {
            continue;
        }

        sim_player.mana -= spell.cost();
        sim_effects.push(Effect::new(*spell));

        update(&mut sim_player, &mut sim_boss, &mut sim_effects);

        if sim_boss.hp <= 0 {
            wins.push(cur + spell.cost());
            continue;
        }

        sim_player.hp -= (sim_boss.damage - sim_player.armor).max(1);

        if sim_player.hp <= 0 {
            continue;
        }

        if let Some(mana) = solve(
            &sim_player,
            &sim_boss,
            &sim_effects,
            cur + spell.cost(),
            hard_mode,
        ) {
            wins.push(mana);
        }
    }

    wins.into_iter().min()
}

fn parse() -> (Player, Boss) {
    let inp = std::fs::read_to_string("../input/22").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();
    let hp = inp[0].split(": ").collect::<Vec<&str>>();
    let dmg = inp[1].split(": ").collect::<Vec<&str>>();

    (
        Player {
            hp: 50,
            mana: 500,
            armor: 0,
        },
        Boss {
            hp: hp[1].parse::<i32>().unwrap(),
            damage: dmg[1].parse::<i32>().unwrap(),
        },
    )
}

fn main() {
    let (player, boss) = parse();
    //let (p, b): (Player, Boss) = (Player { hp: 10, mana: 250, armor: 0 }, Boss { hp: 14, damage: 8 });

    println!("Part 1: {:?}", solve(&player, &boss, &vec![], 0, false));
    println!("Part 2: {:?}", solve(&player, &boss, &vec![], 0, true));
}
