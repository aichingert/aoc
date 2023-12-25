struct Unit {
    health: i16,
    weaknesses: u16,
    immunities: u16,
}

impl Unit {
    fn new(health: i16, weaknesses: u16, immunities: u16) -> Self {
        Self {
            health,
            weaknesses,
            immunities,
        }
    }
}

struct Group {
    attack: i16,
    attack_type: u16,

    initiative: u16,
    units: Vec<Unit>,
}

impl Group {
    fn new(attack: i16, attack_type: u16, initiative: u16, units: Vec<Unit>) -> Self {
        Self {
            attack,
            attack_type,
            initiative,
            units,
        }
    }

    // 18 units each with 729 hit points (weak to fire; immune to cold, slashing)
    // with an attack that does 8 radiation damage at initiative 10
    fn parse(s: &str) -> Self {
        println!("{s}");
        let (start, middle) = s.split_once(" (").unwrap();

        let start = start.split(' ').collect::<Vec<_>>();
        let (amount, hp) = (start[0].parse::<usize>().unwrap(), start[4].parse::<i16>().unwrap());

        let (effects, end) = middle.split_once(") ").unwrap();

        let (a, b) = effects.split_once("; ").unwrap();

        let (mut weaknesses, mut immunities) = (0, 0);

        if a.starts_with("weak to") {
            let elements = a[7..].split(", ").collect::<Vec<_>>();
            println!("{:?}", elements);
        } else {

        }


        Self::new(0, 0, 0, vec![])
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim().to_string();
    
    let (immune, infection) = inp.split_once("\n\n").unwrap();

    let immune_groups = immune.lines().skip(1).map(Group::parse).collect::<Vec<_>>();
}
