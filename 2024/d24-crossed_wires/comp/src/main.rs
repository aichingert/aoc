use std::collections::HashMap;

fn solve<'a>(
    cur: &'a str,
    lk: &mut HashMap<&'a str, u64>, 
    gt: &'a HashMap<&'a str, (&'a str, &'a str, &'a str)>
) -> u64 {
    if let Some(&res) = lk.get(cur) {
        return res;
    }

    let Some(gates) = gt.get(cur) else { panic!("Invalid wire"); };

    let lhs = solve(gates.0, lk, gt);
    let rhs = solve(gates.2, lk, gt);

    let res = match gates.1 {
        "OR" => lhs | rhs,
        "XOR" => lhs ^ rhs,
        "AND" => lhs & rhs,
        _ => panic!("invalid operation"),
    };

    lk.insert(cur, res);
    res
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/24").unwrap();
    let (inp, gat) = inp.split_once("\n\n").unwrap();

    let mut lk = HashMap::new();
    let mut gt = HashMap::new();
    let mut out = Vec::new();
    let mut p1 = 0;

    for l in inp.lines() {
        let (name, val) = l.split_once(": ").unwrap();
        lk.insert(name, val.parse::<u64>().unwrap());
    }

    for l in gat.lines().filter(|l| !l.is_empty()) {
        let values = l.split(' ').collect::<Vec<_>>();

        gt.insert(values[4], (values[0], values[1], values[2]));
        if values[4].starts_with("z") {
            out.push(values[4]);
        }
    }

    for z in out {
        let shl = z[1..].parse::<u64>().unwrap();
        p1 |= solve(z, &mut lk, &gt) << shl;
    }

    for (key, value) in lk {
        println!("{key}: {value}");
    }
    println!("p1: {p1}");
}
