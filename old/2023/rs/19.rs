use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Op {
    S,
    B,
    N,
}

fn main() {
    let inp = std::fs::read_to_string("../input/19").unwrap().trim().to_string();
    let inp = inp.split("\n\n").map(|s| s).collect::<Vec<_>>();

    let mut rules = HashMap::new();

    for line in inp[0].lines() {
        let (name, r) = line.split_once('{').unwrap();
        let r = &r[..r.len() - 1];

        let dif = r.split(',').collect::<Vec<_>>();

        let mut rls = Vec::new();

        for i in 0..dif.len() {
            if let Some((cond, to)) = dif[i].split_once(":") {
                if let Some((id, n)) = cond.split_once("<") {
                    rls.push(((id.to_string(), Op::S, n.parse::<u32>().unwrap()), to.to_string()));
                } else {
                    let (id, n) = cond.split_once(">").unwrap();
                    rls.push(((id.to_string(), Op::B, n.parse::<u32>().unwrap()), to.to_string()));
                }
            } else {
                rls.push((("".to_string(), Op::N, 0), dif[i].to_string()));
            }
        }

        rules.insert(name.to_string(), rls);
    }

    let start = String::from("in");
    let mut ans = 0;

    for ratings in inp[1].lines() {
        let ratings = &ratings[1..ratings.len() - 1];
        let vals = ratings.split(',').map(|s| s.split_once("=").unwrap().1.parse::<u32>().unwrap()).collect::<Vec<_>>();

        if check(&vals, &rules, &start) {
            ans += vals.iter().sum::<u32>();
        }
    }

    println!("{ans}");
}

fn idx(s: &String) -> usize {
    match s.as_str() {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("invalid"),
    }
}

fn check(tile: &Vec<u32>, rules: &HashMap<String, Vec<((String, Op, u32), String)>>, current: &String) -> bool {
    if current.as_str() == "A" {
        return true;
    } else if current.as_str() == "R" {
        return false;
    }

    let rule = rules.get(current).unwrap();
    let chcks = rule;

    for c in chcks {
        let (ch, to) = c;

        match ch.1 {
            Op::S => {
                if tile[idx(&ch.0)] < ch.2 {
                    return check(tile, rules, to);
                }
            }
            Op::B => {
                if tile[idx(&ch.0)] > ch.2 {
                    return check(tile, rules, to);
                }
            }
            Op::N => {
                return check(tile, rules, to);
            }
        }
    }
    
    false
}
