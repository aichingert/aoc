// Advent of Code 2016, day 14
// (c) aichingert

#[path="../../utils/rust/md5.rs"] mod md5;

fn part1(salt: &str) -> usize {
    let mut index = 0;
    let mut count = 0;
    let mut hashes = Vec::<Vec<char>>::new();

    while hashes.len() < 1000 {
        hashes.push(md5::md5_utf8(&format!("{salt}{index}")).chars().collect::<Vec<char>>());
        index += 1;
    }

    'cond: while count < 64 {
        let hash = hashes.remove(0);
        let mut fst: char = '_';
        hashes.push(md5::md5_utf8(&(salt.to_owned() + &index.to_string())).chars().collect::<Vec<char>>());
        index += 1;

        for i in 0..hash.len()-2 {
            if hash[i] == hash[i+1] && hash[i+1] == hash[i+2] {
                fst = hash[i];
                break;
            }
        }

        if fst != '_' {
            for i in 0..hashes.len() {
                let t = vec![fst;5];
                for j in 0..hashes[i].len() - 5 {
                    if &hashes[i][j..j+5] ==  t {
                        count += 1;
                        continue 'cond;
                    }
                }
            }
        }
    }
    index - 1001
}
fn part2() {}

fn main() {
    let salt = std::fs::read_to_string("../input/14").unwrap();

    println!("Part 1: {}", part1(&salt.trim()));
    //println!("Part 2: {}", part2());
}
