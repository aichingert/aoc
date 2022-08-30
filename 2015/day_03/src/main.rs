use std::{collections::HashMap, fs};

fn main() {
    let inp: String = fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
    solve_part_two(&inp);
}

fn solve_part_one(inp: &String) {
    let w: Vec<char> = inp.chars().collect();
    let mut m: Vec<(i32, i32)> = vec![(0,0)];
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut s: i32 = 1;

    for i in 0..w.len() {
        match w[i] {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => {}
        }

        if !m.contains(&(x, y)) {
            s += 1;
            m.push((x, y));
        }
    }

    println!("Solution part one: {}", s);
}

fn solve_part_two(inp: &String) {
    let w: Vec<char> = inp.chars().collect();
    let mut m: HashMap<(i32, i32), i32> = HashMap::new();
    m.insert((0, 0), 1);

    let mut fx: i32 = 0;
    let mut fy: i32 = 0;

    let mut sx: i32 = 0;
    let mut sy: i32 = 0;

    let mut a: i32 = 1;

    for i in 0..w.len() {
        if i % 2 == 0 {
            match w[i] {
                '<' => fx -= 1,
                '>' => fx += 1,
                '^' => fy += 1,
                'v' => fy -= 1,
                _ => {}
            }

            if !m.contains_key(&(fx, fy)) {
                a += 1;
                m.insert((fx, fy), 1);
            }
        
        } else {
            match w[i] {
                '<' => sx -= 1,
                '>' => sx += 1,
                '^' => sy += 1,
                'v' => sy -= 1,
                _ => {}
            }

            if !m.contains_key(&(sx, sy)) {
                a += 1;
                m.insert((sx, sy), 1);
            }
        }
    }

    println!("Solution part two: {}", a);
}