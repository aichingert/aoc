use std::collections::{HashMap, HashSet, VecDeque};

type N = i128;

fn dfs(ms: N, y: N, x: N, s: N, map: &Vec<Vec<char>>, filter: &mut HashSet<(N, N, N)>) -> N {
    if !filter.insert((y, x, s)) {
        return 0;
    } 

    let (ay, ax) = (y.rem_euclid(map.len() as N), x.rem_euclid(map[0].len() as N));
    let (ay, ax) = (ay as usize, ax as usize);

    if map[ay][ax] == '#' {
        return 0;
    }

    if s >= ms {
        return 1;
    }

    let mut res: N = 0;
    let s = s + 1;

    res += dfs(ms, y + 1, x, s, map, filter);
    res += dfs(ms, y - 1, x, s, map, filter);
    res += dfs(ms, y, x + 1, s, map, filter);
    res += dfs(ms, y, x - 1, s, map, filter);

    res
}

fn main() {
    let inp = std::fs::read_to_string("../input/21").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut start = (0, 0);

    'f: for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] == 'S' {
                start = (i as N, j as N);
                break 'f;
            }
        }
    }

    //println!("{}", dfs(5000, start.0, start.1, 0, &inp, &mut HashSet::new()));

    let (h, w) = (inp.len() as N, inp[0].len() as N);


    let mut prevprev = HashSet::from([start]);
    let mut prev = apply(&prevprev, &inp, (w, h));
    let mut state = prev.clone();
    let mut next = apply_fast(&prevprev, &prev, &inp, (w, h));

    let mut ppl = prevprev.len();
    let mut pl = prev.len();


    println!("      nxt     ");
    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if next.contains(&(i as N, j as N)) {
                print!("O");
            } else {
                print!("{}", inp[i][j]);
            }
        }
        println!();
    }
    println!("================================================");
    let n = 26501365;

    for i in 1..n {
        if i % 100 == 0 {
            println!("{i}");
        }
        let next = apply_fast(&prevprev, &prev, &inp, (w, h));

        let cp = ppl + next.len();
        ppl = pl;
        pl = cp;
        prevprev = prev;
        prev = next; 
    }

    println!("ACT: {:?}", pl);
}

fn apply(set: &HashSet<(N, N)>, inp: &Vec<Vec<char>>, (h, w): (N, N)) -> HashSet<(N, N)> {
    let mut next = HashSet::new();

    for (y, x) in set {
        for (r, c) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (ay, ax) = ((y + r).rem_euclid(h), (x + c).rem_euclid(w));
            let (ay, ax) = (ay as usize, ax as usize);

            if inp[ay][ax] != '#' {
                next.insert((y + r, x + c));
            }
        }
    }

    next
}

fn apply_fast(prevprev: &HashSet<(N, N)>, prev: &HashSet<(N, N)>, inp: &Vec<Vec<char>>, (h,w): (N, N)) -> HashSet<(N, N)> {
    let mut next = HashSet::new();

    for (y, x) in prev {
        for (r, c) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (ny, nx) = (y + r, x + c);
            let (ay, ax) = ((ny).rem_euclid(h), (nx).rem_euclid(w));
            let (ay, ax) = (ay as usize, ax as usize);

            if inp[ay][ax] != '#' && !prevprev.contains(&(ny, nx)) {
                next.insert((ny, nx));
            }
        }
    }
    
    next
}
