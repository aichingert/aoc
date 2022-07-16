use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut results: HashMap<usize, usize> = HashMap::new();

    while lines.len() > 0 {
        let mut mask: String = String::new();
        let mut commands: Vec<(usize, String)> = Vec::new();
        let mut second_mask: bool = false;

        for i in 0..lines.len() {
            let split: Vec<&str> = lines[i].split(" = ").collect();

            if split[0] == "mask" && !second_mask {
                mask = split[1].to_string();
                second_mask = true;
            } else if split[0] == "mask" && second_mask {
                break;
            } else {
                let step: Vec<&str> = split[0].split('[').collect();
                let mut to_convert: String = step[1].to_string();
                to_convert.pop();
                let address: usize = to_convert.parse().unwrap();

                let value: String = split[1].to_string();

                commands.push((address, value));
            }
        }

        for i in 0..commands.len() {
            results.insert(commands[i].0, get_result(&mask, &commands[i]));
        }

        for _ in 0..commands.len() + if lines.len() > commands.len() { 1 } else { 0 } {
            lines.remove(0);
        }
    }

    let mut result: usize = 0;

    for value in results.values() {
        result += *value;
    }

    println!("Solution part one: {}", result);
}

fn solve_part_two(input: &String) {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut results: HashMap<usize, usize> = HashMap::new();

    while lines.len() > 0 {
        let mut mask: String = String::new();
        let mut commands: Vec<(usize, String)> = Vec::new();
        let mut second_mask: bool = false;

        for i in 0..lines.len() {
            let split: Vec<&str> = lines[i].split(" = ").collect();

            if split[0] == "mask" && !second_mask {
                mask = split[1].to_string();
                second_mask = true;
            } else if split[0] == "mask" && second_mask {
                break;
            } else {
                let step: Vec<&str> = split[0].split('[').collect();
                let mut to_convert: String = step[1].to_string();
                to_convert.pop();
                let address: usize = to_convert.parse().unwrap();

                let value: String = split[1].to_string();

                commands.push((address, value));
            }
        }

        for i in 0..commands.len() {
            let addresses: Vec<usize> = get_addresses(&mask, &commands[i]);
            
            for j in 0..addresses.len() {
                results.insert(addresses[j], commands[i].1.parse::<usize>().unwrap());
            }
        }


        for _ in 0..commands.len() + if lines.len() > commands.len() { 1 } else { 0 } {
            lines.remove(0);
        }
    }

    let mut result: usize = 0;

    for value in results.values() {
        result += *value;
    }
    println!("Solution part two: {result}");
}

fn get_addresses(mask: &String, command: &(usize, String)) -> Vec<usize> {
    let mut addresses: Vec<usize> = Vec::new();
    let command_bin: String = to_binary(&command.0.to_string());

    let mut starting_address: Vec<char> = Vec::new();

    for i in 0..mask.len() {
        if mask.chars().nth(i).unwrap() == 'X' {
            starting_address.push('X');
        } else if mask.chars().nth(i).unwrap() == '1' || command_bin.chars().nth(i).unwrap() == '1' {
            starting_address.push('1');
        } else {
            starting_address.push('0');
        }
    }

    let mut amount_of_x: u32 = 0;
    let mut x_positions: Vec<usize> = Vec::new();
    let mut address_string: String = String::new();

    for i in 0..starting_address.len() {
        if starting_address[i] == 'X' {
            amount_of_x += 1;
            starting_address[i] = '0';
            x_positions.insert(0, i);
        }

        address_string.push(starting_address[i]);
    }

    addresses.push(to_decimal(&address_string));

    for _ in 0..2_usize.pow(amount_of_x) {
        address_string.clear();
        let mut binary_value: String = String::new();

        for i in 0..x_positions.len() {
            binary_value.push(starting_address[x_positions[i]]);
        }

        let decimal_value: usize = to_decimal(&binary_value);

        let new_values: Vec<char> = to_binary(&(decimal_value + 1).to_string()).chars().collect();

        for i in 0..x_positions.len() {
            starting_address[x_positions[i]] = new_values[new_values.len() - i - 1];
        }

        for i in 0..starting_address.len() {
            address_string.push(starting_address[i]);
        }

        addresses.push(to_decimal(&address_string));
    }

    addresses
}

fn get_result(mask: &String, command: &(usize, String)) -> usize {
    let command_bin: String = to_binary(&command.1);
    let mut resluting_bin: String = String::new(); 

    for i in 0..mask.len() {
        if mask.chars().nth(i).unwrap() == 'X' {
            resluting_bin.insert(0, command_bin.chars().nth(i).unwrap());
        } else {
            resluting_bin.insert(0, mask.chars().nth(i).unwrap());
        }
    }

    to_decimal(&resluting_bin)
}

fn to_binary(decimal: &String) -> String {
    let mut binary: String = String::new();
    let mut value: usize = decimal.parse().unwrap();

    while value > 0 {
        if value % 2 == 0 {
            binary.insert(0, '0');
        } else {
            binary.insert(0, '1');
        }

        value /= 2;
    }

    while binary.len() < 36 {
        binary.insert(0, '0');
    }

    binary
}

fn to_decimal(bin: &String) -> usize {
    let mut result: usize = 0;

    for i in 0..bin.len() {
        if bin.chars().nth(i).unwrap() == '1' {
            result += 2_usize.pow(i as u32) as usize;
        }
    }

    result
}