use std::collections::{HashMap, VecDeque, HashSet};

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut starting = (0, 0);

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            match inp[i][j] {
                'S' => starting = (i, j),
                _ => (),
            }
        }
    }

    let mut bfs = VecDeque::from([(starting, 0)]);
    let mut dis = HashMap::new();

    while let Some(((y, x), d)) = bfs.pop_front() {
        if let Some(id) = dis.get(&(y, x)) {
            if *id > d {
                dis.insert((y, x), d);
            } else {
                continue;
            }
        } else {
            dis.insert((y, x), d);
        }

        match inp[y][x] {
            '-' => {
                for vec in [(0,1),(0,-1)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[0].len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    match (inp[ny][nx], vec.1) {
                        ('-', _) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('J', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('7', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('L', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('F', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        _ => (),
                    }
                }
            }
            '|' => {
                for vec in [(1,0),(-1,0)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[0].len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    match (inp[ny][nx], vec.0) {
                        ('|', _) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('L', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('J', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('F', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('7', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        _ => (),
                    }
                }
            }
            'L' => {
                for vec in [(0,1),(-1,0)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[0].len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    match (inp[ny][nx], vec.1) {
                        ('-', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('J', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('7', _) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('|', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('F', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        _ => (),
                    }
                }
            }
            'J' => {
                for vec in [(0,-1),(-1,0)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[0].len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    match (inp[ny][nx], vec.1) {
                        ('-', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('L', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('|', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('F', _) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('7', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        _ => (),
                    }
                }
            }
            '7' => {
                for vec in [(0,-1),(1,0)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[0].len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    match (inp[ny][nx], vec.1) {
                        ('-', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('L', _) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('F', -1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('|', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('J', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        _ => (),
                    }
                }
            }
            'F' => {
                for vec in [(0,1),(1,0)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[0].len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    match (inp[ny][nx], vec.1) {
                        ('-', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('7', 1) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('J', _) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('|', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        ('L', 0) => { bfs.push_back(((ny, nx), d + 1)); }
                        _ => (),
                    }
                }
            }
            'S' => {
                //bfs.push_back(((y, x + 1), 1));
                //bfs.push_back(((y, x - 1), 1));
                //real ^

                bfs.push_back(((y + 1, x), 1));
                bfs.push_back(((y, x + 1), 1));
            }
            x => println!("whu: {:?} {x}", inp),
        }
    }

    let mut walls = HashSet::new();

    for ((y, x), _) in &dis {
        if inp[*y][*x] == '|' || inp[*y][*x] == '-' {
            walls.insert((*y, *x));
        }
    }

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if let Some(_) = dis.get(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut ans = 0;

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            let mut bfs = VecDeque::from([(i, j)]);
            let mut sen = HashSet::from([(i, j)]);
            let mut out = false;

            'lp: while let Some((y, x)) = bfs.pop_front() {
                for vec in [(0,1),(1,0),(-1,0),(0,-1)] {
                    let (ny, nx) = (y as i32 + vec.0, x as i32 + vec.1);

                    if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[i].len() as i32 {
                        out = true;
                        break 'lp;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);
                    if !sen.insert((ny, nx)) {
                        continue;
                    }

                    if walls.contains(&(ny, nx)) {
                        if inp[ny][nx] == '|' && vec.1 != 0 || inp[ny][nx] == '-' && vec.0 != 0 {
                            bfs.push_back((ny, nx));
                        }
                    } else {
                        bfs.push_back((ny, nx));
                    }
                }
            }

            if !out {
                println!("{:?}", (i, j));
                ans += 1;
            }
        }
    }

    println!("{:?}", dis.values().max());
    println!("{:?}", ans);
}
