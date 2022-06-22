fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut frequenzy: i32 = 0;

    for line in input.lines() {
        frequenzy += line.parse::<i32>().unwrap();
    }

    println!("Solution part one: {}", frequenzy);
}

fn solve_part_two(input: &String) {
    let mut frequenzy: i32 = 0;
    let mut values: Vec<i32> = Vec::new();

    while !values.contains(&frequenzy) {
        values.push(frequenzy);
        
        for line in input.lines() {
            frequenzy += line.parse::<i32>().unwrap();

            if values.contains(&frequenzy) {
                break;
            }

            values.push(frequenzy);
        }

        values.pop();
    }

    println!("Solution part one: {}", frequenzy);
}