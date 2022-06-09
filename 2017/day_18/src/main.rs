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
                } else {
                    if instructions[idx][1].parse::<i64>().unwrap() > 0 {
                        let value: i64;

                        value = instructions[idx][2].parse::<i64>().unwrap();

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

    let mut values_program_one: HashMap<String, (i64, bool)> = HashMap::new();
    let mut values_program_two: HashMap<String, (i64, bool)> = HashMap::new();

    let mut sended: Vec<i64> = Vec::new();
    let mut sent: Vec<i64> = Vec::new();

    for line in input.lines() {
        instructions.push(line.split(' ').collect());
    }

    for c in 'a'..='z' {
        if c != 'p' {
            values_program_one.insert(c.to_string(), (0, false));
        } else {
            values_program_one.insert("p".to_string(), (0, true));
        }
    }

    for c in 'a'..='z' {
        if c != 'p' {
            values_program_two.insert(c.to_string(), (0, false));
        } else {
            values_program_two.insert("p".to_string(), (1, true));
        }
    }

    sent = cycle(&mut values_program_one, &instructions, &mut sended);
    sended = cycle(&mut values_program_two, &instructions, &mut sent);
    
    println!("Solution part two: {}", sended.len());
}

fn cycle(program_values: &mut HashMap<String, (i64, bool)>, instructions: &Vec<Vec<&str>>, received: &mut Vec<i64>) -> Vec<i64> {
    let mut sended: Vec<i64> = Vec::new();
    let mut idx = 0;

    while idx < instructions.len() {
        match instructions[idx][0] {
            "snd" => {
                if program_values.contains_key(instructions[idx][1]) && program_values[instructions[idx][1]].1 {
                    sended.push(program_values[instructions[idx][1]].0);
                } else {
                    sended.push(instructions[idx][1].parse::<i64>().unwrap());
                }
            },
            "set" => {
                if program_values.contains_key(instructions[idx][1]) {
                    program_values.remove(instructions[idx][1]).unwrap();
                }

                if program_values.contains_key(instructions[idx][2]) {
                    program_values.insert(instructions[idx][1].to_string(), program_values[instructions[idx][2]]);
                } else {
                    program_values.insert(instructions[idx][1].to_string(), (instructions[idx][2].parse::<i64>().unwrap(), true));
                }
            },
            "add" => {
                let mut value: i64;

                if program_values.contains_key(instructions[idx][2]) {
                    value = program_values[instructions[idx][2]].0;
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if program_values.contains_key(instructions[idx][1]) {
                    value += program_values.remove(instructions[idx][1]).unwrap().0;
                } 

                program_values.insert(instructions[idx][1].to_string(), (value, true));
            },
            "mul" => {
                let mut value: i64;

                if program_values.contains_key(instructions[idx][2]) {
                    value = program_values[instructions[idx][2]].0;
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if program_values.contains_key(instructions[idx][1]) {
                    value *= program_values.remove(instructions[idx][1]).unwrap().0;
                } 

                program_values.insert(instructions[idx][1].to_string(), (value, true));
            },
            "mod" => {
                let mut value: i64;

                if program_values.contains_key(instructions[idx][2]) {
                    value = program_values[instructions[idx][2]].0;
                } else {
                    value = instructions[idx][2].parse::<i64>().unwrap();
                }

                if program_values.contains_key(instructions[idx][1]) {
                    value = program_values.remove(instructions[idx][1]).unwrap().0 % value;
                } 

                program_values.insert(instructions[idx][1].to_string(), (value, true));
            },
            "rcv" => {
                if received.len() == 0 {
                    break;
                }

                program_values.remove(instructions[idx][1]);
                program_values.insert(instructions[idx][1].to_string(), (received.remove(0), true));
            },
            "jgz" => {
                let a = instructions[idx][1].parse::<i64>();

                if a.is_ok() && a.unwrap() > 0 {
                    let value: i64;

                    value = instructions[idx][2].parse::<i64>().unwrap();

                    if value > 0 {
                        idx += value as usize;
                    } else {
                        idx -= value.abs() as usize;
                    }

                    continue;
                }

                if program_values[instructions[idx][1]].1 {
                    if program_values[instructions[idx][1]].0 > 0 {
                        let value: i64;

                        if program_values.contains_key(instructions[idx][2]) {
                            value = program_values[instructions[idx][2]].0;
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

    sended
}