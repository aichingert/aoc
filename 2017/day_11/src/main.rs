fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&content);
}

fn solve_part_one(content: &String) {
    let directions: Vec<_> = content.trim().split(',').collect();
    let mut cords: (f32, f32) = (0., 0.);
    let mut furthest: (f32 , f32) = (0., 0.);

    for direction in directions {
        match direction {
            "nw" => {
                cords.0 -= 0.5;
                cords.1 += 0.5;
            },
            "n" => {
                cords.1 += 1.;
            },
            "ne" => {
                cords.0 += 0.5;
                cords.1 += 0.5;
            },
            "se" => {
                cords.0 += 0.5;
                cords.1 -= 0.5;
            },
            "s" => {
                cords.1 -= 1.;
            },
            "sw" => {
                cords.0 -= 0.5;
                cords.1 -= 0.5;
            },
            _ => {}
        }

        let current: f32 = cords.0.abs() + cords.1.abs();
        let check: f32 = furthest.0.abs() + furthest.1.abs();

        if current > check {
            furthest.0 = cords.0;
            furthest.1 = cords.1;
        }
    }


    println!("Solution part 1: {}", get_steps(&mut cords));
    println!("Solution part 2: {}", get_steps(&mut furthest));
}

fn get_steps(cords: &mut (f32, f32)) -> i32 {
    let mut steps: i32 = 0;

    while cords.0 != 0. || cords.1 != 0. {
        if cords.0 > 0. && cords.1 > 0. {
            while cords.0 != 0. {
                cords.0 -= 0.5;
                cords.1 -= 0.5;
                steps += 1;
            }
        } else if cords.0 > 0. && cords.1 < 0. {
            while cords.0 != 0. {
                cords.0 -= 0.5;
                cords.1 += 0.5;
                steps += 1;
            }
        } else if cords.0 < 0. && cords.1 > 0. {
            while cords.0 != 0. {
                cords.0 += 0.5;
                cords.1 -= 0.5;
                steps += 1;
            }
        } else if cords.0 < 0. && cords.1 < 0. {
            while cords.0 != 0. {
                cords.0 += 0.5;
                cords.1 += 0.5;
                steps += 1;
            }
        } else if cords.1 > 0. {
            while cords.1 != 0. {
                cords.1 -= 1.;
                steps += 1;
            }
        } else {
            while cords.1 != 0. {
                cords.1 += 1.;
                steps += 1;
            }
        }
    }

    steps
}