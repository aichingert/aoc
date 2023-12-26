use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
struct Group {
    attack: i32,
    attack_type: String,

    weaknesses: HashSet<String>,
    immunities: HashSet<String>,

    initiative: u32,

    hp: i32,
    units: i32,
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.effective_power().cmp(&other.effective_power()) {
            Ordering::Equal => {
                self.initiative.cmp(&other.initiative)
            },
            ord => ord,
        })
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

    // 18 units each with 729 hit points (weak to fire; immune to cold, slashing)
    // with an attack that does 8 radiation damage at initiative 10
    fn parse(s: &str) -> Self {
        println!("{:?}", s);
        if let Some((start, middle)) = s.split_once(" (") {
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
        } else {
            let vals = s.split(' ').collect::<Vec<_>>();
            Self::new(vals[12].parse().unwrap(), vals[13].to_string(), HashSet::new(), HashSet::new(), vals[17].parse().unwrap(), vals[4].parse().unwrap(), vals[0].parse().unwrap())
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

    fn chose(group: &Vec<Self>, other: &Vec<Self>) -> Vec<i16> {
        let mut taken = Vec::new();

        for i in 0..group.len() {
            let mut pos = -1;
            let mut dmg = 0;
            let mut eff = 0;
            let mut ini = 0;

            for j in 0..other.len() {
                match group[i].damage_to(&other[j]).cmp(&dmg) {
                    Ordering::Equal => {
                        match other[j].effective_power().cmp(&eff) {
                            Ordering::Equal => {
                                if !taken.contains(&(j as i16)) && other[j].initiative > ini {
                                    ini = other[j].initiative;
                                    pos = j as i16;
                                }
                            }
                            Ordering::Greater => {
                                if !taken.contains(&(j as i16)) {
                                    eff = other[j].effective_power();
                                    ini = other[j].initiative;
                                    pos = j as i16;
                                }
                            }
                            _ => (),
                        }
                    }
                    Ordering::Greater => {
                        if !taken.contains(&(j as i16)) {
                            dmg = group[i].damage_to(&other[j]);
                            eff = other[j].effective_power();
                            ini = other[j].initiative;
                            pos = j as i16;
                        }
                    }
                    _ => (),
                }
            }

            taken.push(pos);
        }

        taken
    }

    fn get_order(group: &Vec<Self>, g: bool, order: &mut Vec<(usize, bool, u32)>) {
        'lp: for i in 0..group.len() {
            let mut j = 0;

            while j < order.len() {
                if order[j].2 < group[i].initiative {
                    order.insert(j, (i, g, group[i].initiative));
                    continue 'lp;
                }

                j += 1;
            }

            order.push((i, g, group[i].initiative));
        }
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim().to_string();
    
    let (immune, infection) = inp.split_once("\n\n").unwrap();

    let mut immune_groups = immune.lines().skip(1).map(Group::parse).collect::<Vec<_>>();
    let mut infection_groups = infection.lines().skip(1).map(Group::parse).collect::<Vec<_>>();

    

    while immune_groups.len() > 0 && infection_groups.len() > 0 {
        let imtargets = Group::chose(&immune_groups, &infection_groups);
        let intargets = Group::chose(&infection_groups, &immune_groups);

        let mut order = Vec::new();
        Group::get_order(&immune_groups, true, &mut order);
        Group::get_order(&infection_groups, false, &mut order);

        for (pos, is_immune, _) in order {
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

        immune_groups.sort_by(|a,b| b.cmp(&a));
        infection_groups.sort_by(|a,b| b.cmp(&a));
    }

    let ans = infection_groups.iter().chain(immune_groups.iter()).map(|g| g.units).sum::<i32>();

    println!("{:?}", ans);

}
