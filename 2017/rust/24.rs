// Advent of Code 2017, day 24
// (c) aichingert

#[derive(Clone)]
struct Component {
    ports: [u32;2],
    used: [bool;2],
}

#[derive(Clone)]
struct Bridge {
    connections: Vec<Component>,
}

impl Component {
    fn new(ports: [u32;2]) -> Component {
        Component { ports, used: [false;2] }
    }
}

impl Bridge {
    fn new() -> Bridge {
        Bridge { connections: Vec::new() }
    }

    fn add(&mut self, component: Component) -> bool {
        let len = self.connections.len();

        match len {
            0 => self.connections.push(component),
            _ => 'main: for i in 0..2 {
                for j in 0..2 {
                    if !self.connections[len-1].used[i] && self.connections[len-1].ports[i] == component.ports[j] {
                        self.connections[len-1].used[i] = true;
                        self.connections.push(component);
                        self.connections[len].used[j] = true;
                        break 'main;
                    }
                } 
            },
        }

        self.connections.len() != len
    }

    fn strength(&self) -> u32 {
        self.connections.iter().map(|comp| comp.ports[0] + comp.ports[1]).sum::<u32>()
    }
}

fn solve(comps: &mut Vec<Component>) -> (u32,u32) {
    let mut possible = Vec::<Bridge>::new();
    let mut start = Bridge::new();
    let mut solve = (0,0,0); // part1, part2, len

    start.add(Component::new([0,0]));
    rec_find(comps, &mut start, &mut possible);
    
    for pos in possible {
        solve.0 = solve.0.max(pos.strength());
        if pos.connections.len() > solve.2 {
            solve.1 = pos.strength();
            solve.2 = pos.connections.len();
        } else if pos.connections.len() == solve.2 {
            solve.1 = solve.1.max(pos.strength());
        }
    }

    (solve.0,solve.1)
}

fn rec_find(comps: &mut Vec<Component>, cur: &mut Bridge, bridges: &mut Vec<Bridge>) {
    bridges.push(cur.clone());

    let mut i: usize = 0;

    while i < comps.len() {
        let mut clone = cur.clone();
        if clone.add(comps[i].clone()) {
            let comp = comps.remove(i);
            rec_find(&mut comps.clone(), &mut clone, bridges);
            comps.insert(i, comp);
        }

        i += 1;
    }
}

fn main() {
    let mut inp = std::fs::read_to_string("../input/24").unwrap()
        .lines()
        .map(|l| {let p = l.split_once('/').unwrap(); Component::new([p.0.parse::<u32>().unwrap(),p.1.parse::<u32>().unwrap()])})
        .collect::<Vec<Component>>();
    let (part1,part2) = solve(&mut inp); 

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
