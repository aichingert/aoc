fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut height: i32 = 0;

    for c in input.chars() {
        if c == '(' {
            height += 1;
        } else {
            height -= 1;
        }
    }

    println!("Solution part one: {}", height);
}

fn solve_part_two(input: &String) {
    let mut height: i32 = 0;
    let mut position: i32 = 0;

    for c in input.chars() {
        if c == '(' {
            height += 1;
        } else {
            height -= 1;
        }

        position += 1;

        if height == -1 {
            break;
        }
    }

    println!("Solution part two: {}", position);
}