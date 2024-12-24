use std::collections::{HashMap, VecDeque};

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
    let (mut x, mut y) = (0, 0);

    for l in inp.lines() {
        let (name, val) = l.split_once(": ").unwrap();
        lk.insert(name, val.parse::<u64>().unwrap());

        if name.starts_with("x") {
            x |= lk.get(name).unwrap() << name[1..].parse::<u32>().unwrap();
        }
        if name.starts_with("y") {
            y |= lk.get(name).unwrap() << name[1..].parse::<u32>().unwrap();
        }
    }

    for l in gat.lines().filter(|l| !l.is_empty()) {
        let values = l.split(' ').collect::<Vec<_>>();

        gt.insert(values[4], (values[0], values[1], values[2]));
        if values[4].starts_with("z") {
            out.push(values[4]);
        }
    }

    let mut plk = lk.clone();

    for z in &out {
        let shl = z[1..].parse::<u64>().unwrap();
        p1 |= solve(z, &mut plk, &gt) << shl;
    }

    println!("p1: {p1}");

    println!("{:064b}", p1);
    println!("{:064b}", x + y);

    let mut ans = 0;

    while ans != x + y {
        let mut cl = lk.clone();

        ans = 0;
        for z in &out {
            let shl = z[1..].parse::<u64>().unwrap();
            ans |= solve(z, &mut cl, &gt) << shl;
        }

        let mut src = 65u32;

        for i in 0..64 {
            if ans & (1 << i) != (x + y) & (1 << i) {
                if src == 65 {
                    src = i;
                    break;
                }
            }
        }
        
        let mut srch = VecDeque::new();

        for z in &out {
            if z[1..].parse::<u32>().unwrap() == src {
                srch.push_front(*z);
            }
        }

        while let Some(term) = srch.pop_front() {
            let Some(wire) = gt.get(term) else {
                println!("INP: {term:?}");
                continue;
            };

            let looking = (x + y) & (1 << src);

            println!("[{term}]  := {wire:?}");
            println!("{looking} := {} {} {}", cl[wire.0], wire.1, cl[wire.2]);
        }

        //                      [z17]
        //   ("tfc"             "OR"            "qhq")
        //x17  AND y17                       qwg AND wvj
        //0    AND   0                         0 AND 1
        // [qwg]  :=                ("qwd",     "OR",       "swm")
        // 131072 :=                            0 OR 0
        // [qwd]  :=        ("x16", "AND", "y16")    ("wnf", "AND", "kbw")
        // 131072 := 0 AND 0
        // INP: "x16"
        // INP: "y16"
        // [wnf]  := ("y16", "XOR", "x16")
        // 131072 := 0 XOR 0
        // [kbw]  := ("kbj", "OR", "ttr")
        // 131072 := 0 OR 0
        // [kbj]  := ("y15", "AND", "x15")
        // 131072 := 0 AND 0
        // [ttr]  := ("qff", "AND", "dks")
        // 131072 := 0 AND 1
        // INP: "y15"
        // INP: "x15"
        // [qff]  := ("y15", "XOR", "x15")
        // 131072 := 0 XOR 0


        break;
    }

    
}
