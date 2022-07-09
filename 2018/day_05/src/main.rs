fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    println!("Solution part one: {}", solve_part_one(&input));
    solve_part_two(&input);
}

fn solve_part_one(input: &String) -> usize {
    let mut characters: Vec<char> = input.chars().collect();
    let mut offset: usize;
    let mut changed: bool = true;

    while changed {
        changed = false;
        offset = 0;

        for i in 0..characters.len() - 1 {
            if characters[i - offset] as u8 + 32 == characters[i + 1 - offset] as u8 || characters[i - offset] as u8 - 32 == characters[i + 1 - offset] as u8 {
                characters.remove(i - offset);
                characters.remove(i - offset);
                changed = true;
                break;

                //offset += 2;
                //println!("{}, {}", i, offset);
            }
        }
    }

    characters.len()
}

fn solve_part_two(input: &String) {
    let mut len: usize = 100000000;

    for c in 'a'..='z'{
        let mut to_extract: String = String::new();

        for ch in input.chars() {
            if ch.to_ascii_lowercase() != c {
                to_extract.push(ch);
            }
        }

        let testing_len = solve_part_one(&to_extract);

        if testing_len < len {
            len = testing_len;
        }
    }

    println!("Solution part two: {}", len);
}