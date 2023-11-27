use std::collections::{HashSet, HashMap, VecDeque};

type Point = (usize, usize);

fn part_one(start: Point, end: Point, paths: &HashSet<Point>, portals: &HashMap<Point, Point>) -> u32 {
    let mut bfs = VecDeque::from([(start, 0)]);
    let mut kwn = HashSet::from([start]);

    while let Some((pos, t)) = bfs.pop_front() {
        for nxt in [(0,1),(1,0),(0,-1),(-1,0)] {
            let cord = ((pos.0 as i32 + nxt.0) as usize, (pos.1 as i32 + nxt.1) as usize);

            if cord == end {
                return t + 1;
            }

            if !kwn.insert(cord) { continue; }

            if paths.contains(&cord) {
                bfs.push_back((cord, t + 1));
            }

            if let Some(jmp) = portals.get(&cord) {
                bfs.push_back((*jmp, t + 2));
            }
        }

    }

    panic!("no solution found!");
}

fn part_two(start: Point, end: Point, paths: &HashSet<Point>, portals: &HashMap<Point, Point>) -> u32 {
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
                let is_outer_portal = cord.0 < 3 || cord.1 < 3 || cord.0 + 4 > 113 || cord.1 + 4 > 119;

                if l == 0 && !is_outer_portal {
                    bfs.push_back((*jmp, l + 1, t + 2));
                } else if l != 0 {
                    let layer = if is_outer_portal {
                        l - 1
                    } else {
                        l + 1
                    };

                    bfs.push_back((*jmp, layer, t + 2));
                }
            }
        }

    }

    panic!("no solution found!");
}

fn parse() -> (Point, Point, HashSet<Point>, HashMap<Point, Point>) {
    let inp = std::fs::read_to_string("../input/20").unwrap();
    let map = inp.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut portals: HashMap<Point, Point> = HashMap::new();
    let mut parsing: HashMap<(char, char), Point> = HashMap::new();
    let mut known = HashSet::new();
    let mut paths = HashSet::new();
    let (mut start, mut end) = ((0, 0), (0,0));

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '.' => { paths.insert((i, j)); }
                'A'..='Z' => {
                    let mut sp = (0, 0);

                    for cord in [(0,1),(1,0),(0,-1),(-1,0)] {
                        let nxt = (i as i32 + cord.0, j as i32 + cord.1);

                        if nxt.0 < 0 || nxt.1 < 0 || nxt.0 >= map.len() as i32 || nxt.1 >= map[i].len() as i32 || 
                        map[nxt.0 as usize][nxt.1 as usize] == '.' || map[nxt.0 as usize][nxt.1 as usize] == ' ' {
                            continue;
                        }

                        sp = (nxt.0 as usize, nxt.1 as usize);
                        break;
                    }

                    let mut pos = None;
                    let fp = (i, j);

                    if !known.insert(fp) || !known.insert(sp) {
                        continue;
                    }

                    for (y, x) in [fp, sp] {
                        pos = if y > 0 && map[y - 1][x] == '.' {
                            Some((y - 1, x))
                        } else if y + 1 < map.len() && map[y + 1][x] == '.' {
                            Some((y + 1, x))
                        } else if x > 0 && map[y][x - 1] == '.' {
                            Some((y, x - 1))
                        } else if x + 1 < map[i].len() && map[y][x + 1] == '.' {
                            Some((y, x + 1))
                        } else {
                            None
                        };

                        if pos.is_some() {
                            break;
                        }
                    }
                    let pos = pos.unwrap();

                    let (fs, sc) = (map[fp.0][fp.1], map[sp.0][sp.1]);

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
                _ => (),
            }
        }
    }

    (start, end, paths, portals) 
}

fn main() {
    let (start, end, paths, portals) = parse();

    println!("Part one: {}", part_one(start, end, &paths, &portals));
    println!("Part two: {}", part_two(start, end, &paths, &portals));
}
