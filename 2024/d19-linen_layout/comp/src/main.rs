use std::collections::{HashMap, HashSet};

fn val(
    cur: &[char],
    cnt: &mut HashMap<usize, usize>,
    set: &HashSet<Vec<char>>,
    max: usize,
) -> usize {
    if cur.is_empty() {
        return 1;
    }
    if let Some(&r) = cnt.get(&cur.len()) {
        return r;
    }

    let mut i = 1;
    let mut a = 0;

    while i <= max && i <= cur.len() {
        if set.contains(&cur[..i]) {
            a += val(&cur[i..], cnt, set, max);
        }

        i += 1;
    }

    cnt.insert(cur.len(), a);
    a
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/19").unwrap();

    let (c, f) = inp.split_once("\n\n").unwrap();
    let mut p = HashSet::new();
    let mut m = 0;

    for oc in c.split(", ").collect::<Vec<_>>() {
        m = m.max(oc.len());
        p.insert(oc.chars().collect::<Vec<_>>());
    }

    let f = f.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();

    let mut p1 = 0;
    let p2 = f
        .iter()
        .map(|c| c.chars().collect::<Vec<_>>())
        .map(|s| val(&s, &mut HashMap::new(), &p, m))
        .fold(0, |a, b| {
            if b > 0 {
                p1 += 1;
            }

            a + b
        });

    println!("p1: {p1}");
    println!("p2: {p2}");
}
