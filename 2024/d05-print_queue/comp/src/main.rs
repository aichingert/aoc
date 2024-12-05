use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/05").unwrap();
    let (or, pp) = inp.split_once("\n\n").unwrap();
    let mut m = HashMap::<i32, Vec<i32>>::new();
    let mut p1 = 0;
    let mut p2 = 0;

    or.lines()
        .map(|s| {
            s.split('|')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|p| {
            m.entry(p[0])
                .and_modify(|v| v.push(p[1]))
                .or_insert(vec![p[1]]);
        });
    let pp = pp
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for u in &pp {
        let mut v = true;

        for i in 0..u.len() - 1 {
            if !m.contains_key(&u[i]) || !m[&u[i]].contains(&u[i + 1]) {
                v = false;
                break;
            }
        }

        if v {
            p1 += u[u.len() / 2];
        } else {
            let mut c = vec![(0, 0); u.len()];

            for i in 0..u.len() {
                c[i].1 = i;

                for j in 0..u.len() {
                    if i == j { continue; }

                    if !m.contains_key(&u[j]) {
                        continue;
                    }

                    if m[&u[j]].contains(&u[i]) {
                        c[i].0 += 1;
                    }
                }
            }

            c.sort_unstable();
            println!("{u:?}");
            println!("{c:?}");

            p2 += u[c[c.len() / 2].1];
        }
    }

    println!("{p1}");
    println!("{p2}");
}
