use std::collections::HashMap;

fn main() {
    let input: Vec<i32> = vec![16,1,0,18,12,14,19];

    solve(&input);
}

fn solve(input: &Vec<i32>) {
    let mut m: HashMap<i32, i32> = HashMap::new();
    let mut l: i32 = 0;
    let s: i32 = input.len() as i32 + 2;

    for i in 0..input.len() {
        m.insert(input[i], i as i32 + 1);
    }

    for i in s..=2020 {
        if m.contains_key(&l) {
            let h: i32 = m.remove(&l).unwrap();
            let v: i32 = i - 1;
            m.insert(l, v);

            l = v - h;
        } else {
            m.insert(l, i - 1);
            l = 0;
        }
    }

    println!("Solution part one: {}", l);

    m.clear();

    for i in 0..input.len() {
        m.insert(input[i], i as i32 + 1);
    }

    l = 0;

    for i in s..=30000000 {
        if m.contains_key(&l) {
            let h: i32 = m.remove(&l).unwrap();
            let v: i32 = i - 1;
            m.insert(l, v);

            l = v - h;
        } else {
            m.insert(l, i - 1);
            l = 0;
        }
    }

    println!("Solution part two: {}", l);
}