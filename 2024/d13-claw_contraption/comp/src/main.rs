use std::collections::{HashMap, HashSet};

fn sol(a: (i64, i64), b: (i64, i64), p: (i64, i64)) -> i64 {
    let xo = a.0 * b.1;
    let xu = a.1 * b.0;

    let po = p.0 * b.1;
    let pu = p.1 * b.0;

    let x_eq = xo - xu;
    let p_eq = po - pu;
    let x = p_eq / x_eq;

    if p_eq.rem_euclid(x_eq) != 0 
    || (p.0 - a.0 * x).rem_euclid(b.0) != 0  {
        return 0;
    }

    x * 3 + (p.0 - a.0 * x) / b.0
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/13").unwrap();

    let entries = inp.split("\n\n");
    let mut p1 = 0;
    let mut p2 = 0;

    for (i, entry) in entries.filter(|l| !l.trim().is_empty()).enumerate() {
        let l = entry.lines().collect::<Vec<_>>();
        let a = l[0].split(' ').collect::<Vec<_>>();
        let a = (a[2][2..a[2].len() - 1].parse::<i64>().unwrap(), a[3][2..].parse::<i64>().unwrap());
        let b = l[1].split(' ').collect::<Vec<_>>();
        let b = (b[2][2..b[2].len() - 1].parse::<i64>().unwrap(), b[3][2..].parse::<i64>().unwrap());

        let xy = l[2].split(' ').collect::<Vec<_>>();
        let prize = (xy[1][2..xy[1].len() - 1].parse::<i64>().unwrap(), xy[2][2..].parse::<i64>().unwrap());
        // a[0] * x + b[0] * y = p[0]
        // a[1] * x + b[1] * y = p[1]

        p1 += sol(a, b, prize);
        p2 += sol(a, b, (prize.0 + 10000000000000, prize.1 + 10000000000000));
    }

    println!("p1: {p1}");
    println!("p2: {p2}");
}
