use std::collections::{VecDeque, HashMap, HashSet};

fn main() {
    let g = std::fs::read_to_string("../../../input/2024/16").unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut sy, mut sx) = (0, 0);

    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == 'S' {
                (sy, sx) = (i, j);
            }
        }
    }

    let mut m = HashMap::new();
    let mut mis = i32::MAX;
    let mut mip = HashSet::new();
    let mut bfs = VecDeque::from([(0, 1, sy, sx, 0, HashSet::new())]);

    while let Some((dy, dx, py, px, c, mut s)) = bfs.pop_front() {
        if g[py][px] == '#' || c > mis {
            continue;
        }
        if let Some(&r) = m.get(&(py, px, dy, dx)) {
            if r < c {
                continue;
            }
        }

        s.insert((py, px));
        if g[py][px] == 'E' {
            if mis > c {
                mis = c;
                mip = s.clone();
            } else if mis == c {
                for &(y, x) in &s {
                    mip.insert((y, x));
                }
            }
        }
        m.insert((py, px, dy, dx), c);

        match (dy, dx) {
            (1, 0) => bfs.push_back((0, -1, py, px, c + 1000, s.clone())),
            (-1, 0) => bfs.push_back((0, 1, py, px, c + 1000, s.clone())),
            (0, 1) => bfs.push_back((1, 0, py, px, c + 1000, s.clone())),
            (0, -1) => bfs.push_back((-1, 0, py, px, c + 1000, s.clone())),
            _ => unreachable!(),
        }
        match (dy, dx) {
            (1, 0) => bfs.push_back((0, 1, py, px, c + 1000, s.clone())),
            (-1, 0) => bfs.push_back((0, -1, py, px, c + 1000, s.clone())),
            (0, 1) => bfs.push_back((-1, 0, py, px, c + 1000, s.clone())),
            (0, -1) => bfs.push_back((1, 0, py, px, c + 1000, s.clone())),
            _ => unreachable!(),
        }

        let (y, x) = ((py as i32 + dy) as usize, (px as i32 + dx) as usize);
        bfs.push_back((dy, dx, y, x, c + 1, s));
    }
    
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if mip.contains(&(i, j)) {
                print!("O");
            } else {
                print!("{}", g[i][j]);
            }
        }
        println!();
    }

    println!("{mis}");
    println!("{}", mip.len());
}
