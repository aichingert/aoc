fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut sum: i32 = 0;

    for line in input.lines() {
        sum += line.parse::<i32>().unwrap() / 3 - 2;
    }

    println!("Solution part one: {}", sum);
}

fn solve_part_two(input: &String) {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut value: i32 = line.parse::<i32>().unwrap();
        
        while value > 0 {
            value = value / 3 - 2;
            if value > 0 {
                sum += value;
            }
        }
    }

    println!("Solution part two: {}", sum);
}