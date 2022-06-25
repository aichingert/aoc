fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut numbers: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for number in input.chars() {
        numbers.push((number as u8 - '0' as u8) as i32);
    }

    for i in 0..numbers.len() {
        if i == numbers.len() - 1 {
            if numbers[i] == numbers[0] {
                sum += numbers[i];
            }
        } else if numbers[i] == numbers[i + 1] {
            sum += numbers[i];
        }
    }

    println!("Solution part one: {}", sum);
}

fn solve_part_two(input: &String) {
    let mut numbers: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    for number in input.chars() {
        numbers.push((number as u8 - '0' as u8) as i32);
    }

    let steps: usize = numbers.len() / 2;

    for i in 0..numbers.len() {
        if i + steps >= numbers.len() {
            if numbers[i] == numbers[i - steps] {
                sum += numbers[i];
            }
        } else if numbers[i] == numbers[i + steps] {
            sum += numbers[i];
        }
    }

    println!("Solution part two: {}", sum);
}