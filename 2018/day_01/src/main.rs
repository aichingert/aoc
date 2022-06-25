use std::{fs, collections::HashMap};

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut frequenzy: i32 = 0;

    for line in input.lines() {
        frequenzy += line.parse::<i32>().unwrap();
    }

    println!("Solution part one: {}", frequenzy);
}

fn solve_part_two(input: &String) {
    let mut frequenzy: i32 = 0;
    let mut values: HashMap<i32, bool> = HashMap::new();
    let mut contains: bool = false;

    while !values.contains_key(&frequenzy) {
        values.insert(frequenzy, false);

        for line in input.lines() {
            frequenzy += line.parse::<i32>().unwrap();

            if values.contains_key(&frequenzy) {
                contains = true;
                break;
            }

            values.insert(frequenzy, false);
        }

        if contains {
            break;
        }
        values.remove(&frequenzy);
    }

    println!("Solution part one: {}", frequenzy);
}