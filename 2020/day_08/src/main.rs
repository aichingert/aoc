fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

#[derive(Debug, Clone)]
struct Instruction {
    action: String,
    value: i32,
}

fn solve_part_one(input: &String) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut visited: Vec<usize> = Vec::new();
    let mut acc: i32 = 0;

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();

        let instruction: Instruction = Instruction { 
            action: values[0].to_string(), 
            value: values[1].parse::<i32>().unwrap(), 
        };

        instructions.push(instruction);
    }

    let mut pos: usize = 0;
    let mut jumped: bool;

    while !visited.contains(&pos) {
        visited.push(pos);
        jumped = false;

        match &instructions[pos].action as &str {
            "acc" => {
                acc += instructions[pos].value;
            },
            "jmp" => {
                jumped = true;
                
                if instructions[pos].value < 0 {
                    let mut value: i32 = instructions[pos].value;
                    while value < 0 {
                        pos -= 1;
                        value += 1;
                    }
                } else {
                    pos += instructions[pos].value as usize;
                }
            }
            _ => {}
        }

        if !jumped {
            pos += 1;
        }
    }

    println!("Solution part one: {}", acc);
}

fn solve_part_two(input: &String) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut visited: Vec<usize> = Vec::new();
    let mut acc: i32 = 0;

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();

        let instruction: Instruction = Instruction { 
            action: values[0].to_string(), 
            value: values[1].parse::<i32>().unwrap(), 
        };

        instructions.push(instruction);
    }

    let mut pos: usize;
    let mut jumped: bool;
    let mut normal_exit: bool = false;
    let mut changed_pos: Vec<usize> = Vec::new();
    let mut nops: bool = true;

    while !normal_exit {
        normal_exit = true;
        acc = 0;
        visited.clear();
        pos = 0;

        if nops {
            nops = false;

            for i in 0..instructions.len() {
                if &instructions[i].action as &str == "nop" && !changed_pos.contains(&i) {
                    instructions[i].action = "jmp".to_string();
                    changed_pos.push(i);
                    nops = true;
                    break;
                }
            }
        }

        if !nops {
            for i in 0..instructions.len() {
                if &instructions[i].action as &str == "jmp" && !changed_pos.contains(&i) {
                    instructions[i].action = "nop".to_string();
                    changed_pos.push(i);
                    break;
                }
            }
        }

        while !visited.contains(&pos) {
            visited.push(pos);
            jumped = false;

            if pos >= instructions.len() {
                visited.remove(visited.len() - 1);
                break;
            }
    
            match &instructions[pos].action as &str {
                "acc" => {
                    acc += instructions[pos].value;
                },
                "jmp" => {
                    jumped = true;
                    
                    if instructions[pos].value < 0 {
                        let mut value: i32 = instructions[pos].value;
                        while value < 0 {
                            pos -= 1;
                            value += 1;
                        }
                    } else {
                        pos += instructions[pos].value as usize;
                    }
                }
                _ => {}
            }
    
            if !jumped {
                pos += 1;
            }
        }

        if visited.contains(&pos) {
            normal_exit = false;
            if nops {
                instructions[changed_pos[changed_pos.len() - 1]].action = "nop".to_string();
            } else {
                instructions[changed_pos[changed_pos.len() - 1]].action = "jmp".to_string();
            }
        }
    }

    println!("Solution part two: {}", acc);
}