use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(input);
}

fn solve_part_one(input: &String) {
    let mut instructions: Vec<Vec<&str>> = Vec::new();
    let mut values: HashMap<String, i64> = HashMap::new();
    let mut recovery_values: HashMap<String, i64> = HashMap::new();

    let mut idx: usize = 0;

    for line in input.lines() {
        instructions.push(line.split(' ').collect());
    }

    for c in 'a'..'z' {
        values.insert(c.to_string(), 0);
    }

    while idx < instructions.len() {
        match instructions[idx][0] {
            "snd" => {
                if values.contains_key(instructions[idx][1]) {
                    recovery_values.insert(instructions[idx][1].to_string(), values[instructions[idx][1]]);
                }
            },
            "set" => {
                if values.contains_key(instructions[idx][1]) {
                    values.remove(instructions[idx][1]).unwrap();
                }

                if values.contains_key(instructions[idx][2]) {
                    values.insert(instructions[idx][1].to_string(), values[instructions[idx][2]]);
                } else {
                    values.insert(instructions[idx][1].to_string(), instructions[idx][2].parse::<i64>().unwrap());
                }
            },
            "add" => {
                let mut value: i64;

                if values.contains_key(instructions[idx][2]) {
                    value = values[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values.contains_key(instructions[idx][1]) {
                    value += values.remove(instructions[idx][1]).unwrap();
                } 

                values.insert(instructions[idx][1].to_string(), value);
            },
            "mul" => {
                let mut value: i64;

                if values.contains_key(instructions[idx][2]) {
                    value = values[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values.contains_key(instructions[idx][1]) {
                    value *= values.remove(instructions[idx][1]).unwrap();
                } 

                values.insert(instructions[idx][1].to_string(), value);
            },
            "mod" => {
                let mut value: i64;

                if values.contains_key(instructions[idx][2]) {
                    value = values[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values.contains_key(instructions[idx][1]) {
                    value = values.remove(instructions[idx][1]).unwrap() % value;
                } 

                values.insert(instructions[idx][1].to_string(), value);
            },
            "rcv" => {
                if values.contains_key(instructions[idx][1]) {
                    if values[instructions[idx][1]] != 0 && recovery_values.contains_key(instructions[idx][1]) {
                        println!("Solution part one: {}", recovery_values[instructions[idx][1]]);
                        return;
                    }
                }
            },
            "jgz" => {
                if values.contains_key(instructions[idx][1]) {
                    if values[instructions[idx][1]] > 0 {
                        let value: i64;

                        if values.contains_key(instructions[idx][2]) {
                            value = values[instructions[idx][2]];
                        } else {
                            value = instructions[idx][2].parse::<i64>().unwrap();
                        }

                        if value > 0 {
                            idx += value as usize;
                        } else {
                            idx -= value.abs() as usize;
                        }
 
                        continue;
                    }
                }

            },
            _ => {}
        }

        idx += 1;
    }
}

fn solve_part_two(input: String) {
    let mut instructions: Vec<Vec<&str>> = Vec::new();

    let mut values_program_one: HashMap<String, i64> = HashMap::new();
    let mut values_program_two: HashMap<String, i64> = HashMap::new();

    let mut sended: Vec<i64> = Vec::new();

    let mut idx: usize = 0;

    for line in input.lines() {
        instructions.push(line.split(' ').collect());
    }

    for c in 'a'..'z' {
        values_program_one.insert(c.to_string(), 0);
    }

    while idx < instructions.len() {
        match instructions[idx][0] {
            "snd" => {
                if values_program_one.contains_key(instructions[idx][1]) {
                    sended.push(values_program_one[instructions[idx][1]]);
                } else {
                    sended.push(instructions[idx][1].parse::<i64>().unwrap());
                }
            },
            "set" => {
                if values_program_one.contains_key(instructions[idx][1]) {
                    values_program_one.remove(instructions[idx][1]).unwrap();
                }

                if values_program_one.contains_key(instructions[idx][2]) {
                    values_program_one.insert(instructions[idx][1].to_string(), values_program_one[instructions[idx][2]]);
                } else {
                    values_program_one.insert(instructions[idx][1].to_string(), instructions[idx][2].parse::<i64>().unwrap());
                }
            },
            "add" => {
                let mut value: i64;

                if values_program_one.contains_key(instructions[idx][2]) {
                    value = values_program_one[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values_program_one.contains_key(instructions[idx][1]) {
                    value += values_program_one.remove(instructions[idx][1]).unwrap();
                } 

                values_program_one.insert(instructions[idx][1].to_string(), value);
            },
            "mul" => {
                let mut value: i64;

                if values_program_one.contains_key(instructions[idx][2]) {
                    value = values_program_one[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values_program_one.contains_key(instructions[idx][1]) {
                    value *= values_program_one.remove(instructions[idx][1]).unwrap();
                } 

                values_program_one.insert(instructions[idx][1].to_string(), value);
            },
            "mod" => {
                let mut value: i64;

                if values_program_one.contains_key(instructions[idx][2]) {
                    value = values_program_one[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values_program_one.contains_key(instructions[idx][1]) {
                    value = values_program_one.remove(instructions[idx][1]).unwrap() % value;
                } 

                values_program_one.insert(instructions[idx][1].to_string(), value);
            },
            "rcv" => {
                break;
            },
            "jgz" => {
                if values_program_one.contains_key(instructions[idx][1]) {
                    if values_program_one[instructions[idx][1]] > 0 {
                        let value: i64;

                        if values_program_one.contains_key(instructions[idx][2]) {
                            value = values_program_one[instructions[idx][2]];
                        } else {
                            value = instructions[idx][2].parse::<i64>().unwrap();
                        }

                        if value > 0 {
                            idx += value as usize;
                        } else {
                            idx -= value.abs() as usize;
                        }
 
                        continue;
                    }
                }

            },
            _ => {}
        }

        idx += 1;
    }
     
    println!("{:?}", sended.len());

    for c in 'a'..='z' {
        if c != 'p' {
            values_program_two.insert(c.to_string(), 0);
        } else {
            values_program_two.insert(c.to_string(), 1);
        }
    }

    let mut send_count = 0;
    let mut letter_count: i32 = 0;
    idx = 0;

    while idx < instructions.len() {
        match instructions[idx][0] {
            "snd" => {
                send_count += 1;
            },
            "set" => {
                if values_program_two.contains_key(instructions[idx][1]) {
                    values_program_two.remove(instructions[idx][1]).unwrap();
                }

                if values_program_two.contains_key(instructions[idx][2]) {
                    values_program_two.insert(instructions[idx][1].to_string(), values_program_two[instructions[idx][2]]);
                } else {
                    values_program_two.insert(instructions[idx][1].to_string(), instructions[idx][2].parse::<i64>().unwrap());
                }
            },
            "add" => {
                let mut value: i64;

                if values_program_two.contains_key(instructions[idx][2]) {
                    value = values_program_two[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values_program_two.contains_key(instructions[idx][1]) {
                    value += values_program_two.remove(instructions[idx][1]).unwrap();
                } 

                values_program_two.insert(instructions[idx][1].to_string(), value);
            },
            "mul" => {
                let mut value: i64;

                if values_program_two.contains_key(instructions[idx][2]) {
                    value = values_program_two[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values_program_two.contains_key(instructions[idx][1]) {
                    value *= values_program_two.remove(instructions[idx][1]).unwrap();
                } 

                values_program_two.insert(instructions[idx][1].to_string(), value);
            },
            "mod" => {
                let mut value: i64;

                if values_program_two.contains_key(instructions[idx][2]) {
                    value = values_program_two[instructions[idx][2]];
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if values_program_two.contains_key(instructions[idx][1]) {
                    value = values_program_two.remove(instructions[idx][1]).unwrap() % value;
                } 

                values_program_two.insert(instructions[idx][1].to_string(), value);
            },
            "rcv" => {
                let mut idx: i32 = 0;
                let mut value: String = String::new();

                for c in 'a'..'z' {
                    if idx == letter_count {
                        value = c.to_string();
                        break;
                    }

                    idx += 1;
                }

                if sended.len() == 0 {
                    break;
                } else {
                    println!("{:?}", sended[0]);
                }

                let mut number: i64 = values_program_two.remove(&value).unwrap();
                number += sended.remove(0);
                values_program_two.insert(value, number);
                letter_count += 1;

                if letter_count > 24 {
                    letter_count = 0;
                }
            },
            "jgz" => {
                if values_program_two.contains_key(instructions[idx][1]) {
                    if values_program_two[instructions[idx][1]] > 0 {
                        let value: i64;

                        if values_program_two.contains_key(instructions[idx][2]) {
                            value = values_program_two[instructions[idx][2]];
                        } else {
                            value = instructions[idx][2].parse::<i64>().unwrap();
                        }

                        if value > 0 {
                            idx += value as usize;
                        } else {
                            idx -= value.abs() as usize;
                        }
 
                        continue;
                    }
                }

            },
            _ => {}
        }

        idx += 1;
    }
    
    println!("Solution part two: {}", send_count);
}