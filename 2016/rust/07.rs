// Advent of Code 2016, day 7
// (c) aichingert

fn has_abba(cur: &Vec<char>) -> bool {
    for i in 0..cur.len()-3 {
        if cur[i] != cur[i+1] && cur[i] == cur[i+3] && cur[i+1] == cur[i+2] { return true; }
    }
    false
}

fn part1(inp: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
    let mut ans = 0;

    'outer: for l in inp {
        let mut v = false;

        for hypr in l.1.iter() {
            if has_abba(&hypr.chars().collect::<Vec<char>>()) { continue 'outer; }
        }

        for addr in l.0.iter() {
            if has_abba(&addr.chars().collect::<Vec<char>>()) {
                v = true;
                break;
            }
        }

        if v { ans += 1; }
    }

    ans
}

fn has_ssl(addr: &Vec<String>, hypr: &Vec<String>) -> bool {
    for i in 0..addr.len() {
        let c = addr[i].chars().collect::<Vec<char>>();

        for j in 0..c.len()-2 {
            if c[j] != c[j+1] && c[j] == c[j+2] {
                for hyp in hypr.iter() {
                    let ch = hyp.chars().collect::<Vec<char>>();

                    for k in 0..ch.len()-2 {
                        if ch[k] == ch[k+2] && ch[k] == c[j+1] && ch[k+1] == c[j] {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn part2(inp: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    inp.iter().filter(|l| has_ssl(&l.0, &l.1)).count()
}

fn parse() -> Vec<(Vec<String>, Vec<String>)> {
    let mut ips = Vec::new();
    let inp = std::fs::read_to_string("../input/07").unwrap();

    for l in inp.lines() {
        let (lhs, rest): (&str, &str) = l.split_once('[').unwrap();
        let loc = ips.len();
        ips.push((vec![lhs.to_string()], vec![]));

        for split in rest.split('[') {
            let (hypr, addr): (&str, &str) = split.split_once(']').unwrap();
            ips[loc].0.push(addr.to_string());
            ips[loc].1.push(hypr.to_string());
        }

    }

    ips
}

fn main() {
    let inp = parse();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
