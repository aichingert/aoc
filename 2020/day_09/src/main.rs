fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    println!("Solution part one: {}", solve_part_one(&input));
    solve_part_two(&input);
}

fn solve_part_one(input: &String) -> i64 {
    let preamble: usize = 25;
    let mut counter: usize = 0;
    let mut values: Vec<i64> = Vec::new();
    let mut not_contained: i64 = 0;

    for line in input.lines() {
        if counter < preamble {
            values.push(line.parse::<i64>().unwrap());
            counter += 1;
        } else {
            let number: i64 = line.parse::<i64>().unwrap();

            if !check_sum(&values, number) {
                not_contained = number;
                break;
            } else {
                values.remove(0);
                values.push(number);
            }
        }
    }

    not_contained
}

fn solve_part_two(input: &String) {
    let number: i64 = solve_part_one(input);
    let mut range: Vec<i64> = Vec::new();

    let numbers: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();

    for i in 0..numbers.len() {
        if sum(&range) < number {
            range.push(numbers[i]);
        } else {
            range.push(numbers[i]);

            while sum(&range) > number {
                range.remove(0);
            }
        }

        if sum(&range) == number && range.len() > 1 && numbers[i] != number {
            break;
        }
    }

    println!("Solution part two: {}", finish(&range));
}

fn sum(range: &Vec<i64>) -> i64 {
    let mut sum: i64 = 0;

    for i in 0..range.len() {
        sum += range[i];
    }

    sum 
}

fn finish(numbers: &Vec<i64>) -> i64 {
    let mut smallest: i64 = 1000000000000;
    let mut highest: i64 = 0;

    for i in 0..numbers.len() {
        if smallest > numbers[i] {
            smallest = numbers[i];
        } else if highest < numbers[i] {
            highest = numbers[i];
        }
    }

    smallest + highest
}

fn check_sum(numbers: &Vec<i64>, current: i64) -> bool {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j && numbers[i] + numbers[j] == current {
                return true;
            }
        }
    }

    false
}
