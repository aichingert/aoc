use std::collections::{VecDeque, HashMap, HashSet};

type Pos = (usize, usize);
type HM= HashMap<Pos, char>;

fn get_reachable_keys(start: Pos, k: &HM, d: &HM, w: &HashSet<Pos>, i: &HashSet<char>) -> Vec<Pos> {
    let mut bfs = VecDeque::from([start]);
    let mut visited = HashSet::new();
    let mut reachable_keys = Vec::new();

    while let Some(next) = bfs.pop_front() {
        if w.contains(&next) || (d.contains_key(&next) && !i.contains(d.get(&next).unwrap())) {
            continue;
        }
        visited.insert(next);

        if k.contains_key(&next) && !i.contains(&k.get(&next).unwrap().to_ascii_uppercase()) { 
            reachable_keys.push(next); 
            continue; 
        }

        for vec in [(0,1),(0,-1),(1,0),(-1,0)] {
            let loc = ((next.0 as i32 + vec.0) as usize, (next.1 as i32 + vec.1) as usize);

            if !visited.contains(&loc) {
                bfs.push_back(loc);
            }
        }
    }

    reachable_keys
}

fn part_one(start: Pos, l: u32, inv: &HashSet<char>, k: &HM, d: &HM, w: &HashSet<Pos>) -> u32 {
    println!("{:?}", inv);
    if inv.len() == k.len() {
        return l;
    }

    let mut i = inv.clone();
    let mut ans = u32::MAX;

    for keys in get_reachable_keys(start, k, d, w, &inv) {
        let ch = k.get(&keys).unwrap().to_ascii_uppercase();

        i.insert(ch);
        ans = ans.min(part_one(keys, l + 1, &i, k, d, w));
        i.remove(&ch);
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

    println!("{}", part_one(start, 0, &HashSet::new(), &keys, &doors, &walls));
    //println!("{:?}", get_reachable_keys(start, &keys, &doors, &walls, &HashSet::from(['A'])));

}
