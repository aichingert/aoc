use std::{collections::HashMap, fs};

fn main() {
    let content = fs::read_to_string("input.txt").expect("error");
    let lines: Vec<_> = content.split("\r").collect();
    let mut counter: i32  = 0;

    for to_trim in lines {
        let line = to_trim.trim();
        println!("{:?}", line);
        let words: Vec<_> = line.split(' ').collect();
        let mut containing_words: Vec<&str> = Vec::new();
        let mut invalid: bool = false;

        for word in words {
            if containing_words.contains(&word) {
                invalid = true;
                break;
            }
            else {
                containing_words.push(&word);
            }
        }

        for i in 0..containing_words.len() {
            for j in 0..containing_words.len() {
                if j == i {
                    continue;
                }

                if is_rearragable(containing_words[i], containing_words[j]) {
                    invalid = true;
                }
            }
        }

        if !invalid {
            counter+=1;
        }
    }

    println!("{}", counter);
}

fn is_rearragable(incoming: &str, check: &str) -> bool {
    let mut incoming_chars: HashMap<char, i32> = HashMap::new();
    let mut check_chars: HashMap<char, i32> = HashMap::new();

    if incoming.len() != check.len() {
        return false;
    }

    for current_char in incoming.chars() {
        if incoming_chars.contains_key(&current_char) {
            let value = incoming_chars[&current_char];
            incoming_chars.remove(&current_char);
            incoming_chars.insert(current_char, value + 1);
        }
        else {
            incoming_chars.insert(current_char, 1);
        }
    }

    for current_char in check.chars() {
        if check_chars.contains_key(&current_char) {
            let value = check_chars[&current_char];
            check_chars.remove(&current_char);
            check_chars.insert(current_char, value + 1);
        }
        else {
            check_chars.insert(current_char, 1);
        }
    }

    for key in incoming_chars.keys() {
        if check_chars.contains_key(key) {
            if incoming_chars[key] != check_chars[key] {
                return false;
            }
        }
        else {
            return false;
        }
    }

    true
}