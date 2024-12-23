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
    
    let mut s = Vec::new();

    let mut part_one = 0;
    let mut part_two = 0;

    let mut hm = HashMap::new();

    for line in lines.iter() {
        let mut secret = line.trim().parse::<i64>().unwrap();

        let mut ma = HashMap::new();
        let mut ch = [0; 4];

        let mut se = HashSet::new();

        for i in 0..2000 {
            let next = apply(secret);
            ch[0] = ch[1];
            ch[1] = ch[2];
            ch[2] = ch[3];
            ch[3] = (next % 10) - (secret % 10);

            if i > 2 && se.insert(ch) {
                hm.entry(ch).and_modify(|b| *b += next % 10).or_insert(next % 10);
                ma.insert(ch, next % 10);
            }
            secret = next;
        }

        s.push(ma);
        part_one += secret;
    }

    let mut l = [0; 4];
    let mut mi = 0;

    for (k, v) in hm {
        if v > mi {
            mi = v;
            l = k;
        }
    }

    println!("{mi} {l:?}");

    for i in 0..s.len() {
        println!("{i}");
        for k in s[i].keys() {
            let mut b = s[i][k];

            for j in 0..s.len() {
                if i == j { continue; }

                if let Some(&p) = s[j].get(k) {
                    b += p;
                }
            }

            part_two = part_two.max(b);
        }
    }

    println!("{}", part_two);

    //for i in 0..s.len() {
    //    for j in 3..s[i].len() {
    //        let mut b = s[i][j][0];

    //        for k in 0..s.len() {
    //            if i == k { continue; }

    //            for l in 3..s[k].len() {
    //                if s[i][j - 3][1] == s[k][l - 3][1]
    //                && s[i][j - 2][1] == s[k][l - 2][1]
    //                && s[i][j - 1][1] == s[k][l - 1][1]
    //                && s[i][j][1] == s[k][l][1] {
    //                    b += s[k][l][0];
    //                    break;
    //                }
    //            }
    //        }

    //        part_two = part_two.max(b);
    //    }
    //}

    println!("p1: {part_one}");
    println!("p2: {part_two}");
}
