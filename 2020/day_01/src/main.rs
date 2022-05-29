fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("err");
    let mut solution_one: bool = false;
    let mut solution_two: bool = false;

    let numbers: Vec<_> = content.lines().collect();

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if !solution_one && i != j && numbers[i].parse::<i32>().unwrap() + numbers[j].parse::<i32>().unwrap() == 2020 {
                println!("Solution part 1: {}", numbers[i].parse::<i32>().unwrap() * numbers[j].parse::<i32>().unwrap());
                solution_one = true;
            }

            for k in 0..numbers.len() {
                if !solution_two && i != j && j != k && numbers[i].parse::<i32>().unwrap() + numbers[j].parse::<i32>().unwrap() + numbers[k].parse::<i32>().unwrap() == 2020 {
                    println!("Solution part 2: {}", numbers[i].parse::<i32>().unwrap() * numbers[j].parse::<i32>().unwrap() * numbers[k].parse::<i32>().unwrap());
                    solution_two = true;
                }
            }
        }
    }
}
