use std::collections::{HashSet, HashMap, VecDeque};

type Point = (usize, usize);

fn solve(start: Point, end: Point, paths: &HashSet<Point>, portals: &HashMap<Point, Point>, size: Option<Point>) -> u32 {
    let mut bfs = VecDeque::from([(start, 0, 0)]);
    let mut kwn = HashSet::from([(start, 0)]);

    while let Some((pos, l, t)) = bfs.pop_front() {
        for nxt in [(0,1),(1,0),(0,-1),(-1,0)] {
            let cord = ((pos.0 as i32 + nxt.0) as usize, (pos.1 as i32 + nxt.1) as usize);

            if l == 0 && cord == end {
                return t + 1;
            }

            if !kwn.insert((cord, l)) { continue; }

            if paths.contains(&cord) { 
                bfs.push_back((cord, l, t + 1));
            }

            if let Some(jmp) = portals.get(&cord) {
                if let Some((h, w)) = size {
                    let is_outer_portal = cord.0 < 3 || cord.1 < 3 || cord.0 + 4 >= h || cord.1 + 4 >= w ;

                    if let Some(layer) = match (l, is_outer_portal) {
                        (0, true)  => None,
                        (0, false) => Some(1),
                        (_, true)  => Some(l - 1),
                        (_, false) => Some(l + 1),
                    } {
                        bfs.push_back((*jmp, layer, t + 2));
                    }
                } else {
                    bfs.push_back((*jmp, l, t + 2));
                }
            }
        }
    }

    panic!("no solution found!");
}

fn parse() -> (Point, Point, HashSet<Point>, HashMap<Point, Point>, Point) {
    let inp = std::fs::read_to_string("../input/20").unwrap();
    let map = inp.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let (mut portals, mut parsing) = (HashMap::new(), HashMap::new());
    let (mut known, mut paths) = (HashSet::new(), HashSet::new());
    let (mut start, mut end) = ((0, 0), (0,0));

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                'A'..='Z' => { 
                    let (y, x) = [(0,1),(1,0),(0,-1),(-1,0)].to_vec().into_iter()
                        .filter(|c| { 
                            let point = (i as i32 + c.0, j as i32 + c.1);
                            point.0 > -1 && point.1 > -1 && point.0 < map.len() as i32 && point.1 < map[i].len() as i32 
                                && map[point.0 as usize][point.1 as usize].is_alphabetic()
                        })
                        .collect::<Vec<(i32, i32)>>()[0];
                    
                    let fp = (i, j);
                    let sp = ((i as i32 + y) as usize, (j as i32 + x) as usize);
                    let (fs, sc) = (map[fp.0][fp.1], map[sp.0][sp.1]);

                    if !known.insert(fp) || !known.insert(sp) {
                        continue;
                    }

                    let mut pos = (0, 0);

                    for (y, x) in [fp, sp] {
                        if      y     > 0            && map[y - 1][x] == '.' { pos = (y - 1, x); } 
                        else if y + 1 < map.len()    && map[y + 1][x] == '.' { pos = (y + 1, x); }
                        else if x     > 0            && map[y][x - 1] == '.' { pos = (y, x - 1); }
                        else if x + 1 < map[i].len() && map[y][x + 1] == '.' { pos = (y, x + 1); }
                    }

                    match (fs, sc) {
                        ('A', 'A') => start = pos,
                        ('Z', 'Z') => end = pos,
                        _ => (),
                    };

                    if let Some(position) = parsing.get(&(fs, sc)).or(parsing.get(&(sc, fs))) {
                        portals.insert(*position, pos);
                        portals.insert(pos, *position);
                    } else {
                        parsing.insert((fs, sc), pos);
                    }
                }
                '.' => { paths.insert((i, j)); }
                _   => (),
            }
        }
    }

    (start, end, paths, portals, (map.len(), map[0].len())) 
}

fn main() {
    let (start, end, paths, portals, size) = parse();

    println!("Part one: {}", solve(start, end, &paths, &portals, None));
    println!("Part two: {}", solve(start, end, &paths, &portals, Some(size)));
}
