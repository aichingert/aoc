use std::collections::{HashMap, HashSet};

#[derive(Clone)]
enum Link {
    Value(char),
    Order(Vec<i32>),
    Optional(Vec<Vec<i32>>),
}

fn m_concat(vec: &Vec<Vec<String>>) -> Vec<String> {
    println!("{}", vec.len());
    h_concat(0, vec).unwrap_or(vec![])
}

fn h_concat(cur: usize, vec: &Vec<Vec<String>>) -> Option<Vec<String>> {
    if cur >= vec.len() {
        return None;
    }

    let mut list = Vec::new();

    for s in vec[cur].iter() {
        if let Some(extend) = h_concat(cur + 1, vec) {
            for inner in extend.into_iter() {
                list.push(format!("{s}{inner}"));
            }
        } else {
            list.push(s.clone());
        }
    }

    Some(list)
}

fn solve(d: i32, id: &i32, rules: &HashMap<i32, Link>, cur: &String) -> Vec<String> {
    let d = d + 1;

    match &rules[id] {
        Link::Value(c) => vec![format!("{cur}{c}")],
        Link::Order(o) => {
            m_concat(&o.iter().map(|i| solve(d, i, rules, cur)).collect::<Vec<_>>())
        }
        Link::Optional(opt) => {
            let res = opt.iter()
                .map(|o| o.iter().map(|i| solve(d, i, rules, cur)).collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let mut vec = Vec::new();
            for i in 0..res.len() {
                for e in m_concat(&res[i]) {
                    vec.push(e);
                }
            }

            vec
        }
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/19").unwrap().trim().to_string();
    let mut rules = HashMap::new();
    let mut good = HashSet::new();

    let (fs, sc) = inp.split_once("\n\n").unwrap();

    for line in fs.lines() {
        let (id, rest) = line.split_once(": ").unwrap();

        rules.insert(id.parse::<i32>().unwrap(), if let Some((lhs, rhs)) = rest.split_once(" | ") {
            Link::Optional(
                vec![
                    lhs.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>(),
                    rhs.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>(),
                ]
            )
        } else {
            if rest.starts_with("\"") {
                Link::Value(rest.chars().skip(1).next().unwrap())
            } else {
                Link::Order(rest.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>())
            }
        });
    }

    sc.lines().for_each(|l| { good.insert(l.to_string()); });

    let mut ans = 0;

    //rules.insert(8, Link::Optional(vec![vec![42], vec![42, 8]]));
    //rules.insert(11, Link::Optional(vec![vec![42, 31], vec![42, 11, 31]]));

    let mut m = 0;
    let mut mi = 0;

    for s in solve(0, &0, &rules, &String::new()) {
        m = m.max(s.len());
        mi = mi.min(s.len());

        if good.contains(&s) {
            ans += 1;
        }
    }

    println!("LEN: {}", solve(0, &0, &rules, &String::new()).len());
    println!("MAX: {} : MIN {}", m, mi);
    println!("Part one: {ans}");

}
