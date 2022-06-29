fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let values: Vec<&str> = line.split('x').collect();
        let one: i32 = values[0].parse().unwrap();
        let two: i32 = values[1].parse().unwrap();
        let three: i32 = values[2].parse().unwrap();

        sum += 2 * one * two + 2 * two * three + 2 * three * one;
        sum += get_smallest(one, two, three);
    }

    println!("Solution part one: {}", sum);
}

fn solve_part_two(input: &String) {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let values: Vec<&str> = line.split('x').collect();
        let one: i32 = values[0].parse().unwrap();
        let two: i32 = values[1].parse().unwrap();
        let three: i32 = values[2].parse().unwrap();

        sum += get_smallest_sum(one, two, three) * 2 + one * two * three;
    }

    println!("Solution part two: {}", sum);
}

fn get_smallest(one: i32, two: i32, three: i32) -> i32 {
    let res_one: i32 = one * two;
    let res_two: i32 = two * three;
    let res_three: i32 = three * one;

    if res_one <= res_two && res_one <= res_three {
        return res_one;
    } else if res_two <= res_one && res_two <= res_three {
        return res_two;
    } else {
        res_three
    }
}

fn get_smallest_sum(one: i32, two: i32, three: i32) -> i32 {
    let res_one: i32 = one + two;
    let res_two: i32 = two + three;
    let res_three: i32 = three + one;

    if res_one <= res_two && res_one <= res_three {
        return res_one;
    } else if res_two <= res_one && res_two <= res_three {
        return res_two;
    } else {
        res_three
    }
}