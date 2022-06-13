use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let cords: Vec<_> = input.split(", ").collect();
    let mut pos: (i32, i32) = (0, 0);
    let mut degre: i32 = 0;

    for cord in cords {
        let values: Vec<_> = cord.chars().collect();
        let mut number: String = String::new();

        for i in 1..values.len() {
            number.push(values[i]);
        }

        match values[0] {
            'R' => {
                degre = (degre + 90) % 360;

                if degre == 90 {
                    pos.0 += number.parse::<i32>().unwrap();
                } else if degre == 180 {
                    pos.1 += number.parse::<i32>().unwrap();
                } else if degre == 270 {
                    pos.0 -= number.parse::<i32>().unwrap();
                } else if degre == 0 {
                    pos.1 -= number.parse::<i32>().unwrap();
                } else {
                    println!("{}", degre);
                }
            },
            'L' => {
                degre = (degre - 90) % 360;

                if degre < 0 {
                    degre = 270;
                }

                if degre == 90 {
                    pos.0 += number.parse::<i32>().unwrap();
                } else if degre == 180 {
                    pos.1 += number.parse::<i32>().unwrap();
                } else if degre == 270 {
                    pos.0 -= number.parse::<i32>().unwrap();
                } else if degre == 0 {
                    pos.1 -= number.parse::<i32>().unwrap();
                } else {
                    println!("{}", degre);
                }
            }
            _ => {}
        }
    }

    println!("Solution part one: {:?}", pos.0.abs() + pos.1.abs())
}

fn solve_part_two(input: &String) {
    let cords: Vec<_> = input.split(", ").collect();
    let mut positions: HashMap<(i32, i32), bool> = HashMap::new();
    let mut pos: (i32, i32) = (0, 0);
    let mut search_pos: (i32, i32, bool) = (0, 0, false);
    let mut degre: i32 = 0;

    while !search_pos.2 {
        for cord in &cords {
            let values: Vec<_> = cord.chars().collect();
            let mut number_string: String = String::new();
    
            for i in 1..values.len() {
                number_string.push(values[i]);
            }
    
            let number: i32 = number_string.parse::<i32>().unwrap();
    
            match values[0] {
                'R' => {
                    degre = (degre + 90) % 360;
    
                    if degre == 90 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0 + i, pos.1)) && !search_pos.2 {
                                search_pos = (pos.0 + i, pos.1, true);
                            } else {
                                positions.insert((pos.0 + i, pos.1), true);
                            }
                        }
                        
                        pos.0 += number;
                    } else if degre == 180 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0, pos.1 + i)) && !search_pos.2 {
                                search_pos = (pos.0, pos.1 + i, true);
                            } else {
                                positions.insert((pos.0, pos.1 + i), true);
                            }
                        }
    
                        pos.1 += number;
                    } else if degre == 270 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0 - i, pos.1)) && !search_pos.2 {
                                search_pos = (pos.0 - i, pos.1, true);
                            } else {
                                positions.insert((pos.0 - i, pos.1), true);
                            }
                        }
    
                        pos.0 -= number;
                    } else if degre == 0 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0, pos.1 - i)) && !search_pos.2 {
                                search_pos = (pos.0, pos.1 - i, true);
                            } else {
                                positions.insert((pos.0, pos.1 - i), true);
                            }
                        }
    
                        pos.1 -= number;
                    } else {
                        println!("{}", degre);
                    }
                },
                'L' => {
                    degre = (degre - 90) % 360;
    
                    if degre < 0 {
                        degre = 270;
                    }
    
                    if degre == 90 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0 + i, pos.1)) && !search_pos.2 {
                                search_pos = (pos.0 + i, pos.1, true);
                            } else {
                                positions.insert((pos.0 + i, pos.1), true);
                            }
                        }
                        
                        pos.0 += number;
                    } else if degre == 180 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0, pos.1 + i)) && !search_pos.2 {
                                search_pos = (pos.0, pos.1 + i, true);
                            } else {
                                positions.insert((pos.0, pos.1 + i), true);
                            }
                        }
    
                        pos.1 += number;
                    } else if degre == 270 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0 - i, pos.1)) && !search_pos.2 {
                                search_pos = (pos.0 - i, pos.1, true);
                            } else {
                                positions.insert((pos.0 - i, pos.1), true);
                            }
                        }
    
                        pos.0 -= number;
                    } else if degre == 0 {
                        for i in 0..number {
                            if positions.contains_key(&(pos.0, pos.1 - i)) && !search_pos.2 {
                                search_pos = (pos.0, pos.1 - i, true);
                            } else {
                                positions.insert((pos.0, pos.1 - i), true);
                            }
                        }
    
                        pos.1 -= number;
                    } else {
                        println!("{}", degre);
                    }
                }
                _ => {}
            }
        }
    } 

    println!("Solution part two: {:?}", search_pos.0.abs() + search_pos.1.abs())
}