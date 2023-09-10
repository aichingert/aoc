use std::collections::{HashSet, BTreeMap};

type P = (i8, i8);

fn part_one(inp: &Vec<P>) -> u32 {
    let mut layouts = HashSet::new();
    let mut state = inp.clone();
    println!("{:?}", state);

    let next_to = |p: &P| -> Vec<P> {
        [(1,0),(-1,0),(0,1),(0,-1)].iter()
            .map(|dir| ((dir.0 + p.0, dir.1 + p.1)))
            .filter(|(y, x)| *x > -1 && *x < 5 && *y > -1 && *y < 5)
            .collect::<Vec<P>>()
    };

    while layouts.insert(state.clone()) {
        let mut next = Vec::new();
        let mut empty_counts = BTreeMap::new();

        for p in state.iter() {
            let c = next_to(p).iter().map(|n| 
                if state.contains(n) { 1 } 
                else { empty_counts.entry(*n).and_modify(|count| *count += 1).or_insert(1); 0 }
            ).sum::<u8>();

            if c == 1 {
                next.push(*p);
            }
        }

        for (p, c) in empty_counts.into_iter() {
            if c == 1 || c == 2 {
                next.push(p);
            }
        }

        state = next;
    }

    state.iter().map(|p| 2_u32.pow((p.0 * 5 + p.1) as u32)).sum::<u32>()
}

fn main() {
    let mut inp = Vec::new();

    for (i, e) in std::fs::read_to_string("../input/24").unwrap().trim().lines().enumerate() {
        for (j, ch) in e.chars().enumerate() {
            if ch == '#' {
                inp.push((i as i8, j as i8));
            }
        }
    }

    println!("{:?}", part_one(&inp));
}
