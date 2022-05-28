use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("err");
    let mut stuck: bool = true;
    let mut steps: i32 = 0;
    let mut idx: usize = 0;

    let lines: Vec<_> = content.split("\r").collect();
    let mut operations: Vec<i32> = Vec::new();

    for line in lines {
        operations.push(line.trim().parse().unwrap());
    }

    while stuck {
        if operations[idx] == 0 {
            operations[idx] += 1;
            steps += 1;
        }

        if operations[idx] < 0 {
            if idx as i32 - operations[idx] < 0 {
                stuck = false;
            }
        }

        if idx as i32 + operations[idx] >= operations.len() as i32 {
            stuck = false;
        }

        if stuck {
            let parse_i32_usize = idx as i32 + operations[idx];

            if operations[idx] >= 3 {
                operations[idx] -= 1;
            }
            else {
                operations[idx] += 1;
            }

            idx = parse_i32_usize as usize;
        }

        steps += 1;
    }    

    println!("{}", steps);
}
