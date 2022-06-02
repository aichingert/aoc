const SIZE: usize = 256;

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&content);
    solve_part_two(content);
}

fn solve_part_one(content: &String) {
    let mut elements: [i32; SIZE] = (0..256).collect::<Vec<_>>()
    .try_into().expect("wrong size iterator");
    let mut min: usize = 0;
    let mut range: usize = 0;
    let mut skip_size: usize = 0;
    let mut changing: usize = 0;

    let numbers: Vec<_> = content.split(',').map( | str | {
        str.parse::<usize>().unwrap()
    }).collect();

    for number in numbers {
        min = changing;

        changing += skip_size + number;
        skip_size += 1;

        changing %= SIZE;

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

    println!("Solution part 1: {}", elements[0] * elements[1]);
}

fn solve_part_two(content: String) {
    let mut elements: [i32; SIZE] = (0..256).collect::<Vec<_>>()
    .try_into().expect("wrong size iterator");
    let mut min: usize = 0;
    let mut range: usize = 0;
    let mut skip_size: usize = 0;
    let mut changing: usize = 0;

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
    
            changing %= SIZE;
    
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

    println!("Solution part 2: {}", generate_hash(dense_hash(&mut elements)));
}

fn reverse(min: &mut usize, number_range: &mut usize, elements: &mut [i32; SIZE]) -> Vec<i32> {
    let mut min_idx: usize = *min;
    let mut cut: Vec<i32> = Vec::new();

    for i in 0..*number_range {
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

fn dense_hash(elements: &mut [i32; SIZE]) -> Vec<i32> {
    let mut hash_numbers: Vec<i32> = Vec::new();
    let mut hexed_number: i32 = 0;

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