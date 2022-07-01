fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        match split[0] {
            "forward" => {
                horizontal += split[1].parse::<i32>().unwrap();
            },
            "up" => {
                depth -= split[1].parse::<i32>().unwrap();
            },
            "down" => {
                depth += split[1].parse::<i32>().unwrap();
            }
            _ => {}
        }
    }

    println!("Solution part one: {}", depth * horizontal);
}

fn solve_part_two(input: &String) {
    let mut aim: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        let value: i32 = split[1].parse().unwrap();

        match split[0] {
            "forward" => {
                horizontal += value;
                depth += value * aim;
            },
            "up" => {
                aim -= value;
            },
            "down" => {
                aim += value;
            },
            _ => {}
        }
    }

    println!("Solution part two: {}", horizontal * depth);
}