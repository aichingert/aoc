fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input)
}

fn solve_part_one(input: &String) {
    let mut count_vali_tls: i32 = 0;

    for inp in input.lines() {
        if is_valid(inp) {
            count_vali_tls += 1;
        }
    }

    println!("Solution part one: {}", count_vali_tls);
}

fn solve_part_two(input: &String) {
    let mut count: i32 = 0;

    for inp in input.lines() {
        if valid_part_two(inp) {
            count += 1;
        }
    }

    println!("Solution part two: {}", count);
}

fn contains(hypernet: &String, aba: (u8, u8)) -> bool {
    let t = hypernet.as_bytes();
    for i in 0..t.len() - 2 {
        if t[i] == aba.1 && t[i + 1] == aba.0 && t[i + 2] == aba.1 {
            return true;
        }
    }
    false
}

fn valid_part_two(input: &str) -> bool {
    let t = split(input);
    let supernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 0 { Some(&x.0) } else { None }).collect();
    let hypernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 1 { Some(&x.0) } else { None }).collect();
    let abas = parse_aba(supernets);
    for aba in abas {
        for hypernet in hypernets.iter() {
            if contains(hypernet, aba) {
                return true;
            }
        }
    }
    false
}

fn parse_aba(inputs: Vec<&String>) -> Vec<(u8, u8)> {
    let mut values = Vec::new();
    for input in inputs.iter() {
        let t = input.as_bytes();
        for i in 0..t.len() - 2 {
            if t[i] == t[i + 2] && t[i] != t[i + 1] {
                values.push((t[i], t[i + 1]));
            }
        }
    }
    values
}

fn is_valid(input: &str) -> bool {
    let t = split(input);
    let supernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 0 { Some(&x.0) } else { None }).collect();
    let hypernets: Vec<&String> =
        t.iter().filter_map(|x| if x.1 == 1 { Some(&x.0) } else { None }).collect();

    if hypernets.iter().any(|x| is_abba(*x)) {
        return false;
    }
    if supernets.iter().any(|x| is_abba(*x)) {
        return true;
    }
    false
}

fn is_abba(input: &String) -> bool {
    let t = input.as_bytes();
    for i in 0..t.len() - 3 {
        if t[i] == t[i + 3] && t[i + 1] == t[i + 2] && t[i] != t[i + 1] {
            return true;
        }
    }
    false
}

fn split(input: &str) -> Vec<(String, usize)> {
    let mut r = vec![];
    let mut buf = String::new();
    let mut depth = 0;
    for c in input.chars() {
        if c == '[' {
            if buf != "" {
                r.push((buf, depth));
                buf = String::new();
            }
            depth += 1;
        } else if c == ']' {
            if buf != "" {
                r.push((buf, depth));
                buf = String::new();
            }
            depth -= 1;
        } else {
            buf.push(c);
        }
    }
    if buf != "" {
        r.push((buf, depth));
    }
    r
}