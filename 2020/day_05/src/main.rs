fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
}

fn solve_part_one(input: &String) {
    let mut highest_score: usize = 0;
    let mut seats: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        let y: usize;
        let x: usize;

        let mut min: usize = 0;
        let mut max: usize = 127;

        let characters: Vec<char> = line.chars().collect();

        for i in 0..6 {
            let mut new_val: usize;

            if characters[i] == 'F' {
                new_val = max - min;

                if new_val % 2 != 0 {
                    new_val = new_val / 2 + 1;
                } else {
                    new_val = new_val / 2;
                }

                max -= new_val;
            } else {
                new_val = max - min;

                if new_val % 2 != 0 {
                    new_val = new_val / 2 + 1;
                } else {
                    new_val = new_val / 2;
                }
                min += new_val;
            }
        }

        if characters[6] == 'F' {
            y = min;
        } else {
            y = max;
        }
        
        min = 0;
        max = 7;

        for i in 7..characters.len() - 1 {
            let mut new_val: usize;

            if characters[i] == 'L' {
                new_val = max - min;

                if new_val % 2 != 0 {
                    new_val = new_val / 2 + 1;
                } else {
                    new_val = new_val / 2;
                }

                max -= new_val;
            } else {
                new_val = max - min;

                if new_val % 2 != 0 {
                    new_val = new_val / 2 + 1;
                } else {
                    new_val = new_val / 2;
                }

                min += new_val;
            }
        }

        if characters[characters.len() - 1] == 'L' {
            x = min;
        } else {
            x = max;
        }

        if y * 8 + x > highest_score {
            highest_score = y * 8 + x;
        }

        seats.push((y, x));
    }

    println!("Solution part one: {}", highest_score);

    solve_part_two(&seats);
}

fn solve_part_two(seats: &Vec<(usize, usize)>) {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; 8]; 128];
    let mut x: usize = 0;
    let mut y: usize = 0;

    for seat in seats {
        if seat.0 == 0 && seat.0 == 127 {
            continue;
        }

        map[seat.0][seat.1] = '#';
    }

    let mut changed: bool = false;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !changed && map[i][j] == '#' {
                changed = true;
            }

            if changed && map[i][j] == '.' {
                y = i;
                x = j;
                break;
            }
        }

        if y != 0 {
            break;
        }
    }

    println!("Solution part two: {}", y * 8 + x);
}