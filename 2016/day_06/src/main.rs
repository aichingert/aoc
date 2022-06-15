use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut rows: Vec<Vec<char>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut idx: usize = 0;

    for line in input.lines() {
        for c in line.chars() {
            rows[idx].push(c);
            idx += 1;
        }

        idx = 0;
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    let mut result: String = String::new();
    let mut biggest: usize = 0;
    let mut current_char: char = ' ';

    for row in rows {
        for c in row {
            if map.contains_key(&c) {
                let val = map.remove(&c).unwrap();
                map.insert(c, val + 1);
            } else {
                map.insert(c, 1);
            }
        }

        for key in map.keys() {
            if map[key] > biggest {
                biggest = map[key];
                current_char = *key;
            }
        }

        map.clear();
        result.push(current_char);
        biggest = 0;
        current_char = ' ';
    }

    println!("Solution part one: {}", result);
} 

fn solve_part_two(input: &String) {
    let mut rows: Vec<Vec<char>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut idx: usize = 0;

    for line in input.lines() {
        for c in line.chars() {
            rows[idx].push(c);
            idx += 1;
        }

        idx = 0;
    }

    let mut map: HashMap<char, usize> = HashMap::new();
    let mut result: String = String::new();
    let mut biggest: usize = 10000;
    let mut current_char: char = ' ';

    for row in rows {
        for c in row {
            if map.contains_key(&c) {
                let val = map.remove(&c).unwrap();
                map.insert(c, val + 1);
            } else {
                map.insert(c, 1);
            }
        }

        for key in map.keys() {
            if map[key] < biggest {
                biggest = map[key];
                current_char = *key;
            }
        }

        map.clear();
        result.push(current_char);
        biggest = 10000000;
        current_char = ' ';
    }

    println!("Solution part one: {}", result);
}