fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

#[derive(Debug, Clone)]
struct Reindeer {
    speed: u16,
    time: u16,
    rest: u16,
    timer: u16,
    is_flying: bool,
    distance: u16,
    points: u16,
}

fn solve_part_one(input: &String) {
    let mut reindeers: Vec<Reindeer> = Vec::new();

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let reindeer: Reindeer = Reindeer { 
            speed: values[3].parse().unwrap(), 
            time: values[6].parse().unwrap(), 
            rest: values[13].parse().unwrap(),
            timer: values[6].parse().unwrap(),
            is_flying: true,
            distance: 0,
            points: 0,
        };

        reindeers.push(reindeer);
    }

    for reindeer in &mut reindeers {
        for _ in 0..2503 {
            if reindeer.timer == 0 {
                if reindeer.is_flying {
                    reindeer.is_flying = false;
                    reindeer.timer = reindeer.rest - 1;
                } else {
                    reindeer.is_flying = true;
                    reindeer.timer = reindeer.time;
                    reindeer.distance += reindeer.speed;
                    reindeer.timer -= 1;
                }
            } else {
                if reindeer.is_flying {
                    reindeer.distance += reindeer.speed;
                }

                reindeer.timer -= 1;
            }
        }
    }   

    let mut highest_dis: u16 = 0;

    for r in &reindeers {
        if r.distance > highest_dis {
            highest_dis = r.distance;
        }
    }

    println!("Solution part one: {}", highest_dis);
}

fn solve_part_two(input: &String) {
    let mut reindeers: Vec<Reindeer> = Vec::new();

    for line in input.lines() {
        let values: Vec<&str> = line.split(' ').collect();
        let reindeer: Reindeer = Reindeer { 
            speed: values[3].parse().unwrap(), 
            time: values[6].parse().unwrap(), 
            rest: values[13].parse().unwrap(),
            timer: values[6].parse().unwrap(),
            is_flying: true,
            distance: 0,
            points: 0,
        };

        reindeers.push(reindeer);
    }

    for _ in 0..2503 {
        for reindeer in &mut reindeers {
            if reindeer.timer == 0 {
                if reindeer.is_flying {
                    reindeer.is_flying = false;
                    reindeer.timer = reindeer.rest - 1;
                } else {
                    reindeer.is_flying = true;
                    reindeer.timer = reindeer.time;
                    reindeer.distance += reindeer.speed;
                    reindeer.timer -= 1;
                }
            } else {
                if reindeer.is_flying {
                    reindeer.distance += reindeer.speed;
                }

                reindeer.timer -= 1;
            }
        }

        let mut highest_dis: u16 = 0;
        let mut pos: Vec<usize> = Vec::new();

        for i in 0..reindeers.len() {
            if reindeers[i].distance > highest_dis {
                highest_dis = reindeers[i].distance;
            }
        }

        for i in 0..reindeers.len() {
            if reindeers[i].distance == highest_dis {
                pos.push(i);
            }
        }

        for i in pos {
            reindeers[i].points += 1;
        }
    }

    let mut most_points: u16 = 0;

    for i in 0..reindeers.len() {
        if most_points < reindeers[i].points {
            most_points = reindeers[i].points;
        }
    }

    println!("Solution part two: {}", most_points);
}