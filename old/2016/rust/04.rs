// Advent of Code 2016, day 4
// (c) aichingert

use std::collections::HashMap;

fn part1(inp: &[&str]) -> u32 {
    let mut ans = 0u32;

    for l in inp {
        let (lhs, rhs) = l.split_once('[').unwrap();
        let vls = lhs.split('-').collect::<Vec<&str>>();
        let mut valid: bool = true;
        let mut counter: HashMap<char, u32> = HashMap::new();

        for i in 0..vls.len()-1 {
            for ch in vls[i].chars() {
                if let Some(cur) = counter.remove(&ch) {
                    counter.insert(ch, cur+1);
                } else {
                    counter.insert(ch, 1);
                }
            }
        }

        for ch in rhs.chars() {
            if counter.contains_key(&ch) {
                let cur = counter[&ch];
                let mut max = 0u32;

                for k in counter.keys() {
                    if counter[k] > max {
                        max = counter[k];
                    }
                }

                if cur < max {
                    valid = false;
                    break;
                } else {
                    counter.remove(&ch);
                }
            } else if ch != ']' {
                valid = false;
                break;
            }
        }

        if valid { ans += vls[vls.len()-1].parse::<u32>().unwrap(); }
    }

    ans
}

fn part2(inp: &[&str]) -> u32 {
    let mut ans = 0u32;

    for i in 0..inp.len() {
        let (lhs, _) = inp[i].split_once('[').unwrap();
        let mut lhs = lhs.split('-').collect::<Vec<&str>>();
        ans = lhs.remove(lhs.len()-1).parse::<u32>().unwrap();
        let mut vls = lhs.join("").to_lowercase().chars().collect::<Vec<char>>();

        for _ in 0..ans {
            for i in 0..vls.len() {
                if vls[i] == 'z' {
                    vls[i] = 'a';
                } else {
                    vls[i] = ((vls[i] as u8) + 1u8) as char;
                }
            }
        }

        if &vls.into_iter().collect::<String>() == "northpoleobjectstorage" {
            break;
        }
    }
    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/04").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
