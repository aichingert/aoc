fn main() {
    let content = std::fs::read_to_string("input.txt").expect("");

    println!("Solution part 1: {}", part_one(&content));
    println!("Solution part 2: {}", part_two(&content));
}

fn part_one(content: &String) -> i32 {
    let mut previous: i32 = 0;
    let mut first: bool = true;
    let mut count: i32 = 0;

    for line in content.lines() {
        if first {
            previous = line.trim().parse::<i32>().unwrap();
            first = false;
        } else {
            let current = line.trim().parse::<i32>().unwrap();
            if current > previous {
                count += 1;
            }

            previous = current;
        }
    }

    count
}

fn part_two(content: &String) -> i32 {
    let mut count: i32 = 0;
    let lines: Vec<_> = content.lines().collect();

    for i in 3..lines.len() {
        let sum_a: i32 = lines[i - 3].trim().parse::<i32>().unwrap() + lines[i - 2].trim().parse::<i32>().unwrap() + lines[i - 1].trim().parse::<i32>().unwrap();
        let sum_b: i32 = lines[i - 2].trim().parse::<i32>().unwrap() + lines[i - 1].trim().parse::<i32>().unwrap() + lines[i].trim().parse::<i32>().unwrap();

        if sum_b > sum_a {
            count += 1;
        }
    }

    count
}