fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    println!("Solution part one: {}", solve_part_one(&input, 1, 3));
    println!("Solution part two: {}", solve_part_two(&input));
}

fn solve_part_one(input: &String, down: usize, right: usize) -> i64 {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut x: Vec<char> = Vec::new();

        for c in line.chars() {
            x.push(c);
        }

        map.push(x);
    }

    let mut x: usize = 0;
    let mut i: usize = 0;
    let mut trees: i64 = 0;

    while i < map.len() {
        if map[i][x] == '#' {
            trees += 1;
        }

        x = (x + right) % map[i].len();
        i += down;
    }

    trees
}

fn solve_part_two(input: &String) -> i64 {
    let mut result: i64 = 1;

    result *= solve_part_one(input, 1, 1);
    result *= solve_part_one(input, 1, 3);
    result *= solve_part_one(input, 1, 5);
    result *= solve_part_one(input, 1, 7);
    result *= solve_part_one(input, 2, 1);

    result
}