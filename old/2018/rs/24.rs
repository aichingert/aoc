use std::collections::HashSet;

#[derive(Debug)]
struct Group {
    attack: i32,
    attack_type: String,

    weaknesses: HashSet<String>,
    immunities: HashSet<String>,

    initiative: u32,

    hp: i32,
    units: i32,
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

    // 18 units each with 729 hit points (weak to fire; immune to cold, slashing)
    // with an attack that does 8 radiation damage at initiative 10
    fn parse(s: &str) -> Self {
        let (start, middle) = s.split_once(" (").unwrap();
        let start = start.split(' ').collect::<Vec<_>>();
        let (amount, hp) = (start[0].parse::<i32>().unwrap(), start[4].parse::<i32>().unwrap());
        let (effects, end) = middle.split_once(") ").unwrap();
        let (mut weaknesses, mut immunities) = (HashSet::new(), HashSet::new());

        let (weak, immu) = if let Some((a, b)) = effects.split_once("; ") {
            if a.starts_with("weak to") {
                (Some(a[8..].split(", ").collect::<Vec<_>>()), Some(b[10..].split(", ").collect::<Vec<_>>()))
            } else {
                (Some(b[8..].split(", ").collect::<Vec<_>>()), Some(a[10..].split(", ").collect::<Vec<_>>()))
            }
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
        let (dmg, attack_type, initiative) = (vals[5].parse::<i32>().unwrap(), vals[6].to_string(), vals[10].parse::<u32>().unwrap());

        Self::new(dmg, attack_type, weaknesses, immunities, initiative, hp, amount)
    }

    fn effective_power(&self) -> i32 {
        self.units * self.attack
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim().to_string();
    
    let (immune, infection) = inp.split_once("\n\n").unwrap();

    let mut immune_groups = immune.lines().skip(1).map(Group::parse).collect::<Vec<_>>();
    let mut infection_groups = infection.lines().skip(1).map(Group::parse).collect::<Vec<_>>();

    for i in 0..immune_groups.len() {
        println!("{}", immune_groups[i].effective_power());
    }

    immune_groups.sort_by(|a, b| a.effective_power().cmp(&b.effective_power()));

    while immune_groups.len() > 0 && infection_groups.len() > 0 {

    }


}
