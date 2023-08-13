use crate::day::{
    d15::{Coordinate, Unit},
    Input, InputResult, Output, Wrapper,
};
use std::collections::{HashMap, HashSet};

type Map = HashMap<Coordinate, char>;
const UPPER_BOUND: u32 = u16::MAX as u32;

fn get_open_tiles_next_to_enemies(kind: char, map: &Map, units: &Vec<Unit>) -> Vec<Coordinate> {
    units
        .iter()
        .filter(|u| u.kind != kind)
        .flat_map(|u| u.pos.adjacent())
        .filter(|p| is_tile_open(map, units, p))
        .collect::<Vec<Coordinate>>()
}

fn get_distances(point: Coordinate, map: &Map, units: &Vec<Unit>) -> HashMap<Coordinate, u32> {
    let mut dp = HashMap::from([(point, 0)]);
    let mut q: HashSet<Coordinate> = HashSet::from([point]);
    let mut p = vec![point];

    while !p.is_empty() {
        let idx = {
            let mut best = u32::MAX;
            let mut found = None;

            for i in 0..p.len() {
                let cur = *dp.entry(p[i]).or_insert(UPPER_BOUND);

                if cur + 1 < best {
                    best = cur;
                    found = Some(i)
                }
            }

            found.unwrap()
        };

        let current = p.remove(idx);

        for adjacent in current.adjacent().into_iter() {
            if q.contains(&adjacent) || !is_tile_open(map, units, &adjacent) {
                continue;
            }

            let cur = *dp.get(&current).unwrap();
            let nxt = dp.entry(adjacent).or_insert(UPPER_BOUND);

            if *nxt > cur + 1 {
                *nxt = cur + 1;
            }

            p.push(adjacent);
            q.insert(adjacent);
        }
    }

    dp
}

fn is_tile_open(map: &Map, units: &Vec<Unit>, pos: &Coordinate) -> bool {
    if map.get(pos).is_some_and(|ch| *ch == '#') {
        return false;
    }

    units.iter().find(|u| u.pos == *pos).is_none()
}

fn is_next_to_enemie(units: &Vec<Unit>, unit: &Unit) -> Option<usize> {
    let mut enemies = unit
        .pos
        .adjacent()
        .into_iter()
        .filter_map(|p| {
            units
                .iter()
                .enumerate()
                .find(|(_, u)| u.pos == p && u.kind != unit.kind)
        })
        .collect::<Vec<(usize, &Unit)>>();
    enemies.sort_by(|a, b| a.1.hp.cmp(&b.1.hp).then(a.1.pos.cmp(&b.1.pos)));

    if let Some((index, _)) = enemies.first() {
        return Some(*index);
    }

    None
}

fn round(map: &Map, units: &mut Vec<Unit>) -> bool {
    units.sort_by_key(|u| u.pos);

    let mut i: usize = 0;
    while i < units.len() {
        let unit: Unit = units[i];

        if units.iter().find(|u| unit.kind != u.kind).is_none() {
            return false;
        }

        if let Some(index) = is_next_to_enemie(units, &unit) {
            units[index].hp -= unit.dmg;

            if units[index].hp <= 0 {
                units.remove(index);

                if index > i {
                    i += 1;
                }
                continue;
            }

            i += 1;
            continue;
        }

        let dp = get_distances(unit.pos, map, units);

        let tile = get_open_tiles_next_to_enemies(unit.kind, map, units)
            .into_iter()
            .filter(|p| dp.get(p).is_some() && is_tile_open(map, units, p))
            .map(|p| (dp.get(&p).unwrap(), p))
            .min();

        if let Some((_, pos)) = tile {
            let enemy_dp = get_distances(pos, map, units);
            let (_, new_pos) = unit
                .pos
                .adjacent()
                .into_iter()
                .filter(|p| enemy_dp.get(p).is_some() && is_tile_open(map, units, &p))
                .map(|p| (enemy_dp.get(&p).unwrap(), p))
                .min()
                .unwrap();

            units[i].pos = new_pos;
        }

        let unit = units[i];

        if let Some(index) = is_next_to_enemie(units, &unit) {
            units[index].hp -= unit.dmg;

            if units[index].hp <= 0 {
                units.remove(index);

                if index < i {
                    continue;
                }
            }
        }

        i += 1;
    }

    true
}

pub fn run(input: Input) -> (Output, Output) {
    let (units, map): (Vec<Unit>, HashMap<Coordinate, char>) = input.unwrap();
    let (mut part_one, mut part_two): (Output, Output) = (Output::None, Output::None);
    let elves = units
        .iter()
        .map(|u| if u.kind == 'E' { 1 } else { 0 })
        .sum::<u16>();

    for dmg in 3.. {
        let mut battle_units = units.clone();
        let mut rounds: u32 = 0;

        for unit in &mut battle_units {
            if unit.kind == 'E' {
                unit.dmg = dmg;
            }
        }

        while round(&map, &mut battle_units) {
            rounds += 1;
        }

        let remaining = battle_units
            .iter()
            .map(|u| if u.kind == 'E' { 1 } else { 0 })
            .sum::<u16>();

        if dmg == 3 {
            part_one = Output::Nu32(rounds * battle_units.iter().map(|u| u.hp as u32).sum::<u32>());
        } else if remaining == elves {
            part_two = Output::Nu32(rounds * battle_units.iter().map(|u| u.hp as u32).sum::<u32>());
            break;
        }
    }

    (part_one, part_two)
}

pub fn parse() -> InputResult<Input> {
    let mut map: HashMap<Coordinate, char> = HashMap::new();
    let mut units: Vec<Unit> = Vec::new();

    for (y, line) in std::fs::read_to_string("../input/15")?.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let point: Coordinate = Coordinate { y, x };

            match ch {
                '#' | '.' => map.insert(point, ch),
                'E' | 'G' => {
                    units.push(Unit {
                        hp: 200,
                        pos: point,
                        dmg: 3,
                        kind: ch,
                    });
                    map.insert(point, '.')
                }
                _ => return Err(crate::day::InputError::InvalidInput),
            };
        }
    }

    Ok(Input::D15((units, map)))
}
