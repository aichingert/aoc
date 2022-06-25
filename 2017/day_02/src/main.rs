fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut sum: i32 = 0;
    
    for line in input.lines() {
        let values: Vec<&str> = line.split('\t').collect();
        let mut smallest = 10000000;
        let mut biggest = 0;

        for value in values {
            let value: i32 = value.parse::<i32>().unwrap();

            if value > biggest {
                biggest = value;
            }

            if value < smallest {
                smallest = value;
            }
        }

        sum += biggest - smallest;
    }

    println!("Solution part one: {}", sum);
}

fn solve_part_two(input: &String) {
    let mut sum: i32 = 0;
    
    for line in input.lines() {
        let values: Vec<&str> = line.split('\t').collect();
        let mut found: bool = false;

        for i in 0..values.len() {
            for j in 0..values.len() {
                if i != j {
                    if values[i].parse::<i32>().unwrap() % values[j].parse::<i32>().unwrap() == 0 {
                        sum += values[i].parse::<i32>().unwrap() / values[j].parse::<i32>().unwrap();
                        found = true;
                        break;
                    }
                }
            }

            if found {
                break;
            }
        }
    }

    println!("Solution part two: {}", sum);
}