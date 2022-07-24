fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

// Not proud about this code but it does the job :~
fn solve_part_one(input: &String) {
    let values: Vec<char> = input.chars().collect();
    let mut is_marker: bool = false;
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut sec: bool = false;
    let mut skip_size: i32 = 0i32;
    let mut size: usize = 0;

    for i in 0..values.len() {
        if skip_size != 0 {
            skip_size -= 1;
            continue;
        }

        if is_marker {
            if values[i] == 'x' {
                sec = true;
            } else if !sec {
                first.push(values[i]);
            } else if values[i] != ')' {
                second.push(values[i]);
            }
        }

        match values[i] {
            '(' => {
                if i > 0 {
                    if values[i - 1] != ')' {
                        is_marker = true;
                    } else {
                        size += 1;
                    }
                } else {
                    is_marker = true;
                }
            },
            ')' => {
                if is_marker {
                    let mul: (i32, i32) = (first.parse().unwrap(), second.parse().unwrap());

                    for _ in 0..mul.1 {
                        for j in 0..mul.0 {
                            if i + j as usize + 1 < values.len() {
                                size += 1;
                            }
                        }
                    }

                    is_marker = false;
                    sec = false;
                    first.clear();
                    second.clear();
                    skip_size = mul.0;
                }
            },
            _ => {
                if !is_marker {
                    size += 1;
                }
            }
        }        
    }


    println!("Solution part one: {}", size);
}

fn solve_part_two(input: &String) {
    let values: Vec<char> = input.chars().collect();
    let mut is_marker: bool = false;
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut sec: bool = false;
    let mut skip_size: i32 = 0i32;
    let mut size: usize = 0;

    for i in 0..values.len() {
        if skip_size != 0 {
            skip_size -= 1;
            continue;
        }

        if is_marker {
            if values[i] == 'x' {
                sec = true;
            } else if !sec {
                first.push(values[i]);
            } else if values[i] != ')' {
                second.push(values[i]);
            }
        }

        match values[i] {
            '(' => {
                if i > 0 {
                    if values[i - 1] != ')' {
                        is_marker = true;
                    }
                } else {
                    is_marker = true;
                }
            },
            ')' => {
                if is_marker {
                    let mut mul: (i32, i32) = (first.parse().unwrap(), second.parse().unwrap());
                    skip_size = mul.0;

                    mul = marker(mul, &values, i);

                    size += (mul.0 * mul.1) as usize;

                    is_marker = false;
                    sec = false;
                    first.clear();
                    second.clear();
                }
            },
            _ => {
                if !is_marker {
                    size += 1;
                }
            }
        }        
    }

    println!("Solution part two: {}", size);
}

fn marker(mul: (i32, i32), values: &Vec<char>, i: usize) -> (i32, i32) {
    let mut is_marker: bool = false;
    let mut fir: bool = true;
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut size: usize = 0;
    let mut new_mul: (i32, i32) = (0, 0);

    for j in 0..mul.0 {
        if i + j as usize + 1 < values.len() {
            if is_marker {
                if values[i + j as usize + 1] == 'x' {
                    fir = false;
                } else if fir && values[i + j as usize + 1] != 'x' && values[i + j as usize + 1] != ')' {
                    first.push(values[i + j as usize + 1]);
                } else if values[i + j as usize + 1] != 'x' && values[i + j as usize + 1] != ')' {
                    second.push(values[i + j as usize + 1]);
                }
            }

            match values[i + j as usize + 1] {
                '(' => {
                    is_marker = true;
                },
                ')' => {
                    new_mul = (first.parse().unwrap(), second.parse().unwrap());

                    new_mul.1 *= mul.1;

                    new_mul = marker(new_mul, values, i + j as usize + 1);

                    is_marker = false;
                    fir = false;
                    first.clear();
                    second.clear();
                }
                _ => {

                }
            }
        }
    }
    
    new_mul
}