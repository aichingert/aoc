use md5;

fn main() {
    let input: String = "ugkcyxxp".to_string();

    //solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut idx: usize = 0;
    let mut value: String;
    let mut password: String = String::new();

    while password.len() < 8 {
        let mut valid: bool = true;
        value = input.clone() + &idx.to_string();
        let digest = md5::compute(&value);
        let hash: String = format!("{:x}", digest);
    
        for i in 0..5 {
            if hash.chars().nth(i).unwrap() != '0'  {
                valid = false;
                break;
            }
        }
    
        if valid {
            password.push(hash.chars().nth(5).unwrap());
        }

        idx += 1;
    }

    println!("Solution part one: {}", password);
}

fn solve_part_two(input: &String) {
    let mut idx: usize = 0;
    let mut value: String;
    let mut password: [char; 8] = [' '; 8];
    let mut number_count: usize = 0;

    while number_count < 8 {
        let mut valid: bool = true;
        value = input.clone() + &idx.to_string();
        let digest = md5::compute(&value);
        let hash: String = format!("{:x}", digest);
    
        for i in 0..5 {
            if hash.chars().nth(i).unwrap() != '0'  {
                valid = false;
                break;
            }
        }
    
        if valid && hash.chars().nth(5).unwrap().is_numeric() {
            let index: usize = (hash.chars().nth(5).unwrap() as u8 - '0' as u8) as usize;

            if index >= 0 && index < 8 && password[index] == ' ' {
                password[index] = hash.chars().nth(6).unwrap();
                number_count += 1;

            }
        }

        idx += 1;
    }

    println!("Solution part one: {:?}", password);
}