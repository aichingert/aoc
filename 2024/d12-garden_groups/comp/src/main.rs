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
            println!("{}", g[i][j]);

            let mut a = 0;
            let mut p = 0;
            let mut reg = HashSet::new();
            let mut bfs = vec![(i, j)];

            while let Some((y, x)) = bfs.pop() {
                if !s.insert((y, x)) {
                    continue;
                }

                reg.insert((y as i32, x as i32));
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

            let mut c = 0;
            let mut ccs = HashSet::new();

            for &(y, x) in &reg {
                for (cy, cx) in [(-0.5, 0.5), (0.5, 0.5), (0.5, -0.5), (-0.5, -0.5)] {
                    let cr = y as f32 + cy;
                    let cc = x as f32 + cx;

                    let mut p = Vec::new();

                    for (dr, dc) in [(-0.5, 0.5), (0.5, 0.5), (0.5, -0.5), (-0.5, -0.5)] {
                        p.push(((cr + dr) as i32, (cc + dc) as i32));
                    }

                    ccs.insert(p);
                }
            }

            for p in ccs {
                let mut res = [false; 4];
                let mut cnt = 0;

                for (i, &(r, c)) in p.iter().enumerate() {
                    if reg.contains(&(r, c)) {
                        cnt += 1;
                        res[i] = true;
                    }
                }

                match cnt {
                    1 | 3 => c += 1,
                    2 if res == [true, false, true, false] || res == [false, true, false, true] => c += 2,
                    _ => (),
                }
            }

            p1 += a * p;
            p2 += a * c;
        }
    }

    println!("{p1:?}");
    println!("{p2:?}");
}
