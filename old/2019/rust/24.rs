use std::collections::{HashSet, BTreeMap};

type P = (i8, i8);

fn part_one(inp: &Vec<P>) -> u32 {
    let mut layouts = HashSet::new();
    let mut state = inp.clone();

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

fn part_two(inp: &Vec<P>) -> usize {
    let mut state = HashSet::new();
    inp.iter().for_each(|p| { state.insert((0i16, *p)); });
    state.remove(&(0, (2,2)));

    let next_to = |cord: (i16, P)| -> Vec<(i16, P)> {
        let mut adjacent = Vec::new();
        let (dimension, p) = cord;

        for dir in [(1,0),(-1,0),(0,1),(0,-1)].iter() {
            let x = dir.1 + p.1;
            let y = dir.0 + p.0;

            let (nd, nested) = match (y, x) {
                (2, 2) => (dimension + 1, match p {
                    /*   01234
                     * 0 .....
                     * 1 .....
                     * 2 ..?..
                     * 3 .....
                     * 4 ..... */
                    (1,2) => vec![(0,0),(0,1),(0,2),(0,3),(0,4)],
                    (2,1) => vec![(0,0),(1,0),(2,0),(3,0),(4,0)],
                    (3,2) => vec![(4,0),(4,1),(4,2),(4,3),(4,4)],
                    (2,3) => vec![(0,4),(1,4),(2,4),(3,4),(4,4)],
                    _ => panic!("my logic is wrong..."),
                }),
                (5, _) => (dimension - 1, vec![(3,2)]),
                (_, 5) => (dimension - 1, vec![(2,3)]),
                (-1,_) => (dimension - 1, vec![(1,2)]),
                (_,-1) => (dimension - 1, vec![(2,1)]),
                _ => (dimension, vec![(y,x)]),
            };

            for inner in nested.into_iter() {
                adjacent.push((nd, inner));
            }
        }

        adjacent
    };

    for _ in 0..200 {
        let mut next = HashSet::new();
        let mut empty_counts = BTreeMap::new();

        for cord in state.iter() {
            let c = next_to(*cord).into_iter().map(|(d, p)| 
                if state.contains(&(d, p)) { 1 } 
                else { empty_counts.entry((d, p)).and_modify(|count| *count += 1).or_insert(1); 0 }
            ).sum::<u8>();

            if c == 1 {
                next.insert(*cord);
            }
        }

        for ((d, p), c) in empty_counts.into_iter() {
            if c == 1 || c == 2 {
                next.insert((d,p));
            }
        } 

        state = next;
    }

    state.len()
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

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
