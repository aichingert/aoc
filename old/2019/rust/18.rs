use std::collections::{VecDeque, HashSet};

type Point = (usize, usize);

fn part_one(map: &Vec<Vec<char>>, start: Point, max_keys: u32) -> u32 {
    let mut bfs = VecDeque::from([(start, 0u32, 0u32)]);
    let mut vis = HashSet::new();

    while let Some(((y, x), mut keys, dis)) = bfs.pop_front() {
        if keys.count_ones() == max_keys {
            return dis - 1;
        }

        let cur = map[y][x];
        let cond = cur >= 'A' && cur <= 'Z' && keys & (1 << (cur as u8 - b'A')) == 0;

        if cur == '#' || !vis.insert((y, x, keys)) || cond {
            continue;
        }

        if cur >= 'a' && cur <= 'z' {
            keys |= 1 << (cur as u8 - b'A');
        }

        for (r, c) in [(0,1),(1,0),(0,-1),(-1,0)] {
            bfs.push_back((((y as i32 + r) as usize, (x as i32 + c) as usize), keys, dis + 1));
        }
    }

    panic!("help");
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap().trim().to_string();
    let map = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut start = (0, 0);
    let mut expec = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] >= 'a' && map[i][j] <= 'z' {
                expec += 1;
            }

            if map[i][j] == '@' {
                start = (i, j);
            }
        }
    }

    println!("{:?}", part_one(&map, start, expec));
}
