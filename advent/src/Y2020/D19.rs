use std::collections::HashMap;

fn part_one(xs: &[char], r: u32, rlu: &HashMap<u32, Vec<char>>) -> i32 {
    let cur = &rlu[&r];

    if cur[0] == '"' {
        if xs[0] == cur[1] { return 1; }
        return -1;
    }

    for opt in cur.iter().collect::<String>().split(" | ") {
        let mut acc = 0;

        for i in opt.split(' ').map(|n| n.parse::<u32>().unwrap()) {
            let ret = part_one(&xs[acc as usize..], i, rlu);
            if ret == -1 {
                acc = -1;
                break;
            }
            acc += ret;
        }

        if acc != -1 { return acc; }
    }

    -1
}

fn part_two(xs: &[char], r: u32, rlu: &HashMap<u32, Vec<char>>) -> Vec<i32> {
    let cur = &rlu[&r];

    if cur[0] == '"' {
        if xs.len() > 0 && xs[0] == cur[1] { return vec![1]; }
        return vec![];
    }

    let mut aacc = Vec::new();
    for opt in cur.iter().collect::<String>().split(" | ") {
        let mut acc = vec![0];
        for i in opt.split(' ').map(|n| n.parse::<u32>().unwrap()) {
            let mut nacc = Vec::new();
            for ac in acc {
                for c in part_two(&xs[ac as usize..], i, rlu) {
                    nacc.push(ac + c);
                }
            }
            acc = nacc;
        }
        aacc.extend_from_slice(&acc);
    }

    aacc
}

pub fn solve() {
    let inp = std::fs::read_to_string("../input/2020/19")
        .unwrap()
        .trim()
        .to_string();
    let (grammar, test) = inp
        .split_once("\n\n")
        .unwrap();

    let mut rlu = HashMap::new();

    for line in grammar.lines() {
        let (nr, rem) = line.split_once(": ").unwrap();
        rlu.insert(nr.parse::<u32>().unwrap(), rem.chars().collect::<Vec<_>>());
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for line in test.lines() {
        let chs = line.chars().collect::<Vec<_>>();
        if part_one(&chs, 0, &rlu) == line.len() as i32 { p1 += 1; }
    }

    rlu.insert(8, "42 | 42 8".chars().collect::<Vec<_>>());
    rlu.insert(11, "42 31 | 42 11 31".chars().collect::<Vec<_>>());

    for line in test.lines() {
        let chs = line.chars().collect::<Vec<_>>();
        if part_two(&chs, 0, &rlu).contains(&(line.len() as i32)) { p2 += 1; }
    }

    println!("{p1}");
    println!("{p2}");
}
