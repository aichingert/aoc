use std::{collections::HashMap};

fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(content);
}

#[derive(Debug, Clone)]
struct Field {
    idx: i32,
    depth: i32,
    position: i32,
    direction: bool,
}

fn solve_part_one(content: String) {
    let mut board: HashMap<i32, Field> = HashMap::new();
    let mut distance: i32 = 0;
    let mut count: i32 = 0;

    for line in content.lines() {
        let values: Vec<i32> = line.split(": ").map( | x | {
            x.parse::<i32>().unwrap()
        }).collect();

        let field: Field = Field { idx: values[0], depth: values[1], position: 1, direction: true };
        if values[0] > distance {
            distance = values[0];
        }
        board.insert(values[0], field);
    }

    let mut starting_board: HashMap<i32, Field> = board.clone();

    for i in 0..=distance {
        for field in board.clone().values() {
            if i == field.idx {
                if field.position == 1 {
                    count += field.depth * field.idx;
                    break;
                }
            }
        }

        update_board(&mut board);
    }

    println!("Solution part 1: {}", &count);
    solve_part_two(&mut starting_board, distance)
}

fn solve_part_two(board: &mut HashMap<i32, Field>, distance: i32) {
    let mut pico_seconds: i32 = 1;
    let mut delay: i32 = 0;
    let mut dead: bool = true;

    while dead {
        dead = false;
        for key in board.keys() {
            if scanner(delay + key, board[key].depth) == 0 {
                dead = true;
                break;
            }
        }

        delay += 1
    }

    println!("Solve part two: {}", delay - 1);
}

fn scanner(time: i32, height: i32) -> i32 {
    let offset = time % ((height - 1) * 2);

    if offset > height - 1 {
        return 2 * (height - 1) - offset;
    } else {
        return offset;
    }
}

fn update_board(board: &mut HashMap<i32, Field>) {
    let mut fields: Vec<Field> = Vec::new();

    for key in board.clone().keys() {
        fields.push(board.remove(key).unwrap())
    }

    for field in fields.iter_mut() {
        if field.depth > 1 {
            if field.direction {
                field.position += 1
            } else {
                field.position -= 1
            }

            if field.position == field.depth {
                field.direction = false;
            }

            if field.position == 1 {
                field.direction = true;
            }
        }

        board.insert(field.idx, field.clone());
    }
}

/*

    while dead {
        dead = false;
        update_board(&mut starting_board);

        for i in 0..=distance {
            if board.contains_key(&i) {
                if board[&i].position == 1 {
                    dead = true;
                }
            }

            if dead {
                break;
            }
    
            update_board(board);
        }

        if dead {
            delay += 1;
            *board = starting_board.clone();
            println!("{}", delay);
        }
    } */