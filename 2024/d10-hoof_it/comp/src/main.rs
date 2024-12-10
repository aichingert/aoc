use std::collections::{HashMap, HashSet};

fn p1(r: &mut HashSet<(i32, i32)>, y: i32, x: i32, g: &[Vec<char>]) -> i32 { 
   if !r.insert((y, x)) {
        return 0;
    } 
   if g[y as usize][x as usize] == '9' {
        return 1;
    }

    let mut res = 0;

    for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let (dy, dx) = (y + dy, x + dx);

        if dy < 0 || dy >= g.len() as i32 
            || dx < 0 || dx >= g[y as usize].len() as i32 {
            continue;
        }

        if g[y as usize][x as usize] as u8 + 1 == g[dy as usize][dx as usize] as u8 {
            res += p1(r, dy, dx, g);
        }
    }

    res
}

fn p2(r: &mut HashMap<(i32, i32), i32>, y: i32, x: i32, g: &[Vec<char>]) -> i32 { 
    if let Some(&a) = r.get(&(y, x)) {
        return a;
    }
    if g[y as usize][x as usize] == '9' {
        return 1;
    }

    let mut res = 0;

    for (dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let (dy, dx) = (y + dy, x + dx);

        if dy < 0 || dy >= g.len() as i32 
            || dx < 0 || dx >= g[y as usize].len() as i32 {
            continue;
        }

        if g[y as usize][x as usize] as u8 + 1 == g[dy as usize][dx as usize] as u8 {
            res += p2(r, dy, dx, g);
        }
    }

    r.insert((y, x), res);
    res
}

fn main() {
    let g = std::fs::read_to_string("../../../input/2024/10")
        .unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut a1 = 0;
    let mut a2 = 0;
    let mut h = HashMap::new();

    for i in 0..g.len() {
        for j in 0..g[i].len() {
            if g[i][j] == '0' {
                a1 += p1(&mut HashSet::new(), i as i32, j as i32, &g);
                a2 += p2(&mut h, i as i32, j as i32, &g);
            }
        }
    }

    println!("p1: {a1}");
    println!("p2: {a2}");
}
