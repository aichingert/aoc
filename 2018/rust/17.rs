// Advent of Code 2018, day 17
// (c) aichingert

use std::collections::HashSet;

fn part1(walls: &HashSet<(i32,i32)>, water: &mut HashSet<(i32,i32)>, lowest: i32, y: i32) -> usize {
    fill((500,0), walls, water, y);
    water.iter().filter(|(_,y)| *y > lowest).count()
}

fn part2(walls: &HashSet<(i32,i32)>, water: &HashSet<(i32,i32)>) -> usize {
    water.iter().filter(|(x,y)| {
        (water.contains(&(*x-1,*y)) || water.contains(&(*x+1,*y)) || 
        walls.contains(&(*x-1,*y)) || walls.contains(&(*x+1,*y))) &&
        check_direction(walls, &mut water.clone(), &mut (*x, *y), 1) &&
        check_direction(walls, &mut water.clone(), &mut (*x, *y), -1)
    }).count()
}

fn fill(start: (i32,i32), walls: &HashSet<(i32,i32)>, water: &mut HashSet<(i32,i32)>, y: i32) {
    let mut cur = start;
    water.insert(cur);

    while !water.contains(&(cur.0, cur.1 + 1)) && !walls.contains(&(cur.0, cur.1 + 1)) {
        if cur.1 >= y {
            return;
        }

        cur = (cur.0, cur.1 + 1);
        water.insert(cur);
    }

    let (mut lp, mut rp) = (cur,cur);

    if !walls.contains(&(cur.0, cur.1 + 1)) 
    && (!check_direction(walls, &mut water.clone(), &mut (cur.0, cur.1 + 1), 1) 
    || !check_direction(walls, &mut water.clone(), &mut (cur.0, cur.1 + 1), -1)) {
        return;
    }

    let right = check_direction(walls, water, &mut rp, 1);
    let left  = check_direction(walls, water, &mut lp, -1);

    if left && right {
        fill((cur.0, cur.1 - 1), walls, water, y);
    }
    if !left {
        if walls.contains(&(lp.0+1,lp.1+1)) {
            fill(lp, walls, water, y);
        } else {
            water.remove(&lp);
        }
    }
    if !right {
        if walls.contains(&(rp.0-1,rp.1+1)) {
            fill(rp, walls, water, y);
        } else {
            water.remove(&rp);
        }
    }
} 

fn check_direction(walls: &HashSet<(i32,i32)>, water: &mut HashSet<(i32,i32)>, pp: &mut (i32,i32), dir: i32) -> bool {
    let mut blocked = false;
    while !blocked {
        if walls.contains(&(pp.0 + dir,pp.1)) {
            blocked = true;
        } else {
            *pp = (pp.0 + dir, pp.1);
            water.insert(*pp);

            if !(walls.contains(&(pp.0, pp.1 + 1)) 
            || water.contains(&(pp.0, pp.1 + 1))) {
                break;
            }
        }
    }

    blocked
}

fn parse() -> (HashSet<(i32,i32)>, i32, i32) {
    let mut walls: HashSet<(i32,i32)> = HashSet::new();
    let (mut y, mut lowest) = (0i32,i32::MAX);

    for l in std::fs::read_to_string("../input/17").unwrap().lines() {
        let (c1,c2) = l.split_once(", ").unwrap();
        let (from,to) = c2[2..].split_once("..").unwrap();
        let fs: i32 = c1[2..].parse().unwrap();
        let rnge = (from.parse::<i32>().unwrap()..=to.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        match &c1[..2] {
            "x=" => rnge.iter().for_each(|c| { 
                lowest = lowest.min(*c); 
                walls.insert((fs, *c)); 
            }),
            "y=" => rnge.iter().for_each(|c| { 
                y = y.max(fs); 
                lowest = lowest.min(fs); 
                walls.insert((*c, fs)); 
            }),
            _ => panic!("invalid inp")
        };
    }

    (walls, lowest-1, y)
}

fn main() {
    let (walls, lowest, y) = parse();
    let mut water: HashSet<(i32,i32)> = HashSet::new();

    println!("Part 1: {}", part1(&walls, &mut water, lowest, y));
    println!("Part 2: {}", part2(&walls, &mut water));
}
