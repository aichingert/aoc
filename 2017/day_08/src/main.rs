use std::{fs, collections::HashMap};

fn main() {
    let content = fs::read_to_string("input.txt").expect("err");
    solve_part_one(content); // Included part 2
}

fn solve_part_one(content: String) {
    let mut variables: HashMap<String, i32> = HashMap::new();
    let mut highest_number: i32 = 0;

    for line in content.lines() {
        let split: Vec<_> = line.trim().split(" if ").collect();

        let action: Vec<_> = split[0].split(' ').collect();
        let condition: Vec<_> = split[1].split(' ').collect();

        let var = condition[0].to_string();

        let variable = action[0].to_string();
        let idx: String = variable.to_string();
        let act = action[1];
        let value = action[2];

        if !variables.contains_key(&variable) {
            variables.insert(variable, 0);

            update(&mut variables, condition, idx, var, act, value, &mut highest_number);
        }
        else {
            update(&mut variables, condition, idx, var, act, value, &mut highest_number);
        }
    }

    let mut biggest_register: i32 = 0;

    for value in variables.values() {
        if value > &biggest_register {
            biggest_register = *value;
        }
    }

    println!("Solution part 1: {}", biggest_register);
    println!("Solution part 2: {}", highest_number);
}

fn condition_valid(condition: Vec<&str>, number: i32) -> bool {
    let variable: i32 = number;
    let operator: &str = condition[1];
    let value: &str = condition[2];

    match operator {
        ">" => {
            if variable > value.parse::<i32>().unwrap() {
                return true;
            }
        },
        "<" => {
            if variable < value.parse::<i32>().unwrap() {
                return true;
            }
        },
        ">=" => {
            if variable >= value.parse::<i32>().unwrap() {
                return true;
            }
        },
        "<=" => {
            if variable <= value.parse::<i32>().unwrap() {
                return true;
            }
        },
        "==" => {
            if variable == value.parse::<i32>().unwrap() {
                return true;
            }
        },
        "!=" => {
            if variable != value.parse::<i32>().unwrap() {
                return true;
            }
        },
        _ => {
            println!("{:?}", condition);
        }
    }

    false
}

fn update(variables: &mut HashMap<String, i32>, condition: Vec<&str>, variable: String, var: String, act: &str, value: &str, highest_number: &mut i32) {
    if !variables.contains_key(&var) {
        variables.insert(var, 0);
    }

    let idx = condition[0].clone();

    if condition_valid(condition, variables[idx]) {
        match act {
            "inc" => {
                let mut number = variables.remove(&variable).unwrap();
                number += value.parse::<i32>().unwrap();

                if number > *highest_number {
                    *highest_number = number;
                }

                variables.insert(variable, number);
            },
            "dec" => {
                let mut number = variables.remove(&variable).unwrap();
                number -= value.parse::<i32>().unwrap();

                if number > *highest_number {
                    *highest_number = number;
                }

                variables.insert(variable, number);
            },
            _ => {
                println!("{:?}", act);
            }
        }
    }
}