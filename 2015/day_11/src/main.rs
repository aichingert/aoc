fn main() {
    let input: String = "cqjxjnds".to_string();

     solve_part_one(&input);
}

fn solve_part_one(input: &String) {
    let mut values: Vec<u8> = Vec::new();

    for byte in input.as_bytes() {
        values.push(*byte);
    }

    while !is_valid(&values) {
        tick(&mut values);
    }

    print!("Solution part one: ");

    for value in &values {
        print!("{}", *value as char);
    }
    println!();

    
    solve_part_two(&mut values);
}

fn solve_part_two(values: &mut Vec<u8>) {
    tick(values);

    while !is_valid(values) {
        tick(values);
    }

    print!("Solution part two: ");

    for value in values {
        print!("{}", *value as char);
    }
}

fn tick(values: &mut Vec<u8>) {
    let len: usize   = values.len();

    for i in 1..values.len() {
        values[len - i ] += 1;

        if values[len - i] > 122 {
            values[len - i] = 97;
        } else {
            break;
        }
    }
}

fn is_valid(input: &Vec<u8>) -> bool {
    let mut is_valid: bool = false;

    for i in 0..input.len() - 2 {
        if input[i] + 1 == input[i + 1] && input[i] + 2 == input[i + 2] {
            is_valid = true;
            break;
        }
    }

    if !is_valid {
        return is_valid;
    }

    for i in 0..input.len() {
        if input[i] == 105 || input[i] == 111 || input[i] == 108 {
            return false;
        }
    }

    is_valid = false;
    let mut appear_byte: u8 = 0;

    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] {
            is_valid = true;
            appear_byte = input[i];
            break;
        }
    }

    if !is_valid {
        return false;
    }

    is_valid = false;

    for i in 0..input.len() - 1 {
        if input[i] == input[i + 1] && input[i] != appear_byte {
            is_valid = true;
        }
    }

    is_valid
}