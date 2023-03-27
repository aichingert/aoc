// Advent of Code 2019, day 10
// (c) aichingert

fn part1(inp: &Vec<Vec<char>>) -> (usize, usize, Vec<(usize, usize)>) {
    let mut cords = Vec::<(usize,usize)>::new();

    for (i,e) in inp.iter().enumerate() {
        for (j,ch) in e.iter().enumerate() {
            if ch == &'#' { cords.push((i,j)); }
        }
    }

    let mut position = (0, 0);


    for i in 0..cords.len() {
        let mut points = Vec::new();

        for j in 0..cords.len() {
            if i == j { continue; }


            if !points.contains(&((cords[j].0 as f32 - cords[i].0 as f32).atan2(cords[j].1 as f32-cords[i].1 as f32))) {
                points.push((cords[j].0 as f32 - cords[i].0 as f32).atan2(cords[j].1 as f32-cords[i].1 as f32));
            }
        }

        if position.0 < points.len() {
            position = (points.len(), i);
        }
    }

    (position.0, position.1, cords)
}

fn part2(cords: &Vec<(usize,usize)>, best: usize) -> u32 {
    let mut angles = Vec::<((usize, usize), f64)>::new();

    for i in 0..cords.len() {
        if i == best { continue; }

        let mut angle = (cords[best].0 as f32 - cords[i].0 as f32).atan2(cords[best].1 as f32-cords[i].1 as f32) as f64 * 180.0 / 3.14159265358979323846264338327950288f64;
        if angle < 0.0 {
            angle = (angle * -1.0) + 180.0;
        }
        angles.push((((cords[i].0, cords[i].1)), angle));
    }

    let mut idx = 0;
    let mut ang = 90.0;
    let mut dist = 360.0;
    let mut dels = Vec::<(usize, usize)>::new();

    for _ in 0..200 {
        for i in 0..angles.len() {
            if angles[i].1 > ang && angles[i].1 - ang < dist {
                dist = angles[i].1 - ang;
                idx = i;
            }
        }

        ang = angles[idx].1;
        let mut close = i32::MAX;
        let mut rem = 0;
        let mut b = false;

        for i in 0..angles.len() {
            if angles[i].1 == ang && (angles[i].0.0 as i32 - cords[best].0 as i32).abs() + (angles[i].0.1 as i32 - cords[best].1 as i32).abs() < close {
                close = (angles[i].0.0 as i32 - cords[best].0 as i32).abs() + (angles[i].0.1 as i32 - cords[best].1 as i32).abs();
                rem = i;
            }

            if angles[i].1 > ang {
                b = true;
            }
        }

        dels.push((angles[rem].0.1, angles[rem].0.0));
        angles.remove(rem);
        dist = 360.0;
        if !b {
            ang = 0.0;
        }
    }

    (dels[198].0 * 100 + dels[198].1) as u32
}

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap().lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let (part1, loc, cords) = part1(&inp);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(&cords, loc));
}
