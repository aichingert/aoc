fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut counter: i32 = 0;

    for line in input.lines() {
        let values: Vec<&str> = line.split(':').collect();

        let peaks: Vec<&str> = values[0].split(' ').collect();
        let numbers: Vec<&str> = peaks[0].split('-').collect();

        let min: i32 = numbers[0].parse().unwrap();
        let max: i32 = numbers[1].parse().unwrap();
        let searching: char = peaks[1].chars().nth(0).unwrap();
        let mut char_count: i32 = 0;

        for c in values[1].chars() {
            if c == searching {
                char_count += 1;
            }
        }

        if min <= char_count && max >= char_count {
            counter += 1;
        }
    }

    println!("Solution part one: {}", counter);
}

fn solve_part_two(input: &String) {
    let mut counter: i32 = 0;

    for line in input.lines() {
        let values: Vec<&str> = line.split(':').collect();
        let mut contains: bool = false;

        let peaks: Vec<&str> = values[0].split(' ').collect();
        let numbers: Vec<&str> = peaks[0].split('-').collect();

        let idx_one: usize = numbers[0].parse().unwrap();
        let idx_two: usize = numbers[1].parse().unwrap();
        let searching: char = peaks[1].chars().nth(0).unwrap();
        let characters: Vec<char> = values[1].chars().collect();

        if characters[idx_one] == searching {
            contains = true;
        }

        if characters[idx_two] == searching {
            if contains {
                contains = false;
            } else {
                contains = true;
            }
        }

        if contains {
            counter += 1;
        }
    }

    println!("Solution part two: {}", counter);
}