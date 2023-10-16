use std::collections::{VecDeque, HashMap, HashSet};

type Pos = (usize, usize);
type HM= HashMap<Pos, char>;

#[derive(Debug, Clone)]
struct State {
    reachable_keys: Vec<Pos>,
    inventory: HashSet<char>,
    visited: HashSet<Pos>,
    position: Pos,
}

impl State {
    fn new(start: Pos) -> Self {
        Self {
            reachable_keys: Vec::new(),
            inventory: HashSet::new(),
            visited: HashSet::new(),
            position: start,
        }
    }

    fn get_reachable_keys(&mut self, keys: &HM, doors: &HM, walls: &HashSet<Pos>) {
        let mut bfs = VecDeque::from([self.position]);

        while let Some(next) = bfs.pop_front() {
            if walls.contains(&next) || (doors.contains_key(&next) && !self.inventory.contains(doors.get(&next).unwrap())) {
                continue;
            }
            self.visited.insert(next);

            if keys.contains_key(&next) && !self.inventory.contains(&keys.get(&next).unwrap().to_ascii_uppercase()) { 
                self.reachable_keys.push(next); 
                continue; 
            }

            for vec in [(0,1),(0,-1),(1,0),(-1,0)] {
                let loc = ((next.0 as i32 + vec.0) as usize, (next.1 as i32 + vec.1) as usize);

                if !self.visited.contains(&loc) {
                    bfs.push_back(loc);
                }
            }
        }

    }
}

fn part_one(state: State, dist: u32, keys: &HM, doors: &HM, rd: &HashMap<char, Pos>, walls: &HashSet<Pos>) -> u32 {
    if state.inventory.len() == keys.len() {
        return dist;
    }

    let mut ans = u32::MAX;
    let mut state = state;
    let mut idx = 0usize;

    state.get_reachable_keys(keys, doors, walls);
    println!("{dist}: {:?}", state);

    while idx < state.reachable_keys.len() {
        let loc = state.reachable_keys.remove(idx);
        let key = keys.get(&loc).unwrap().to_ascii_uppercase();

        state.inventory.insert(key);
        if let Some(pos) = rd.get(&key) {
            state.position = *pos;
        }

        ans = ans.min(part_one(state.clone(), dist + 1, keys, doors, rd, walls));

        state.inventory.remove(&key);
        state.reachable_keys.insert(idx, loc);
        idx += 1;
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap().trim().to_string();
    let mut walls: HashSet<Pos> = HashSet::new();
    let mut doors: HashMap<Pos, char> = HashMap::new();
    let mut keys:  HashMap<Pos, char> = HashMap::new();
    let mut start: Pos = (0, 0);

    for (i, l) in inp.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '.' => (),
                '#' => { walls.insert((i, j)); },
                '@' => start = (i, j),
                c => if c.is_ascii_uppercase() {
                    doors.insert((i, j), c);
                } else {
                    keys.insert((i, j), c);
                }
            };
        }
    }

    let mut rev_doors: HashMap<char, Pos> = HashMap::new();

    for (k, v) in doors.iter() {
        rev_doors.insert(*v, *k);
    }

    println!("{}", part_one(State::new(start), 0, &keys, &doors, &rev_doors, &walls));
}
