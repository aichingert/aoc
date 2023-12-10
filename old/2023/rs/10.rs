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

fn get_loop(inp: &Vec<Vec<char>>) -> HashMap<(usize, usize), u32> {
    let mut starting = (0, 0);

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

    let pos = [vec![(1,0), (-1,0)], vec![(0,1),(0,-1)],vec![(-1,0),(0,-1)],vec![(-1,0),(0,1)],vec![(1,0),(0,-1)],vec![(1,0),(0,1)]];

    'lp: for tiles in pos {
        let mut bfs: VecDeque<((usize, usize), u32)> = VecDeque::new();
        let mut dist = HashMap::from([(starting, 0)]);

        for tile in tiles {
            let (y, x) = (starting.0 as i32 + tile.0, starting.1 as i32 + tile.1);

            if y < 0 || x < 0 || y >= inp.len() as i32 || x >= inp[0].len() as i32 {
                continue 'lp;
            }

            bfs.push_front(((y as usize, x as usize), 0));
        }

        while let Some(((y, x), d)) = bfs.pop_front() {
            if dist.insert((y, x), d).is_some() {
                continue;
            }

            match inp[y][x] {
                '-' => append_points(((y, x), d), inp, vec![(0,1),(0,-1)], &mut bfs, |ch: char, vec: (i32, i32)| {
                    match (ch, vec.1) {
                        ('-', _) | ('J', 1) | ('7', 1) | ('L', -1) | ('F', -1) => true,
                        _ => false,
                    }
                }),
                '|' => append_points(((y, x), d), inp, vec![(1,0),(-1,0)], &mut bfs, |ch: char, vec: (i32, i32)| {
                    match (ch, vec.0) {
                        ('|', _) | ('J', 1) | ('7', -1) | ('L', 1) | ('F', -1) => true,
                        _ => false,
                    }
                }),
                'L' => append_points(((y, x), d), inp, vec![(0,1),(-1,0)], &mut bfs, |ch: char, vec: (i32, i32)| {
                    match (ch, vec.1) {
                        ('-', 1) | ('|', 0) | ('J', 1) | ('7', _) | ('F', 0) => true,
                        _ => false,
                    }
                }),
                'J' => append_points(((y, x), d), inp, vec![(0,-1),(-1,0)], &mut bfs, |ch: char, vec: (i32, i32)| {
                    match (ch, vec.1) {
                        ('-', -1) | ('|', 0) | ('L', -1) | ('7', 0) | ('F', _) => true,
                        _ => false,
                    }
                }),
                '7' => append_points(((y, x), d), inp, vec![(0,-1),(1,0)], &mut bfs, |ch: char, vec: (i32, i32)| {
                    match (ch, vec.1) {
                        ('-', -1) | ('|', 0) | ('L', _) | ('J', 0) | ('F', -1) => true,
                        _ => false,
                    }
                }),
                'F' => append_points(((y, x), d), inp, vec![(0,1),(1,0)], &mut bfs, |ch: char, vec: (i32, i32)| {
                    match (ch, vec.1) {
                        ('-', 1) | ('|', 0) | ('L', 0) | ('J', _) | ('7', 1) => true,
                        _ => false,
                    }
                }),
                _ => break,
            }
        }

        if dist.values().filter(|&&x| x == 2).count() == 2 {
            return dist;
        }
    }

    panic!("No loop found");
}

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let dist = get_loop(&inp);

    println!("{}", dist.values().max().unwrap());
}
