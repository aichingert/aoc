use std::collections::{HashMap, HashSet};

// A <-> B
// C <-> A
// B <-> C

//for i in &c {
//    for fjump in l.get(i).unwrap() {
//        for sjump in l.get(fjump).unwrap() {
//            if sjump == i {
//                continue;
//            }
//
//            if l.get(sjump).unwrap().contains(i) {
//                let mut int = [i, fjump, sjump];
//                int.sort();
//                pairs.insert(int);
//            }
//        }
//    }
//}

fn sol<'a>(computers: &'a HashSet<&'a str>) -> u32 {
    0
}


fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/23").unwrap();

    let mut c = HashSet::new();
    let mut l: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in inp.lines().filter(|l| !l.is_empty()) {
        let (lhs, rhs) = line.split_once("-").unwrap();
        c.insert(lhs);
        c.insert(rhs);

        l.entry(lhs).and_modify(|s| s.push(rhs)).or_insert(vec![rhs]);
        l.entry(rhs).and_modify(|s| s.push(lhs)).or_insert(vec![lhs]);
    }

    println!("{}", sol(&c));

    let mut pairs = HashSet::new();

    for i in &c {
        for fjump in l.get(i).unwrap() {
            for sjump in l.get(fjump).unwrap() {
                if sjump == i {
                    continue;
                }

                if l.get(sjump).unwrap().contains(i) {
                    let mut int = [i, fjump, sjump];
                    int.sort();
                    pairs.insert(int);
                }
            }
        }
    }

    let mut p1 = 0;

    'out: for pair in &pairs {
        for m in pair {
            if m.starts_with("t") {
                p1 += 1;
                continue 'out;
            }
        }
    }
    println!("p1: {}", p1);
}
