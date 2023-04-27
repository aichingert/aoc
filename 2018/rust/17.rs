// Advent of Code 2018, day 17
// (c) aichingert

use std::collections::HashSet;

fn part1(walls: &HashSet<(i32,i32)>, lowest: i32, y: i32) -> usize {
    let start = (500,0);
    let mut water: HashSet<(i32,i32)> = HashSet::new();
    
    fill(start, walls, &mut water, y);
    pr(walls, &water);

    water.iter().filter(|(x,y)| *y > lowest).count()
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

    match (left,right) {
        (true,true) => fill((cur.0, cur.1 - 1), walls, water, y),
        (true,false) => if walls.contains(&(rp.0-1,rp.1+1)) {
                fill(rp, walls, water, y);
            } else {
                water.remove(&rp);
            },
        (false,true) => if walls.contains(&(lp.0+1,lp.1+1)) {
                fill(lp, walls, water, y);
            } else {
                water.remove(&lp);
            },
        (false,false) => {
            if walls.contains(&(rp.0-1,rp.1+1)) {
                fill(rp, walls, water, y);
            } else {
                water.remove(&rp);
            }
            if walls.contains(&(lp.0+1,lp.1+1)) {
                fill(lp, walls, water, y);
            } else {
                water.remove(&lp);
            }
        },
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

fn pr(walls: &HashSet<(i32,i32)>, water: &HashSet<(i32,i32)>) {
    for i in 0..1915 {
        for j in 330..650 {
            if water.contains(&(j,i)) {
                print!("|");
            } else if walls.contains(&(j,i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
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
    println!("{:?}", lowest);

    println!("Part 1: {}", part1(&walls, lowest, y));
}
