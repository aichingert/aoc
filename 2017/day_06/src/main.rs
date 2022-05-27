use std::{fs, collections::HashMap};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("err");
    let storing: Vec<_> = contents.split('\t').collect();
    let mut scenarios: Vec<Vec<i32>> = Vec::new();
    let mut numbers: Vec<i32> = Vec::new();
    let mut idx_big: (usize, i32) = (0, 0);
    let mut steps: i32 = 0;
    let mut idx: HashMap<Vec<i32>, i32> = HashMap::new();

    for number in storing {
        numbers.push(number.trim().parse().unwrap());
    }

    while !scenarios.contains(&numbers) {
        scenarios.push(numbers.clone());
        idx.insert(numbers.clone(), steps);
        idx_big.0 = 0;
        idx_big.1 = 0;

        for i in 0..numbers.len() {
            if numbers[i] > idx_big.1 {
                idx_big.0 = i;
                idx_big.1 = numbers[i];
            }
        }

        let mut index: usize = idx_big.0;
        numbers[idx_big.0] = 0;

        for _ in 0..idx_big.1 {
            if index + 1 as usize >= numbers.len() {
                index = 0;
            }
            else {
                index += 1;
            }

            numbers[index] += 1;
        }

        steps += 1;
    }

    println!("{:?}", steps);
    println!("{:?}", numbers);
    println!("{}", (idx[&numbers] - steps).abs());
}