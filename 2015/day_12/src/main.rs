fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input, true);
    solve_part_two(&input);
}

fn solve_part_one(input: &String, print: bool) -> i32 {
    let characters: Vec<char> = input.chars().collect();
    let mut is_negative: bool = false;
    let mut skip: i32 = 0;
    let mut number: String = String::new();
    let mut total: i32 = 0;

    for i in 0..characters.len() {  
        if skip != 0 {
            skip -= 1;
            continue;
        }

        if characters[i].is_numeric() {
            if i > 0 && characters[i - 1] == '-' {
                is_negative = true;
            }

            for j in i..characters.len() {
                if characters[j].is_numeric() {
                    number.push(characters[j]);
                    skip += 1;
                } else {
                    break;
                }
            }

            if is_negative {
                total -= number.parse::<i32>().unwrap();
            } else {
                total += number.parse::<i32>().unwrap();
            }

            number.clear();
            is_negative = false;
        }
    }

    if print {
        println!("Solution part one: {}", total);
    }
    total
}

fn solve_part_two(input: &String) {
    let mut characters: Vec<char> = input.chars().collect();
    let mut total: i32 = 0;

   

    println!("Solution part two: {}", total);
}
