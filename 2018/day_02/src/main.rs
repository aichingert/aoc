use std::{fs, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut values: HashMap<char, i32> = HashMap::new();
    let mut first: i32 = 0;
    let mut second: i32 = 0;
    let mut is_f: bool;
    let mut is_s: bool;

    for line in input.lines() {
        is_f = false;
        is_s = false;
        for c in line.chars() {
            if values.contains_key(&c) {
                let count = values.remove(&c).unwrap();
                values.insert(c, count + 1);
            } else {
                values.insert(c, 1);
            }
        }

        for val in values.values() {
            if *val == 2 && !is_f {
                first += 1;
                is_f = true;
            } else if *val == 3 && !is_s {
                second += 1;
                is_s = true;
            }
        }

        values.clear();
    }

    println!("Solution part one: {}", first * second);
}

fn solve_part_two(input: &String) {
    let mut i = 0;
    let mut j = 0;
    let mut result: (i32, i32) = (0, 0);
    let mut ll: String = String::new();
    let mut found = false;
    let mut pos: i32 = 0;
    for line in input.lines() {
        for comp_line in input.lines() {
            if i != j {
                result = find_diff(line.to_string(), comp_line.to_string());

                if result.0 == 1 {
                    ll = line.to_string();
                    pos = result.1;
                    found = true;
                    break;
                }
            }
            j += 1;
        }

        if found {
            break;
        }

        j = 0;
        i += 1;
    }

    let mut k: i32 = 0;

    print!("Solution part one: \"");

    for c in ll.chars() {
        if k != pos {
            print!("{}", c);
        }

        k += 1;
    }

    print!("\"");
    println!();
}

fn find_diff(cur: String, check: String) -> (i32, i32) {
    let mut dif: i32 = 0;
    let mut pos: i32 = 0;
    if cur.len() != check.len() {
        return (dif, pos);
    }

    for i in 0..cur.len() {
        if cur.chars().nth(i) != check.chars().nth(i) {
            dif += 1;
            pos = i as i32;
        }
    }

    (dif, pos)
}