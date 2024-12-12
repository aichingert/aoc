use std::collections::{HashMap, HashSet};

type V2 = (i32, i32);

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/12").unwrap();

    let g = inp.lines().filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut s = HashSet::new();
    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if s.contains(&(i, j)) {
                continue;
            }

            let mut a = 0;
            let mut p = 0;
            let mut cur = HashSet::new();
            let mut bfs = vec![(i, j)];

            while let Some((y, x)) = bfs.pop() {
                if !s.insert((y, x)) {
                    continue;
                }

                cur.insert((y, x));
                a += 1;

                for (dy, dx) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let (ny, nx) = (y as i32 + dy, x as i32 + dx);
                    
                    if ny < 0 || ny >= g.len() as i32 || nx < 0 || nx >= g[0].len() as i32 {
                        p += 1;
                        continue;
                    }
                    if g[ny as usize][nx as usize] != g[i][j] {
                        p += 1;
                        continue;
                    }

                    bfs.push((ny as usize, nx as usize));
                }
            }

            p1 += a * p;
            p2 += a * 0;
        }
    }

    println!("{p1:?}");
    println!("{p2:?}");
}
