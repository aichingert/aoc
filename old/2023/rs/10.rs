use std::collections::{HashMap, VecDeque};

type Point = (usize, usize);

fn append_points(cur: (Point, u32), inp: &Vec<Vec<char>>, directions: Vec<(i32, i32)>, 
    bfs: &mut VecDeque<(Point, u32)>, cond: impl Fn(char, (i32, i32)) -> bool) 
{
    let ((y, x), d) = cur;

    for dir in directions {
        let (ny, nx) = (y as i32 + dir.0, x as i32 + dir.1);

        if ny < 0 || nx < 0 || ny >= inp.len() as i32 || nx >= inp[y].len() as i32 {
            continue;
        }

        let (ny, nx) = (ny as usize, nx as usize);

        if cond(inp[ny][nx], dir) {
            bfs.push_back(((ny, nx), d + 1));
        }
    }
}

fn get_loop(inp: &mut Vec<Vec<char>>) -> Option<HashMap<(usize, usize), u32>> {
    let mut starting = (0,0);

    'search: for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            match inp[i][j] {
                'S' => {
                    starting = (i, j);
                    break 'search;
                }
                _ => (),
            }
        }
    }

    let pos = [
        ('|', vec![(1,0), (-1,0)]),
        ('-', vec![(0,1),(0,-1)]),
        ('L', vec![(-1,0),(0,1)]),
        ('J', vec![(-1,0),(0,-1)]),
        ('7', vec![(1,0),(0,-1)]),
        ('F', vec![(1,0),(0,1)]),
    ];

    // minus 0; pipe 1; L: 2; J: 3; 7: 4; F: 5;
    let validators = [
        |ch: char, vec: (i32, i32)| { matches!((ch, vec.1), ('-', _) | ('J', 1) | ('7', 1) | ('L', -1) | ('F', -1)) },
        |ch: char, vec: (i32, i32)| { matches!((ch, vec.0), ('|', _) | ('J', 1) | ('7', -1) | ('L', 1) | ('F', -1)) },
        |ch: char, vec: (i32, i32)| { matches!((ch, vec.1), ('-', 1) | ('|', 0) | ('J', 1) | ('7', _) | ('F', 0)) },
        |ch: char, vec: (i32, i32)| { matches!((ch, vec.1), ('-', -1) | ('|', 0) | ('L', -1) | ('7', 0) | ('F', _)) },
        |ch: char, vec: (i32, i32)| { matches!((ch, vec.1), ('-', -1) | ('|', 0) | ('L', _) | ('J', 0) | ('F', -1)) },
        |ch: char, vec: (i32, i32)| { matches!((ch, vec.1), ('-', 1) | ('|', 0) | ('L', 0) | ('J', _) | ('7', 1))},
    ];

    'lp: for (rep, tiles) in pos {
        let mut bfs: VecDeque<((usize, usize), u32)> = VecDeque::new();
        let mut dist = HashMap::from([(starting, 0)]);
        inp[starting.0][starting.1] = rep;

        for tile in tiles {
            let (y, x) = (starting.0 as i32 + tile.0, starting.1 as i32 + tile.1);

            if y < 0 || x < 0 || y >= inp.len() as i32 || x >= inp[0].len() as i32 {
                continue 'lp;
            }

            let (y, x) = (y as usize, x as usize);

            match rep {
                '-' => if !validators[0](inp[y][x], tile) { continue 'lp; }
                '|' => if !validators[1](inp[y][x], tile) { continue 'lp; }
                'L' => if !validators[2](inp[y][x], tile) { continue 'lp; }
                'J' => if !validators[3](inp[y][x], tile) { continue 'lp; }
                '7' => if !validators[4](inp[y][x], tile) { continue 'lp; }
                'F' => if !validators[5](inp[y][x], tile) { continue 'lp; }
                _ => continue 'lp,
            }

            bfs.push_front(((y as usize, x as usize), 0));
        }

        while let Some(((y, x), d)) = bfs.pop_front() {
            if dist.insert((y, x), d).is_some() {
                continue;
            }

            match inp[y][x] {
                '-' => append_points(((y, x), d), inp, vec![(0,1),(0,-1)], &mut bfs, validators[0]),
                '|' => append_points(((y, x), d), inp, vec![(1,0),(-1,0)], &mut bfs, validators[1]),
                'L' => append_points(((y, x), d), inp, vec![(0,1),(-1,0)], &mut bfs, validators[2]),
                'J' => append_points(((y, x), d), inp, vec![(0,-1),(-1,0)], &mut bfs, validators[3]),
                '7' => append_points(((y, x), d), inp, vec![(0,-1),(1,0)], &mut bfs, validators[4]),
                'F' => append_points(((y, x), d), inp, vec![(0,1),(1,0)], &mut bfs, validators[5]),
                _ => break,
            }
        }

        if dist.values().filter(|&&x| x == 2).count() == 2 {
            return Some(dist);
        }
    }

    None
}

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap().trim().to_string();
    let mut inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let looping = get_loop(&mut inp).unwrap();
    let mut map = Vec::new();

    for i in 0..inp.len() {
        let mut line = Vec::new();
        for j in 0..inp[i].len() {
            if !looping.contains_key(&(i, j)) {
                line.push(false);
                line.push(false);
            } else {
                if j + 1 < inp[i].len() && (inp[i][j] == '-' || inp[i][j] == 'F' || inp[i][j] == 'L') 
                    && (inp[i][j + 1] == '-' || inp[i][j + 1] == 'J' || inp[i][j + 1] == '7') {
                    line.push(true);
                    line.push(true);
                } else {
                    line.push(true);
                    line.push(false);
                }
            }
        }

        let mut double = Vec::new();
        if i + 1 < inp.len() {
            for j in 0..inp[i].len() {
                if looping.contains_key(&(i, j)) && looping.contains_key(&(i + 1, j)) {
                    let ch = inp[i][j];
                    let to = inp[i + 1][j];

                    if (ch == 'F' || ch == '7' || ch == '|') && (to == 'L' || to == '|' || to == 'J') {
                        double.push(true);
                        double.push(false);
                    } else {
                        double.push(false);
                        double.push(false);
                    }
                } else {
                    double.push(false);
                    double.push(false);
                }
            }
        } else {
            for _ in 0..inp[i].len() {
                double.push(false);
                double.push(false);
            }
        }

        map.push(line);
        map.push(double);
    }

    let mut ans = 0;

    for i in 0..map.len() {
        kill(i, 0, &mut map);
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !map[i][j] && i & 1 == 0 && j & 1 == 0 {
                ans += 1;
            }
        }
    }

    println!("Part one: {}", looping.values().max().unwrap());
    println!("Part two: {}", ans);
}

fn kill(i: usize, j: usize, map: &mut Vec<Vec<bool>>) {
    let mut bfs = VecDeque::from([(i, j)]);

    while let Some((i, j)) = bfs.pop_front() {
        map[i][j] = true;

        for vec in [(0,1),(1,0),(-1,0),(0,-1)] {
            let (ny, nx) = (i as i32 + vec.0, j as i32 + vec.1);

            if ny < 0 || nx < 0 || ny >= map.len() as i32 || nx >= map[0].len() as i32 {
                continue;
            }

            let (ny, nx) = (ny as usize, nx as usize);

            if !map[ny][nx] {
                bfs.push_front((ny, nx));
            }
        }
    }

}

    
