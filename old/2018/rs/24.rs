use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq)]
struct Group {
    hp: i32,
    units: i32,
    attack: i32,
    attack_type: String,

    initiative: u32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.effective_power().cmp(&self.effective_power()).then(other.initiative.cmp(&self.initiative)))
    }
}

impl Group {
    fn new(attack: i32, attack_type: String, weaknesses: HashSet<String>, immunities: HashSet<String>, initiative: u32, hp: i32, units: i32) -> Self {
        Self {
            attack,
            attack_type,
            weaknesses,
            immunities,
            initiative,
            hp,
            units,
        }
    }

    fn parse(s: &str) -> Self {
        if let Some((start, middle)) = s.split_once(" (") {
            let start = start.split(' ').collect::<Vec<_>>();
            let (amount, hp) = (start[0].parse::<i32>().unwrap(), start[4].parse::<i32>().unwrap());
            let (effects, end) = middle.split_once(") ").unwrap();
            let (mut weaknesses, mut immunities) = (HashSet::new(), HashSet::new());

            let (weak, immu) = if let Some((a, b)) = effects.split_once("; ") {
                let (a, b) = if a.starts_with("weak to") { (a, b) } else { (b, a) };
                (Some(a[8..].split(", ").collect::<Vec<_>>()), Some(b[10..].split(", ").collect::<Vec<_>>()))
            } else {
                if effects.starts_with("weak to") {
                    (Some(effects[8..].split(", ").collect::<Vec<_>>()), None)
                } else {
                    (None, Some(effects[10..].split(", ").collect::<Vec<_>>()))
                }
            };

            if let Some(weak) = weak { weak.into_iter().for_each(|w| { weaknesses.insert(w.to_string()); }); }
            if let Some(immu) = immu { immu.into_iter().for_each(|i| { immunities.insert(i.to_string()); }); }

            let vals = end.split(' ').collect::<Vec<_>>();
            let (dmg, a_type, init) = (vals[5].parse().unwrap(), vals[6].to_string(), vals[10].parse().unwrap());
            Self::new(dmg, a_type, weaknesses, immunities, init, hp, amount)
        } else {
            let vals = s.split(' ').collect::<Vec<_>>();

            Self::new(
                vals[12].parse().unwrap(), 
                vals[13].to_string(), 
                HashSet::new(),
                HashSet::new(),
                vals[17].parse().unwrap(),
                vals[4].parse().unwrap(),
                vals[0].parse().unwrap()
            )
        }
    }

    fn effective_power(&self) -> i32 {
        self.units * self.attack
    }

    fn damage_to(&self, other: &Self) -> i32 {
        if other.immunities.contains(&self.attack_type) {
            return 0;
        } 

        self.effective_power() * if other.weaknesses.contains(&self.attack_type) { 2 } else { 1 }
    }

    fn chose(groups: &Vec<Self>, other: &Vec<Self>) -> Vec<i16> {
        let mut taken = 0usize;

        groups.iter()
            .map(|group| {
                let (mut pos, mut dmg, mut eff, mut ini) = (-1, 1, -1, 0);

                for j in 0..other.len() {
                    if taken & (1 << j) > 0 {
                        continue;
                    }

                    let odmg = group.damage_to(&other[j]);
                    let oeff = other[j].effective_power();
                    let oini = other[j].initiative;

                    if odmg.cmp(&dmg).then(oeff.cmp(&eff).then(oini.cmp(&ini))) == Ordering::Greater {
                        (pos, dmg, eff, ini) = ((j as i16), odmg, oeff, oini)
                    }
                }

                if pos != -1 { taken |= 1 << pos; }

                pos
            })
        .collect()
    }

    fn get_order(groups_imm: &Vec<Self>, groups_inf: &Vec<Self>) -> Vec<(usize, bool, u32)> {
        let mut order = groups_imm.iter().enumerate().map(|(i, g)| (i, true, g.initiative)).collect::<Vec<_>>();
        order.extend_from_slice(&groups_inf.iter().enumerate().map(|(i, g)| (i, false, g.initiative)).collect::<Vec<_>>());
        order.sort_by(|a,b| b.2.cmp(&a.2));
        order
    }
}

fn part_one(mut immune_groups: Vec<Group>, mut infection_groups: Vec<Group>) -> i32 {
    while immune_groups.len() > 0 && infection_groups.len() > 0 {
        immune_groups.sort();
        infection_groups.sort();
        let imtargets = Group::chose(&immune_groups, &infection_groups);
        let intargets = Group::chose(&infection_groups, &immune_groups);

        for (pos, is_immune, _) in Group::get_order(&immune_groups, &infection_groups) {
            let (group, other) = if is_immune {
                if immune_groups[pos].units <= 0 || imtargets[pos] == - 1 { continue; }
                (&mut infection_groups[imtargets[pos] as usize], &immune_groups[pos])
            } else {
                if infection_groups[pos].units <= 0 || intargets[pos] == - 1 { continue; }
                (&mut immune_groups[intargets[pos] as usize], &infection_groups[pos])
            };

            group.units -= other.damage_to(group) / group.hp;
        }

        immune_groups = immune_groups.into_iter().filter(|g| g.units > 0).collect::<Vec<_>>();
        infection_groups = infection_groups.into_iter().filter(|g| g.units > 0).collect::<Vec<_>>();
    }

    infection_groups.iter().chain(immune_groups.iter()).map(|g| g.units).sum::<i32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim().to_string();
    let (immune, infection) = inp.split_once("\n\n").unwrap();

    let immune_groups = immune.lines().skip(1).map(Group::parse).collect::<Vec<_>>();
    let infection_groups = infection.lines().skip(1).map(Group::parse).collect::<Vec<_>>();

    
    println!("Part one: {}", part_one(immune_groups.clone(), infection_groups.clone()));

}
