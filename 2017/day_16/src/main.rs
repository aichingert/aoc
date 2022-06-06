use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(input);
}

fn solve_part_one(input: &String) {
    let dance_moves: Vec<_> = input.split(',').collect();
    let mut programs: Vec<char> = Vec::new();

    for c in 'a'..='p' {
        programs.push(c)
    }

    for dance_move in &dance_moves {
        make_move(dance_move.to_string(), &mut programs);
    }

    print!("Solution part 1: ");
    for c in programs {
        print!("{}", c);
    }
    println!("");
}

fn solve_part_two(input: String) {
    let dance_moves: Vec<_> = input.split(',').collect();
    let mut programs: Vec<char> = Vec::new();

    for c in 'a'..='p' {
        programs.push(c)
    }

    let mut blub: HashMap<Vec<char>, i32> = HashMap::new();
    let mut modulo: i32 = 0;

    for i in 0..200 {
        for dance_move in &dance_moves {
            make_move(dance_move.to_string(), &mut programs);
        }

        if !blub.contains_key(&programs) {
            blub.insert(programs.clone(), i);
        } else {
            modulo = i;
            break;
        }
    }

    programs.clear();

    for c in 'a'..='p' {
        programs.push(c)
    }

    for _ in 0..1000000000 % modulo {
        for dance_move in &dance_moves {
            make_move(dance_move.to_string(), &mut programs);
        }
    }
    
    print!("Solution part 2: ");
    for c in programs {
        print!("{}", c);
    }
    println!("");
}

fn make_move(dance_move: String, programs: &mut Vec<char>) {
    let mut values: Vec<_> = dance_move.chars().collect();
    let action: char = values.remove(0);
    let mut from: usize = 0;
    let mut to: usize = 0;

    if dance_move.contains('/') {
        let mut numbers: Vec<String> = dance_move.split('/').map( | x | {
            x.to_string()
        }).collect();
        let a = numbers[0].remove(0);

        if a == 'x' {
            from = numbers[0].parse::<usize>().unwrap();
            to = numbers[1].parse::<usize>().unwrap();
        }
    } else {
        let mut number: String = String::new();
        for c in &values {
            number.push(*c);
        }
        to = number.parse::<usize>().unwrap();
    }

    let mut change: Vec<char> = programs.clone();

    match action {
        's' => {
            for _ in 0..to {
                let element: char = change.pop().unwrap();
                change.insert(0, element);
            }
        },
        'x' => {
            let mut new: Vec<char> = Vec::new();

            for i in 0..change.len() {
                if i == from {
                    new.push(change[to])
                } else if i == to {
                    new.push(change[from])
                } else {
                    new.push(change[i])
                }
            }

            change = new;
        },
        'p' => {
            let mut new: Vec<char> = Vec::new();

            for c in &change {
                if c == &values[0] {
                    new.push(values[2])
                } else if c == &values[2] {
                    new.push(values[0])
                } else {
                    new.push(*c);
                }
            }

            change = new;
        },
        _ => {}
    }

    *programs = change;
}
