use std::{collections::HashMap, fs};

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
}

fn solve_part_one(input: &String) {
    let mut id_count: i32 = 0;
    let mut valid_rooms: Vec<String> = Vec::new();
    for line in input.lines() {
        let values: Vec<_> = line.split('-').collect();
        let check: Vec<_> = values[values.len() - 1].split('[').collect();
        let mut letter_count: HashMap<char, i32> = HashMap::new();
        let mut snd_biggest: Vec<char> = Vec::new();

        for i in 0..values.len() - 1 {
            for c in values[i].chars() {
                if letter_count.contains_key(&c) {
                    let num: i32 = letter_count.remove(&c).unwrap();
                    letter_count.insert(c, num + 1);
                } else {
                    letter_count.insert(c, 1);
                    snd_biggest.push(c);
                }
            }
        }

        let mut biggest: i32 = 1;
        let mut order: String = String::new();
        let mut unordered: String = String::new();
        let mut vals: Vec<char> = Vec::new();
        let mut sec_idx: usize = 0;
        let mut valid: bool = true;
        let mut current: char = ' ';

        for c in check[1].chars() {
            if valid {
                if letter_count.contains_key(&c) && valid {
                    for j in 0..snd_biggest.len() {
                        if !(letter_count[&c] >= letter_count[&snd_biggest[j]]) && snd_biggest[j] != c {
                            valid = false;
                            break;
                        } else if snd_biggest[j] == c {
                            sec_idx = j;
                        }
                    }
                } else if c != ']' {
                    valid = false;
                }

                if valid && c != ']' {
                    snd_biggest.remove(sec_idx);
                    letter_count.remove(&c);
                }
            } else {
                break;
            }
        }

        if valid {
            id_count += check[0].parse::<i32>().unwrap();
            valid_rooms.push(line.to_string());
        }
    }

    println!("Solution part one: {}", id_count);
    solve_part_two(valid_rooms);
}

fn solve_part_two(input: Vec<String>) {
    for inp in input {
        let mut decrypted: String = String::new();
        let split: Vec<_> = inp.split('-').collect();
        let number: Vec<_> = split[split.len() - 1].split('[').collect(); 
        let room_id: i32 = number[0].parse::<i32>().unwrap();
        let mut dash_counter: u8 = 0;

        for c in inp.chars() {
            let new_char: char;

            if c == '-' {
                dash_counter += 1;
                if dash_counter == 3 {
                    break;
                }
                new_char = ' ';
            } else {
                let char_num: i32 = (c as u8 - 'a' as u8) as i32;
                new_char = ((((char_num + room_id) % 26) as u8) + ('a' as u8)) as char;
            }

            decrypted.push(new_char);
        }

        if decrypted == "northpole object storage" {
            println!("Solution part two: {}", room_id);
        }
    }
}