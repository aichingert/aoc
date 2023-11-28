use std::collections::HashSet;

type Point = (usize, usize);

fn part_one(
    cord: Point,
    mut key_ring: u32, 
    solution: u32,
    distance: u32,
    map: &Vec<Vec<char>>, 
    mut vis: HashSet<(usize, usize)>) -> u32 {
    let mut ans = u32::MAX;

    let ch = map[cord.0][cord.1];

    if ch == '#' || vis.contains(&(cord)) {
        return ans;
    }
    vis.insert(cord);

    if ch >= 'A' && ch <= 'Z' && key_ring & (1 << (ch as u8 - b'A')) == 0 {
        return ans;
    }

    if ch >= 'a' && ch <= 'z' && key_ring & (1 << (ch as u8 - b'a')) == 0 {
        key_ring |= 1 << (ch as u8 - b'a');
        vis.clear();
    }

    if key_ring.count_ones() == solution {
        return distance;
    }

    let dist = distance + 1;

    ans = ans.min(part_one((cord.0 - 1, cord.1), key_ring, solution, dist, map, vis.clone()));
    ans = ans.min(part_one((cord.0 + 1, cord.1), key_ring, solution, dist, map, vis.clone()));
    ans = ans.min(part_one((cord.0, cord.1 - 1), key_ring, solution, dist, map, vis.clone()));
    ans = ans.min(part_one((cord.0, cord.1 + 1), key_ring, solution, dist, map, vis));

    ans
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

    println!("{:?}", part_one(start, 0, expec, 0, &map, HashSet::new()));
}
