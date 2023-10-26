use std::collections::{HashSet, HashMap};

type Loc = (usize, usize);

fn around(loc: Loc, map: &Vec<Vec<char>>) -> (Loc, Option<Loc>) {
    let mut second = (0, 0);
    let mut entrance = None;
    for r in -1..2 {
        for c in -1..2 {
            if r == 0 && c == 0 { continue; }
            let (nr, nc) = (loc.0 as i32 + r, loc.1 as i32 + c);
            if nr < 0 || nc < 0 || nr >= map.len() as i32 || nc >= map[loc.0].len() as i32 { continue; }
            let (nr, nc) = (nr as usize, nc as usize);

            match map[nr][nc] => {
                'A'..='Z' => second = (nr, nc),
                '.' => entrance = Some((nr, nc)),
                _ => (),
            }
            print!("{}", map[nr][nc]);
        }
        println!();
    }

    (second, entrance)
}

fn main() {
    let inp = std::fs::read_to_string("../input/20").unwrap();

    let mut walls = HashSet::new();

    let map = inp.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '#' => { walls.insert((i, j)); }
                'A'..='Z' => {
                    println!("CUR: {}", map[i][j]);
                    let (sec, to) = around((i,j), &map);
                }
                _ => (),
            }
        }
    }
}
