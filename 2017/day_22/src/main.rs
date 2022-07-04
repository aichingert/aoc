fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
}

fn solve_part_one(input: &String) {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let mut pointing: i32 = 90; // Degree
    let mut bursts: i32 = 0;

    for line in input.lines() {
        let mut characters: Vec<char> = Vec::new();

        for c in line.chars() {
            characters.push(c);
        }

        map.push(characters);
    }

    pos.0 = map.len() / 2;
    pos.1 = map[pos.0].len() / 2;

    for i in 0..10000 {

        if map[pos.0][pos.1] == '#' {
            map[pos.0][pos.1] = '.';
            pointing -= 90;
        } else {
            map[pos.0][pos.1] = '#';
            pointing += 90;
            bursts += 1;
        }

        if pointing < 0 {
            pointing = 270;
        }

        pointing %= 360;

        match pointing {
            0 => {
                if pos.1 + 1 >= map[pos.0].len() {
                    expand(2, &mut map);
                }

                pos.1 += 1;
            },
            90 => {
                if pos.0 as i32 - 1 < 0 {
                    expand(1, &mut map);
                } else {
                    pos.0 -= 1;
                }
            },
            180 => {
                if pos.1 as i32 - 1 < 0 {
                    expand(3, &mut map);
                } else {
                    pos.1 -= 1;
                }
            },
            270 => {
                if pos.0 + 1 >= map.len() {
                    expand(0, &mut map);
                }

                pos.0 += 1;
            },
            _ => {}
        }
    }

    println!("Solution part one: {}", bursts);
}

fn expand(dir: u8, map: &mut Vec<Vec<char>>) {
    match dir {
        0 => { // expand below
            map.insert(map.len(), vec!['.'; map[0].len()]);
        },
        1 => { // expand up
            map.insert(0, vec!['.'; map[0].len()]);
        },
        2 => { // expand right
            for i in 0..map.len() {
                map[i].push('.');
            }
        },
        3 => { // expand left
            for i in 0..map.len() {
                map[i].insert(0, '.');
            }
        },
        _ => {}
    }
}

fn print(map: &Vec<Vec<char>>, pos: (usize, usize)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if (i, j) == pos {
                print!("* ");
            } else {
                print!("{} ", map[i][j]);
            }
        }
        println!();
    }
}