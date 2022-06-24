use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Error reading file help!");
    
    let mut values: HashMap<&str, u16> = HashMap::new();
    let mut lines: Vec<&str> = Vec::new();
    let mut rem_index: Vec<usize> = Vec::new();
    let mut to_remove;

    //Reading values with number
    for line in content.lines() {
        let vals: Vec<_> = line.split(' ').collect();

        if is_number(vals[0]) && vals.len() == 3 {
            if vals[1] == "->" {
                values.insert(vals[2], vals[0].parse().unwrap());
            }
        }
        else {
            lines.push(line);
        }
    }


    while !values.contains_key("a") {
        let mut counter:usize = 0;
        to_remove = false;

        for line in lines.iter_mut() {
            let vals: Vec<_> = line.split(' ').collect();
    
            if vals[0] == "NOT" {
                if values.contains_key(vals[1]) {
                    values.insert(vals[3], !values[vals[1]]);
                    rem_index.push(counter);
                    to_remove = true;
                }

                if is_number(vals[2]) {
                    let not_value: u16 = vals[1].parse().unwrap();
                    values.insert(vals[3], !not_value);
                    rem_index.push(counter);
                    to_remove = true;
                }
            }
            else {
                if values.contains_key(vals[0]) && is_number(vals[2]) {
                    values.insert(vals[4], know_first(vals[1], values[vals[0]], vals[2]));
                    rem_index.push(counter);
                    to_remove = true;
                }
                else if values.contains_key(vals[2]) && is_number(vals[0]) {
                    values.insert(vals[4], know_second(vals[1], vals[0], values[vals[2]])); 
                    rem_index.push(counter);
                    to_remove = true;
                }
                else if values.contains_key(vals[0]) && values.contains_key(vals[2]) {
                    values.insert(vals[4], know_both(vals[1], values[vals[0]], values[vals[2]]));
                    rem_index.push(counter);
                    to_remove = true;
                }
            }

            if vals.len() == 3 {
                if !values.contains_key(vals[0]) && values.contains_key(vals[2]) {
                    values.insert(vals[0], vals[2].parse().unwrap());
                }
                else if values.contains_key(vals[0]) && !values.contains_key(vals[2]) {
                    values.insert(vals[2], values[vals[0]]);
                }
            }

            counter += 1;
        }

        if to_remove {
            let mut cunt = 0;
            for index in rem_index.iter() {
                lines.remove(*index - cunt);
                cunt += 1;
            }
        }

        rem_index.clear();
    }


    println!("\na: {}", values["a"]);
}

fn is_number(value: &str) -> bool {
    let mut is_valid = false;

    for chur in value.chars() {
        if chur.is_numeric() {
            is_valid = true;
        }
        else {
            is_valid = false;
            break;
        }
    }

    is_valid
}

fn know_first(statement: &str, value_one: u16, value_two: &str) -> u16 {
    let value_of_two: u16 = value_two.parse().unwrap();

    match statement {
        "AND" => return value_one & value_of_two,
        "LSHIFT" => return value_one << value_of_two,
        "RSHIFT" => return value_one >> value_of_two,
        "OR" => return value_one | value_of_two,
        _ => 0,
    }
}

fn know_second(statement: &str, value_one: &str, value_two: u16) -> u16 {
    let value_of_one: u16 = value_one.parse().unwrap();

    match statement {
        "AND" => return value_of_one & value_two,
        "LSHIFT" => return value_of_one << value_two,
        "RSHIFT" => return value_of_one >> value_two,
        "OR" => return value_of_one | value_two,
        _ => 0,
    }
}

fn know_both(statement: &str, value_one: u16, value_two: u16) -> u16 {
    match statement {
        "AND" => return value_one & value_two,
        "LSHIFT" => return value_one << value_two,
        "RSHIFT" => return value_one >> value_two,
        "OR" => return value_one | value_two,
        _ => 0,
    }
}