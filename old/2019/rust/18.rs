use std::collections::{VecDeque, HashSet};

type Point = (usize, usize);

fn part_one(map: &Vec<Vec<char>>, start: Point, max_keys: u32) -> u32 {
    let mut bfs = VecDeque::from([(start, 0u32, 0u32)]);
    let mut vis = HashSet::new();

    while let Some(((y, x), mut keys, dist)) = bfs.pop_front() {
        if keys.count_ones() == max_keys {
            return dist - 1;
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
            bfs.push_back((((y as i32 + r) as usize, (x as i32 + c) as usize), keys, dist + 1));
        }
    }

    panic!("help");
}

fn part_two(map: &mut Vec<Vec<char>>, (y, x): Point, max_keys: u32) -> u32 {
    // Transform for part two
    for i in -1..2 {
        map[(y as i32 + i) as usize][x] = '#';
        map[y][(x as i32 + i) as usize] = '#';
    }

    map[y - 1][x - 1] = '@';
    map[y + 1][x + 1] = '@';
    map[y - 1][x + 1] = '@';
    map[y + 1][x - 1] = '@';
    // Transform done

    let mut bfs = VecDeque::from([(vec![(y-1,x-1), (y+1,x+1), (y-1, x+1), (y+1,x-1)], 0u32, 0u32)]);
    let mut vis = HashSet::new();

    while let Some((robos, mut keys, dist)) = bfs.pop_front() {
        if keys.count_ones() == max_keys {
            return dist - 1;
        }

        for i in 0..robos.len() {
            let cur = map[robos[i].0][robos[i].1];
            let cond = cur >= 'A' && cur <= 'Z' && keys & (1 << (cur as u8 - b'A')) == 0;

            if cur == '#' || !vis.insert((robos[i].0, robos[i].1, keys)) || cond {
                continue;
            }

            if cur >= 'a' && cur <= 'z' {
                keys |= 1 << (cur as u8 - b'a');
            }

            for (r, c) in [(0,1),(1,0),(0,-1),(-1,0)] {
                let robo = ((robos[i].0 as i32 + r) as usize, (robos[i].1 as i32 + c) as usize);

                let mut next = vec![robo];
                next.extend_from_slice(&robos[..i]);
                next.extend_from_slice(&robos[i + 1..]);

                bfs.push_back((next, keys, dist + 1));
            }
        }
    }

    panic!("welp");
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap().trim().to_string();
    let mut map = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (mut start, mut expec) = ((0, 0), 0);

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

    println!("Part one: {}", part_one(&map, start, expec));
    println!("Part two: {}", part_two(&mut map, start, expec));
}
