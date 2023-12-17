use std::collections::{HashMap, HashSet};

type P = (usize, usize);

fn get_dir(prev: &HashMap<P, Option<P>>, (y, x): P) -> u8 {
    if let Some((py, px)) = prev.get(&(y, x)).and_then(|prv| *prv) {
        if py > y { 2 }
        else if py < y { 3 }
        else if px < x { 0 }
        else { 1 }
    } else { 0 }
}

fn next(prev: &HashMap<P, Option<P>>, (y, x): P, (h, w): P) -> Vec<P> {
    let (mut n, mut ne) = (Vec::new(), false);
    let mut state = (y, x);
    let dir = get_dir(prev, state);

    for _ in 0..2 {
        if let Some(prv) = prev.get(&state).and_then(|prv| *prv) {
            if get_dir(prev, prv) != dir {
                ne = true;
            }

            state = prv;
        } else {
            ne = true;
        }
    }

    if ne {
        match dir {
            0 => if x + 1 < w { n.push((y, x + 1)); },
            1 => if x > 0     { n.push((y, x - 1)); },
            2 => if y > 0     { n.push((y - 1, x)); },
            3 => if y + 1 < h { n.push((y + 1, x)); },
            _ => panic!("invalid dir {}", dir),
        }
    }

    match dir {
        0 | 1 => {
            if y > 0     { n.push((y - 1, x)); }
            if y + 1 < h { n.push((y + 1, x)); }
        }
        2 | 3 => {
            if x > 0     { n.push((y, x - 1)); }
            if x + 1 < w { n.push((y, x + 1)); }
        }
        _ => panic!("invalid dir {}", dir),
    }

    n
}

fn main() {
    let inp = std::fs::read_to_string("../input/17").unwrap().trim().to_string();
    let inp = inp
        .split("\n")
        .map(|s| s.chars().map(|n| (n as u8 - b'0') as u64).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut dist = Vec::new();
    let mut prev = HashMap::new();
    let mut queue = HashSet::new();

    for i in 0..inp.len() {
        dist.push(vec![u64::MAX; inp[i].len()]);
        for j in 0..inp[i].len() {
            queue.insert((i, j));
        }
    }

    dist[0][0] = 0;
    prev.insert((0, 0), None);

    while !queue.is_empty() {
        let (br, uh) = {
            let mut found = None;
            let mut dst = u64::MAX;

            for &(y,x) in &queue {
                if dist[y][x] < dst {
                    found = Some((y, x));
                    dst = dist[y][x];
                }
            }
            for i in 0..dist.len() {
                for j in 0..dist[i].len() {
                    print!("{}\t", dist[i][j]);
                }
                println!();
            }
            println!();

            found.unwrap()
        }; 

        if br == inp.len() - 1 && uh == inp[0].len() - 1 {
            println!("{:?}", dist[inp.len() - 1][inp[0].len() - 1]);
            break;
        }

        queue.remove(&(br, uh));

        for (y, x) in next(&prev, (br, uh), (inp.len(), inp[0].len())) {
            let alt = dist[br][uh] + inp[y][x];

            if alt < dist[y][x] {
                dist[y][x] = alt;
                prev.insert((y, x), Some((br, uh)));
            }
        }
    }

    for i in 0..dist.len() {
        for j in 0..dist[i].len() {
            print!("{}\t", dist[i][j]);
        }
        println!();
    }
    println!();

    let mut u = Some((inp.len() - 1, inp[0].len() - 1));
    let mut p = HashMap::new();

    while let Some(state) = u {
        let d = get_dir(&prev, state);
        let ch = match d {
            0 => '>',
            1 => '<',
            2 => '^',
            3 => 'v',
            _ => unreachable!(),
        };
        p.insert(state, ch);
        u = *prev.get(&state).unwrap();
    }

    for i in 0..dist.len() {
        for j in 0..dist[i].len() {
            if p.contains_key(&(i, j)) {
                print!("{}\t", p.get(&(i, j)).unwrap());
            } else if dist[i][j] > 20000 {
                print!("{}\t", "INF");
            } else {
                print!("{}\t", dist[i][j]);
            }
        }
        println!();
    }

}
