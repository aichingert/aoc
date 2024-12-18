use std::collections::{VecDeque, HashSet};

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/18").unwrap();
    let inp = inp.lines().collect::<Vec<_>>();


    let mut g = [[false; 71]; 71];

    for line in inp.iter().take(1024) {
        let c = line.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        g[c[1]][c[0]] = true;
    }

    let (mut s, mut e) = ((0, 0), (g.len() - 1, g[0].len() - 1));

    let mut bfs = VecDeque::from([(s, 0)]);
    let mut sen = HashSet::new();
    let mut p1 = u32::MAX;
    let mut p2 = 0;

    while let Some(((y, x), s)) = bfs.pop_front() {
        if (y, x) == e {
            p1 = p1.min(s);
            continue;
        }
        if !sen.insert((y, x)) {
            continue;
        }

        for (dy, dx) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);

            if ny < 0 || ny >= g.len() as i32 || nx < 0 || nx >= g[0].len() as i32 {
                continue;
            }
            if g[ny as usize][nx as usize] {
                continue;
            }

            bfs.push_back(((ny as usize, nx as usize), s + 1));
        }
    }

    let mut i = 1023;

    loop {
        i += 1;
        let c = inp[i].split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        g[c[1]][c[0]] = true;

        let mut ans = u32::MAX;
        let mut bfs = VecDeque::from([(s, 0)]);
        let mut sen = HashSet::new();

        while let Some(((y, x), s)) = bfs.pop_front() {
            if (y, x) == e {
                ans = ans.min(s);
                continue;
            }
            if !sen.insert((y, x)) {
                continue;
            }

            for (dy, dx) in [(0,1),(1,0),(0,-1),(-1,0)] {
                let (ny, nx) = (y as i32 + dy, x as i32 + dx);

                if ny < 0 || ny >= g.len() as i32 || nx < 0 || nx >= g[0].len() as i32 {
                    continue;
                }
                if g[ny as usize][nx as usize] {
                    continue;
                }

                bfs.push_back(((ny as usize, nx as usize), s + 1));
            }
        }

        if ans == u32::MAX {
            println!("{c:?}");
            p2 = i;
            break;
        }
    }

    println!("p1: {p1}");
    println!("p2: {}", p2);
}
