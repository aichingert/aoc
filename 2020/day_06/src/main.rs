use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut counter: usize = 0;
    let mut answers: HashMap<char, bool> = HashMap::new();

    for line in input.lines() {
        if line == "" {
            counter += answers.len();
            answers.clear();
            continue;
        }

        for c in line.chars() {
            answers.insert(c, true);
        }
    }

    counter += answers.len();

    println!("Solution part one: {}", counter);
}

fn solve_part_two(input: &String) {
    let mut counter: i32 = 0;
    let mut answers: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        if line == "" {
            counter += check_same_answers(&answers);
            answers.clear();
            continue;
        }

        answers.push(line.chars().collect());
    }

    counter += check_same_answers(&answers);

    println!("Solution part two: {}", counter);
}

fn check_same_answers(answers: &Vec<Vec<char>>) -> i32 {
    let mut same_answers: i32 = 0;
    let mut contains: bool;

    if answers.len() > 1 {
        for i in 0..answers[0].len() {
            contains = true;

            for j in 1..answers.len() {
                if !answers[j].contains(&answers[0][i]) {
                    contains = false;
                }
            }

            if contains {
                same_answers += 1;
            }
        }
    } else {
        same_answers += answers[0].len() as i32;
    }

    same_answers
}