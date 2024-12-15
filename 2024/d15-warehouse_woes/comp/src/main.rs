use std::collections::{HashMap, HashSet};

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/15").unwrap();
    let mut p1 = 0;
    let mut p2 = 0;

    let (map, cons) = inp.split_once("\n\n").unwrap();
    let mut prv = map.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut map = prv.clone();
    let (mut y, mut x) = (0, 0);

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                y = i;
                x = j;
            }
        }
    }

    let v: HashMap<char, (i32, i32)> = HashMap::from_iter([
        ('v', (1, 0)), ('^', (-1, 0)), ('<', (0, -1)), ('>', (0, 1))
    ]);

    for line in cons.lines().filter(|l| !l.is_empty()) {
        for con in line.chars() {
            let (dy, dx) = v[&con];
            let (ny, nx) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);

            match map[ny][nx] {
                'O' => {
                    let (mut oy, mut ox) = (ny, nx);
                    while map[oy][ox] == 'O' {
                        (oy, ox) = ((oy as i32 + dy) as usize, (ox as i32 + dx) as usize);
                    }

                    if map[oy][ox] == '#' {
                        continue;
                    }

                    println!("{y} ; {x} ; {:?} | {oy} ; {ox} | {ny} ; {nx} ", (dy, dx));

                    map[oy][ox] = map[ny][nx];
                    map[ny][nx] = map[y][x];
                    map[y][x] = '.';
                    (y, x) = (ny, nx);
                }
                '.' => {
                    map[y][x] = '.';
                    map[ny][nx] = '@';
                    (y, x) = (ny, nx);
                }
                _ => (),
            } 
        }
    } 


    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                p1 += 100 * i + j;
            }
        }
    }

    let mut map = vec![vec!['.'; map[0].len() * 2]; map.len()];
    for i in 0..prv.len() {
        for j in 0..prv[i].len() {
            match prv[i][j] {
                'O' => {
                    map[i][j * 2] = '[';
                    map[i][j * 2 + 1] = ']';
                }
                '@' => {
                    map[i][j * 2] = '@';
                    (y, x) = (i, j * 2);
                }
                '#' => {
                    map[i][j * 2] = '#';
                    map[i][j * 2 + 1] = '#';
                }
                _ => (),
            }
        }
    }

    for line in cons.lines().filter(|l| !l.is_empty()) {
        'c: for con in line.chars() {
            let (dy, dx) = v[&con];
            let (ny, nx) = ((y as i32 + dy) as usize, (x as i32 + dx) as usize);

            match map[ny][nx] {
                '[' | ']' => {
                    if con == 'v' || con == '^' {
                        let mut bfs = vec![(ny, nx)];
                        let mut btm = HashSet::new();

                        while let Some((sy, sx)) = bfs.pop() {
                            let (oy, ox) = ((sy as i32 + dy) as usize, (sx as i32 + dx) as usize);
                            match map[sy][sx] {
                                '[' => {
                                    bfs.push((oy, ox));
                                    bfs.push((oy, ox + 1));

                                    btm.insert((oy, ox));
                                    btm.insert((oy, ox + 1));
                                }
                                ']' => {
                                    bfs.push((oy, ox));
                                    bfs.push((oy, ox - 1));

                                    btm.insert((oy, ox));
                                    btm.insert((oy, ox - 1));
                                }
                                '.' => (),
                                '#' => continue 'c,
                                _ => unreachable!(),
                            }
                        }

                        let mut btm = btm.into_iter().collect::<Vec<_>>();
                        btm.sort_by(|a, b| if con == '^' { a.cmp(b) } else { b.cmp(a) });

                        for i in 0..btm.len() {
                            let (ty, tx) = btm[i];
                            let (fy, fx) = ((ty as i32 - dy) as usize, (tx as i32 - dx) as usize);

                            map[ty][tx] = map[fy][fx];
                            map[fy][fx] = '.';
                        }

                        map[y][x] = '.';
                        map[ny][nx] = '@';
                        (y, x) = (ny, nx);
                    } else {
                        let (mut oy, mut ox) = (ny, nx);
                        while map[oy][ox] == '[' || map[oy][ox] == ']' {
                            (oy, ox) = ((oy as i32 + dy) as usize, (ox as i32 + dx) as usize);
                        }

                        if map[oy][ox] == '#' {
                            continue;
                        }

                        while x != ox {
                            let px = ox;
                            ox = (ox as i32 - dx) as usize;
                            map[y][px] = map[y][ox];
                        }

                        map[y][x] = '.';
                        (y, x) = (ny, nx);
                    }
                }
                '.' => {
                    map[y][x] = '.';
                    map[ny][nx] = '@';
                    (y, x) = (ny, nx);
                }
                _ => (),
            } 
        }
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '[' {
                p2 += i * 100 + j;
            }
        }
    }

    println!("p1: {p1}");
    println!("p2: {p2}");
}
