fn main() {
    let input_map: String = std::fs::read_to_string("input.txt").
    expect("err");

    solve(&input_map);
}

fn solve(input: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut line_c: Vec<char> = Vec::new();
        for c in line.chars() {
            line_c.push(c);
        }
        map.push(line_c);
    }

    let mut position: (usize, usize) = (0, 0);
    let mut can_move: bool = true;
    let mut current_direction: char = 's';
    let mut letters: String = String::new();
    let mut steps: i32 = 0;

    for j in 0..map[0].len() {
        if map[0][j] == '|' {
            position = (1, j);
            break;
        }
    }

    while can_move {
        let result = move_one(&mut position, current_direction , &map, &mut letters);
        can_move = result.0;
        current_direction = result.1;
        steps += 1;
    }

    println!("Solution part one: {:?}", letters);
    println!("Solution part two: {}", steps);
}

fn move_one(position: &mut (usize, usize), current_direction: char, map: &Vec<Vec<char>>, letters: &mut String) -> (bool, char) {
    let mut direc: char = current_direction;
    let mut can_move: bool = true;

    match map[position.0][position.1] {
        '+' => {
            if map[position.0 + 1][position.1] != ' ' && current_direction != 'n' {
                direc = 's';
            } else if map[position.0 - 1][position.1] != ' ' && current_direction != 's' {
                direc = 'n';
            } else if map[position.0][position.1 - 1] != ' ' && current_direction != 'e' {
                direc = 'w';
            } else if map[position.0][position.1 + 1] != ' ' && current_direction != 'w' {
                direc = 'e';
            }
        },
        _ => {
            match map[position.0][position.1] {
                '+' => {},
                '|' => {},
                '-' => {},
                ' ' => {
                    can_move = false;
                },
                _ => {
                    letters.push(map[position.0][position.1]);
                }
            }
        }
    }

    if can_move {
        match direc {
            'n' => {
                position.0 -= 1;
            },
            's' => {
                position.0 += 1;
            },
            'w' => {
                position.1 -= 1;
            },
            'e' => {
                position.1 += 1;
            }
            _ => {}
        }
    }

    (can_move, direc)
}