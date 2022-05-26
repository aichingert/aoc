fn main() {
    let content = std::fs::read_to_string("input.txt").expect("err");

    solve(content);
}

fn solve(content: String) {
    let mut group_count: i32 = 1;
    let mut score: i32 = 0;
    let mut is_garbage: bool = false;
    let mut skip: bool = false;

    // part 2
    let mut garbage_count: i32 = 0;

    for line in content.lines() {
        let chars: Vec<_> = line.chars().collect();

        for i in 0..chars.len() {
            if skip {
                skip = false;
                continue;
            }

            if is_garbage {
                match chars[i] {
                    '>' => {
                        is_garbage = false;
                    },
                    '!' => {
                        skip = true;
                    },
                    _ => {
                        garbage_count += 1;
                    }
                }
            }
            else {
                match chars[i] {
                    '{' => {
                        score += group_count;
                        group_count += 1;
                    },
                    '}' => {
                        group_count -= 1;
                    },
                    '<' => {
                        is_garbage = true;
                    }
                    '!' => {
                        skip = true;
                    },
                    _ => {}
                }
            }
        }
    }

    println!("Solution part 1: {}", score);
    println!("Solution part 2: {}", garbage_count);
}