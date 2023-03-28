// Advent of Code 2016, day 14
// (c) aichingert

#[path="../../utils/rust/md5.rs"] mod md5;

fn generate_hash(salt: &str, streching: bool) -> Vec<char> {
    match streching {
        true => {
            let mut hash = md5::md5_utf8(salt);

            for i in 0..2016 { hash = md5::md5_utf8(&hash); }
            hash.chars().collect::<Vec<char>>()
        },
        false => md5::md5_utf8(salt).chars().collect::<Vec<char>>()
    }
}

fn solve(salt: &str, streching: bool) -> usize {
    let mut index = 0;
    let mut count = 0;
    let mut hashes = Vec::<Vec<char>>::new();

    while hashes.len() < 1000 {
        hashes.push(generate_hash(&format!("{salt}{index}"), streching));
        index += 1;
    }

    'cond: while count < 64 {
        let hash = hashes.remove(0);
        hashes.push(generate_hash(&format!("{salt}{index}"), streching));
        let mut fst: char = '_';
        index += 1;

        for i in 0..hash.len()-2 {
            if hash[i] == hash[i+1] && hash[i+1] == hash[i+2] {
                fst = hash[i]; break;
            }
        }

        if fst != '_' {
            for i in 0..hashes.len() {
                let vec = vec![fst;5];
                for j in 0..hashes[i].len() - 5 {
                    if &hashes[i][j..j+5] == vec {
                        count += 1;
                        continue 'cond;
                    }
                }
            }
        }
    }

    index - 1001
}

fn main() {
    let salt = std::fs::read_to_string("../input/14").unwrap();

    println!("Part 1: {}", solve(&salt.trim(), false));
    println!("Part 2: {}", solve(&salt.trim(), true));
}
