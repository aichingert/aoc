fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut possible_count: i32 = 0;
    
    for line in input.lines() {
        let values: Vec<_> = line.split_whitespace().collect();

        let num_one: i32 = values[0].parse::<i32>().unwrap();
        let num_two: i32 = values[1].parse::<i32>().unwrap();
        let num_three: i32 = values[2].parse::<i32>().unwrap();
        
        if num_one + num_two > num_three && num_two + num_three > num_one && num_one + num_three > num_two {
            possible_count += 1;
        }
    }

    println!("Solution part one: {}", possible_count);
}

fn solve_part_two(input: &String) {
    let mut possible_count: i32 = 0;
    let mut counter: i32 = 1;
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let mut col_one: Vec<i32> = Vec::new();
    let mut col_two: Vec<i32> = Vec::new();
    let mut col_three: Vec<i32> = Vec::new();


    for line in input.lines() {
        let values: Vec<_> = line.split_whitespace().collect();

        if counter % 3 == 0 {
            col_one.push(values[0].parse::<i32>().unwrap());
            col_two.push(values[1].parse::<i32>().unwrap());
            col_three.push(values[2].parse::<i32>().unwrap());

            numbers.push(col_one.clone());
            numbers.push(col_two.clone());
            numbers.push(col_three.clone());

            col_one.clear();
            col_two.clear();
            col_three.clear();
        } else {
            col_one.push(values[0].parse::<i32>().unwrap());
            col_two.push(values[1].parse::<i32>().unwrap());
            col_three.push(values[2].parse::<i32>().unwrap());
        }

        counter += 1;
    }

    for num_values in numbers {
        if num_values[0] + num_values[1] > num_values[2] && num_values[1] + num_values[2] > num_values[0] && num_values[0] + num_values[2] > num_values[1] {
            possible_count += 1;
        }
    }

    println!("Solution part two: {}", possible_count);
}