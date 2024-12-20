use std::collections::{VecDeque, HashSet, HashMap};

const P1: i32 = 100;

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/20").unwrap();
    let g = inp.lines().filter(|l| !l.is_empty()).map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let d = g.iter().map(|l| l.iter().filter(|&&c| c == '.').count()).sum::<usize>() + 1;
    println!("{d:?}");

    let (mut sy, mut sx) = (0, 0);
    let (mut ey, mut ex) = (0, 0);
    let mut p1 = 0;

    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == 'S' {
                (sy, sx) = (i, j);
            }
            if g[i][j] == 'E' {
                (ey, ex) = (i, j);
            }
        }
    }

    let mut bfs = VecDeque::from([(sy, sx, 0)]);
    let mut map = HashMap::<(usize, usize), i32>::new();

    while let Some((y, x, s)) = bfs.pop_front() {
        if map.contains_key(&(y, x)) {
            continue;
        }
        map.insert((y, x), s);
        if (y, x) == (ey, ex) {
            break;
        }

        for (dy, dx) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);

            if g[ny as usize][nx as usize] != '#' {
                bfs.push_back((ny as usize, nx as usize, s + 1));
            }
        }
    }

    for &(fy, fx) in map.keys() {
        for ty in -20..=20 {
            for tx in -20..=20 {
                if ty == 0 && tx == 0 {
                    continue;
                }
                // x ; y | a ; b
                // 2 ; 2 | 5 ; 5 => 
                //
                // |5 - 2| + |5 - 2| => 3 3

                let (py, px) = (fy as i32 + ty, fx as i32 + tx);

                if py < 0 || py >= g.len() as i32 || px < 0 || px >= g[0].len() as i32 {
                    continue;
                }

                if g[py as usize][px as usize] != '#' {
                    let d = (fy as i32 - py).abs() + (fx as i32 - px).abs();

                    if d <= 20 {
                        let t = map[&(py as usize, px as usize)];
                        let f = map[&(fy, fx)];
                        
                        if P1 <= t - f - d {
                            p1 += 1;
                        }
                    }
                }
            }
        }

        println!("{fy} {fx}");
    }

    /*
    for ((ty, tx, fy, fx), d) in cts.into_iter() {
        let t = map[&(ty, tx)];
        let f = map[&(fy, fx)];
        
        if P1 <= t - f - d {
            p1 += 1;
        }
    }
    */

    /*
    while let Some((ty, tx, fy, fx)) = cts.pop_front() {

        let t = map[&(ty, tx)];
        let f = map[&(fy, fx)];
        
        if P1 <= t - f - 2 {
            p1 += 1;
        }
        if t > f + 2 {
            m.entry(t - f - 2).and_modify(|n| *n += 1).or_insert(1);
            println!("{t} {f} => {}", t - f - 2);
        }
    }
    */

    //let mut v = m.into_iter().collect::<Vec<_>>();
    //v.sort_unstable();

    //println!("{v:?}");
    println!("{p1}");
}
