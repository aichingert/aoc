fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        map.push(line.chars().collect());
        new_map.push(vec!['.'; line.len()]);
    }

    loop {
        new_map.clear();

        for i in 0..map.len() {
            let mut line: Vec<char> = Vec::new();
    
            for j in 0..map[i].len() {
                if map[i][j] != '.' {
                    let mut count: i32 = count_around_part_one(&map, (i, j));
    
                    if map[i][j] == '#' {
                        count -= 1;
                        if count >= 4 {
                            line.push('L');
                        } else {
                            line.push('#');
                        }
                    } else {
                        if count == 0 {
                            line.push('#');
                        } else {
                            line.push('L');
                        }
                    }
                } else {
                    line.push('.');
                }
            }
    
            new_map.push(line);
        }

        if changed(&map, &new_map) {
            map = new_map.clone();
        } else {
            break;
        }
    }

    let mut counter: i32 = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#'{
                counter += 1;
            }
        }
    }

    println!("Solution part one: {}", counter);
}

fn solve_part_two(input: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        map.push(line.chars().collect());
        new_map.push(vec!['.'; line.len()]);
    }

    loop {
        new_map.clear();

        for i in 0..map.len() {
            let mut line: Vec<char> = Vec::new();
    
            for j in 0..map[i].len() {
                if map[i][j] != '.' {
                    let mut count: i32 = count_around_part_two(&map, (i, j));
    
                    if map[i][j] == '#' {
                        count -= 1;
                        if count >= 5 {
                            line.push('L');
                        } else {
                            line.push('#');
                        }
                    } else {
                        if count == 0 {
                            line.push('#');
                        } else {
                            line.push('L');
                        }
                    }
                } else {
                    line.push('.');
                }
            }
    
            new_map.push(line);
        }

        if changed(&map, &new_map) {
            map = new_map.clone();
        } else {
            break;
        }
    }

    let mut counter: i32 = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#'{
                counter += 1;
            }
        }
    }

    println!("Solution part two: {}", counter);
}

fn count_around_part_one(map: &Vec<Vec<char>>, current_pos: (usize, usize)) -> i32 {
    let mut counter: i32 = 0;
    let mut already_counted: Vec<(usize, usize)> = Vec::new();

    for i in -1..2 {
        for j in -1..2 {
            if current_pos.0 as i32 + i >= 0 && current_pos.0 as i32 + i < map.len() as i32 && current_pos.1 as i32 + j >= 0 && current_pos.1 as i32 + j < map[current_pos.0].len() as i32 {
                if map[(current_pos.0 as i32 + i) as usize][(current_pos.1 as i32 + j) as usize] == '#' {
                    counter += 1;
                    already_counted.push(((current_pos.0 as i32 + i) as usize, (current_pos.1 as i32 + j) as usize));
                }
            } 
        }
    }

    counter
}

fn count_around_part_two(map: &Vec<Vec<char>>, current_pos: (usize, usize)) -> i32 {
    let mut counter: i32 = 0;
    let mut already_counted: Vec<(usize, usize)> = Vec::new();

    for i in -1..2 {
        for j in -1..2 {
            if current_pos.0 as i32 + i >= 0 && current_pos.0 as i32 + i < map.len() as i32 && current_pos.1 as i32 + j >= 0 && current_pos.1 as i32 + j < map[current_pos.0].len() as i32 {
                if map[(current_pos.0 as i32 + i) as usize][(current_pos.1 as i32 + j) as usize] == '#' {
                    counter += 1;
                    already_counted.push(((current_pos.0 as i32 + i) as usize, (current_pos.1 as i32 + j) as usize));
                }
            } 
        }
    }

    let mut pointer: usize = current_pos.0;

    while pointer > 0 {
        pointer -= 1;
        
        if map[pointer][current_pos.1] != '.' {
            if map[pointer][current_pos.1] == '#' && !already_counted.contains(&(pointer, current_pos.1)) {
                counter += 1;
            }

            break;
        }
    }

    pointer = current_pos.0;

    while pointer < map.len() - 1 {
        pointer += 1;
        
        if map[pointer][current_pos.1] != '.' {
            if map[pointer][current_pos.1] == '#' && !already_counted.contains(&(pointer, current_pos.1)) {
                counter += 1;
            }

            break;
        }
    }

    pointer = current_pos.1;

    while pointer > 0 {
        pointer -= 1;
        
        if map[current_pos.0][pointer] != '.' {
            if map[current_pos.0][pointer] == '#' && !already_counted.contains(&(current_pos.0, pointer)) {
                counter += 1;
            }

            break;
        }
    }

    pointer = current_pos.1;

    while pointer < map[0].len() - 1 {
        pointer += 1;
        
        if map[current_pos.0][pointer] != '.' {
            if map[current_pos.0][pointer] == '#' && !already_counted.contains(&(current_pos.0, pointer)) {
                counter += 1;
            }

            break;
        }
    }

    let mut second_pointer: usize = current_pos.1;
    pointer = current_pos.0;

    while pointer > 0 && second_pointer > 0 {
        pointer -= 1;
        second_pointer -= 1;

        if map[pointer][second_pointer] != '.' {
            if map[pointer][second_pointer] == '#' && !already_counted.contains(&(pointer, second_pointer)) {
                counter += 1;
            }

            break;
        }
    }

    pointer = current_pos.0;
    second_pointer = current_pos.1;

    while pointer < map.len() - 1 && second_pointer < map[0].len() - 1 {
        pointer += 1;
        second_pointer += 1;

        if map[pointer][second_pointer] != '.' {
            if map[pointer][second_pointer] == '#' && !already_counted.contains(&(pointer, second_pointer)) {
                counter += 1;
            }

            break;
        }
    }

    pointer = current_pos.0;
    second_pointer = current_pos.1;

    while pointer > 0 && second_pointer < map[0].len() - 1 {
        pointer -= 1;
        second_pointer += 1;

        if map[pointer][second_pointer] != '.' {
            if map[pointer][second_pointer] == '#' && !already_counted.contains(&(pointer, second_pointer)) {
                counter += 1;
            }

            break;
        }
    }

    pointer = current_pos.0;
    second_pointer = current_pos.1;

    while pointer < map.len() - 1 && second_pointer > 0 {
        pointer += 1;
        second_pointer -= 1;

        if map[pointer][second_pointer] != '.' {
            if map[pointer][second_pointer] == '#' && !already_counted.contains(&(pointer, second_pointer)) {
                counter += 1;
            }

            break;
        }
    }

    counter
}

fn changed(map: &Vec<Vec<char>>, check: &Vec<Vec<char>>) -> bool {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != check[i][j]{
                return true;
            }
        }
    }

    false
}