fn main() {
    let input: usize = 304;

    solve_part_one(input);
    solve_part_two(input);
}

fn solve_part_one(input: usize) {
    let mut numbers: Vec<usize> = vec![0];
    let mut index: usize = 0;
    let mut element: usize = 1;
    let steps: usize = input;

    for _ in 0..2017 {
        index = (index + steps) % numbers.len() + 1;
        numbers.insert(index, element);
        element += 1;
    }

    println!("Solution part 1: {}", numbers[index + 1]);
}

fn solve_part_two(input: usize) {
    let mut len: usize = 1;
    let mut index: usize = 0;
    let steps: usize = input;
    let mut searching: usize = 0;

    for _ in 0..50000000 {
        index = (index + steps) % len + 1;

        if index == 1 {
            searching = len;
        }

        len += 1;
    }

    println!("Solution part 2: {}", searching);
}