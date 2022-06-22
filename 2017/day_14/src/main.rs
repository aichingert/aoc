fn main() {
    let input: String = "uugsqrei".to_string();

    solve_part_one(input);
}

fn solve_part_one(input: String) {
    let mut rows: Vec<Vec<char>> = Vec::new();

    for i in 0..128 {
        let mut binary_form: String = String::new();
        let mut elements: [i32; 256] = (0..256).collect::<Vec<_>>()
            .try_into().expect("wrong size iterator");
        let mut min: usize = 0;
        let mut range: usize = 0;
        let mut skip_size: usize = 0;
        let mut changing: usize = 0;
        let content: String = input.clone()+ &"-".to_string() + &i.to_string();

        let mut numbers: Vec<_> = content.chars().map( | str | {
            str as usize
        }).collect();
        let prefixes: Vec<usize> = vec![17, 31, 73, 47, 23];

        for prefix in prefixes {
            numbers.push(prefix)
        }

        for _ in 0..64 {
            for number in numbers.clone() {
                min = changing;
        
                changing += skip_size + number;
                skip_size += 1;
        
                changing %= 256;
        
                range = number;
        
                let cut = reverse(&mut min, &mut range, &mut elements);
        
                let mut min_idx = min;
        
                for i in 0..cut.len() {
                    if min_idx >= elements.len() {
                        min_idx = 0;
                    }
        
                    elements[min_idx] = cut[i];
                    min_idx += 1;
                }
            }
        }

        let hash: String = generate_hash(dense_hash(&mut elements));
        println!("{}", hash);
        for c in hash.chars() {
            to_binary(c, &mut binary_form)
        }

        let col: Vec<char> = binary_form.chars().collect();
        rows.push(col.clone());
    }

    let mut count: i32 = 0;

    for s in &rows {
        for c in s {
            match c {
                '1' => {
                    count += 1;
                },
                _ => {}
            }
        }
    }

    println!("Solution part 1: {}", count);
    solve_part_two(rows)
}

fn solve_part_two(rows: Vec<Vec<char>>) {
    let mut found: Vec<(usize, usize)> = Vec::new();
    let mut group_count: i32 = 0;

    for i in 0..8 {
        for j in 0..8 {
            print!("{}", rows[i][j])
        }
        println!()
    }

    for i in 0..rows.len() {
        for j in 0..rows[i].len() {
            if rows[i][j] == '1' {
                if !found.contains(&(i, j)) {
                    found.push((i, j));
                    find_ones(&rows, (i, j), &mut found);  
                    group_count += 1;
                }
            }
        }
    }

    println!("Solution part 2: {}", group_count);
}

fn find_ones(grid: &Vec<Vec<char>>, starting_pos: (usize, usize), already_found: &mut Vec<(usize, usize)>) {
    if starting_pos.1 + 1 < grid.len() {
        if !already_found.contains(&(starting_pos.0, starting_pos.1 + 1)) && grid[starting_pos.0][starting_pos.1 + 1] == '1' {
            already_found.push((starting_pos.0, starting_pos.1 + 1));
            find_ones(grid, (starting_pos.0, starting_pos.1 + 1), already_found)
        }
    }

    if starting_pos.0 + 1 < grid.len() {
        if !already_found.contains(&(starting_pos.0 + 1, starting_pos.1)) && grid[starting_pos.0 + 1][starting_pos.1] == '1' {
            already_found.push((starting_pos.0 +1, starting_pos.1));
            find_ones(grid, (starting_pos.0 + 1, starting_pos.1), already_found)
        }
    }

    if starting_pos.0 as i32 - 1 > -1 {
        if !already_found.contains(&(starting_pos.0 - 1, starting_pos.1)) && grid[starting_pos.0 - 1][starting_pos.1] == '1' {
            already_found.push((starting_pos.0 - 1, starting_pos.1));
            find_ones(grid, (starting_pos.0 - 1, starting_pos.1), already_found)
        }
    }

    if starting_pos.1 as i32 - 1 > -1 {
        if !already_found.contains(&(starting_pos.0, starting_pos.1 - 1)) && grid[starting_pos.0][starting_pos.1 - 1] == '1' {
            already_found.push((starting_pos.0, starting_pos.1 - 1));
            find_ones(grid, (starting_pos.0, starting_pos.1 - 1), already_found)
        }
    }
}

fn reverse(min: &mut usize, number_range: &mut usize, elements: &mut [i32; 256]) -> Vec<i32> {
    let mut min_idx: usize = *min;
    let mut cut: Vec<i32> = Vec::new();

    for _ in 0..*number_range {
        if min_idx >= elements.len() {
            min_idx = 0;
        }

        cut.push(elements[min_idx].clone());
        min_idx += 1;
    }

    let len = cut.len();

    for i in 0..cut.len()/2 {
        let swap = cut[i];
        cut[i] = cut[len-i-1];
        cut[len-i-1] = swap;
    }

    cut
}

fn dense_hash(elements: &mut [i32; 256]) -> Vec<i32> {
    let mut hash_numbers: Vec<i32> = Vec::new();
    let mut hexed_number: i32;

    for i in 0..elements.len()/16 {
        let idx = i * 16;

        hexed_number = elements[idx] ^ elements[idx + 1] ^ elements[idx + 2] ^ elements[idx + 3] ^ elements[idx + 4] ^ elements[idx + 5] ^ elements[idx + 6] ^ elements[idx + 7] ^ elements[idx + 8] ^ elements[idx + 9] ^ elements[idx + 10] ^ elements[idx + 11] ^ elements[idx + 12] ^ elements[idx + 13] ^ elements[idx + 14] ^ elements[idx + 15];
        hash_numbers.push(hexed_number);
    }

    hash_numbers
}

fn generate_hash(dense_hash: Vec<i32>) -> String {
    let mut hash: String = String::new();

    for i in 0..dense_hash.len() {
       hash.push_str(&to_hex(dense_hash[i]));
    }

    hash
}

fn to_hex (value: i32) -> String {
    format!("{:02X}", value)
}

fn to_binary(value: char, binary_form: &mut String) {
    match value.to_ascii_uppercase() {
        '0' => {
            binary_form.push_str("0000");
        },
        '1' => {
            binary_form.push_str("0001");
        },
        '2' => {
            binary_form.push_str("0010");
        },
        '3' => {
            binary_form.push_str("0011");
        },
        '4' => {
            binary_form.push_str("0100");
        },
        '5' => {
            binary_form.push_str("0101");
        },
        '6' => {
            binary_form.push_str("0110");
        },
        '7' => {
            binary_form.push_str("0111");
        },
        '8' => {
            binary_form.push_str("1000");
        },
        '9' => {
            binary_form.push_str("1001");
        },
        'A' => {
            binary_form.push_str("1010");
        },
        'B' => {
            binary_form.push_str("1011");
        },
        'C' => {
            binary_form.push_str("1100");
        },
        'D' => {
            binary_form.push_str("1101");
        },
        'E' => {
            binary_form.push_str("1110");
        },
        'F' => {
            binary_form.push_str("1111");
        },
        _ => {
            println!("not matching")
        }
    }
}