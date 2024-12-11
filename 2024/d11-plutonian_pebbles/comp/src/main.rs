use std::collections::{HashMap, HashSet};
use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/11").unwrap();

    let mut p1 = 0;

    let mut ss = inp
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|n| n.trim().parse::<BigUint>().unwrap())
        .collect::<Vec<_>>();

    let mul = BigUint::new(vec![2024]);
    let mut h: HashMap<BigUint, usize> = HashMap::new();

    for s in ss {
        h.entry(s).and_modify(|n| *n += 1).or_insert(1);
    }

    for i in 0..75 {
        if i == 25 {
            for &v in h.values() {
                p1 += v;
            }
        }

        let mut n = HashMap::new();

        for (k, v) in h {
            let s = k.to_string();

            if s.len() & 1 == 0 {
                let f = s[..s.len() / 2].parse::<BigUint>().unwrap();
                let s = s[s.len() / 2..].parse::<BigUint>().unwrap();

                n.entry(f).and_modify(|n| *n += v).or_insert(v);
                n.entry(s).and_modify(|n| *n += v).or_insert(v);
            } else if k == BigUint::ZERO  {
                n.entry(BigUint::one()).and_modify(|n| *n += v)
                    .or_insert(v);
            } else {
                n.entry(k * mul.clone()).and_modify(|n| *n += v).or_insert(v);
            }
        }

        h = n;
    }
    
    let mut a = 0;

    for &v in h.values() {
        a += v;
    }


    println!("p1: {}", p1);
    println!("p2: {}", a);
}
