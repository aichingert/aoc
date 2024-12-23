use std::collections::{HashMap, HashSet};

fn apply(mut sc: i64) -> i64 {
    sc = (sc ^ (sc * 64)) % 16777216;
    sc = (sc ^ (sc / 32)) % 16777216;
    sc = (sc ^ (sc * 2048)) % 16777216;
    sc
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/22").unwrap();
    let lines = inp.lines().filter(|l| !l.is_empty()).collect::<Vec<_>>();

    let mut part_one = 0;
    let mut part_two = HashMap::new();

    for line in lines.iter() {
        let mut secret = line.trim().parse::<i64>().unwrap();
        let mut ch = [0; 4];
        let mut seq = HashSet::new();

        for i in 0..2000 {
            let next = apply(secret);
            ch[0] = ch[1];
            ch[1] = ch[2];
            ch[2] = ch[3];
            ch[3] = (next % 10) - (secret % 10);

            if i > 2 && seq.insert(ch) {
                part_two.entry(ch).and_modify(|b| *b += next % 10).or_insert(next % 10);
            }
            secret = next;
        }

        part_one += secret;
    }

    println!("p1: {part_one}");
    println!("p2: {}", part_two.values().max().unwrap());
}
