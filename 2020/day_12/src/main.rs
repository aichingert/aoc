fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut y: i32 = 0;
    let mut x: i32 = 0;
    let mut angle: i32 = 0;

    for line in input.lines() {
        let mut value: String = line.to_string();

        let direction: char = value.remove(0);

        match direction {
            'N' => {
                y += value.parse::<i32>().unwrap();
            },
            'S' => {
                y -= value.parse::<i32>().unwrap();
            },
            'E' => {
                x += value.parse::<i32>().unwrap();
            },
            'W' => {
                x -= value.parse::<i32>().unwrap();
            },
            'R' => {
                angle = (angle + value.parse::<i32>().unwrap()) % 360;
            },
            'L' => {
                angle = (angle - value.parse::<i32>().unwrap() + 360) % 360;
            },
            'F' => {
                match angle {
                    0 => {
                        x += value.parse::<i32>().unwrap();
                    },
                    90 => {
                        y -= value.parse::<i32>().unwrap();
                    },
                    180 => {
                        x -= value.parse::<i32>().unwrap();
                    },
                    270 => {
                        y += value.parse::<i32>().unwrap();
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }

    println!("Solution part one: {:?}", (y.abs() + x.abs()));
}

fn solve_part_two(input: &String) {
    let mut w_x: i32 = 10;
    let mut w_y: i32 = 1;

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for line in input.lines() {
        let mut value: String = line.to_string();

        let direction: char = value.remove(0);

        match direction {
            'N' => {
                w_y += value.parse::<i32>().unwrap();
            },
            'S' => {
                w_y -= value.parse::<i32>().unwrap();
            },
            'E' => {
                w_x += value.parse::<i32>().unwrap();
            },
            'W' => {
                w_x -= value.parse::<i32>().unwrap();
            },
            'R' => {
                let ticks: i32 = (value.parse::<i32>().unwrap() / 90) % 4;

                for _ in 0..ticks {
                    let temp: i32 = w_x;
                    w_x = w_y;
                    w_y = temp * -1;
                }
            },
            'L' => {
                let ticks: i32 = (value.parse::<i32>().unwrap() / 90) % 4;

                for _ in 0..ticks {
                    let temp: i32 = w_y;
                    w_y = w_x;
                    w_x = temp * -1;
                }
            },
            'F' => {
                let amount: i32 = value.parse::<i32>().unwrap();
                x += w_x * amount;
                y += w_y * amount;
            }
            _ => {}
        }
    }

    println!("Solution part two: {}", (x.abs() + y.abs()));
}