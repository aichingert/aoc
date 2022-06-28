fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut sum: i32 = 0;
    let mut to_negate: i32;
    let mut count = 0;
    let mut is_previous_backslash: bool;

    for line in input.lines() {
        sum += line.len() as i32;
        is_previous_backslash = false;
        to_negate = 0;

        for c in line.chars() {
            if count != 0 {
                count -= 1;
                continue;
            }

            match c {
                '\\' => {
                    if is_previous_backslash {
                        to_negate += 1;
                        is_previous_backslash = false;
                    } else {
                        is_previous_backslash = true;
                    }
                },
                '\"' => {
                    if is_previous_backslash {
                        to_negate += 1;
                        is_previous_backslash = false;
                    }
                }
                'x' => {
                    if is_previous_backslash {
                        count = 2;
                        to_negate += 1;
                        is_previous_backslash = false;
                    } else {
                        to_negate += 1;
                    }
                }
                _ => {
                    to_negate += 1;
                    is_previous_backslash = false;
                }
            }
        }
        
        sum -= to_negate;
    }   

    println!("Solution part one: {}", sum);
}

fn solve_part_two(input: &String) {
    let mut sum: i32 = 0;

    for line in input.lines() {
        sum += 2 - line.len() as i32;

        for c in line.chars() {
            match c {
                '\"' => {
                    sum += 2
                },
                '\\' => {
                    sum += 2
                }
                _ => {
                    sum += 1
                }
            }
        }
    }

    println!("Solution part two: {}", sum);
}