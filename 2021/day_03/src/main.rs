fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("e&rror");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let c: Vec<&str> = input
        .lines()
        .collect();
    let mut v: Vec<(u32, u32)> = vec![(0, 0); c[0].len()];
    let mut idx: usize = 0;

    let mut e: Vec<&str> = Vec::new();
    let mut g: Vec<&str> = Vec::new();

    for sl in c {
        let s: String = sl.to_string();

        for i in 0..s.len() {
            if &s[i..=i] == "0" {
                v[idx].0 += 1;
            } else{
                v[idx].1 += 1;
            }

            idx += 1;
        }

        idx = 0;
    }

    for n in v.iter() {
        if n.0 > n.1 {
            g.insert(g.len(), "0");
            e.insert(e.len(), "1");
        } else {
            g.insert(g.len(), "1");
            e.insert(e.len(), "0");
        }
    }

    println!("Solution part one: {}", convert(&g) * convert(&e));
}

fn solve_part_two(input: &String) {
    let t: Vec<&str> = input
        .lines()
        .collect();
    let mut c: Vec<Vec<char>> = Vec::new();

    let ogx: Vec<char>;
    let co2: Vec<char>;
    let mut v: (u16, u16) = (0, 0);

    for i in 0..t.len() {
        c.push(t[i].chars().collect());
    }


    for i in 0..c[0].len() {
        for j in 0..c.len() {
            if c[j][i] == '0' {
                v.0 += 1;
            } else {
                v.1 += 1;
            }
        }

        let mut offset: usize = 0;

        if v.0 > v.1 {
            for j in 0..c.len() {
                if c[j - offset][i] == '1' && c.len() > 1 {
                    c.remove(j - offset);
                    offset += 1;
                }
            }
        } else{
            for j in 0..c.len() {
                if c[j - offset][i] == '0' && c.len() > 1 {
                    c.remove(j - offset);
                    offset += 1;
                }
            }
        }

        v = (0,0);
    }

    ogx = c[0].clone();

    for i in 0..t.len() {
        c.push(t[i].chars().collect());
    }

    for i in 0..c[0].len() {
        for j in 0..c.len() {
            if c[j][i] == '0' {
                v.0 += 1;
            } else {
                v.1 += 1;
            }
        }

        let mut offset: usize = 0;

        if v.0 > v.1 {
            for j in 0..c.len() {
                if c[j - offset][i] == '0' && c.len() > 1 {
                    c.remove(j - offset);
                    offset += 1;
                }
            }
        } else{
            for j in 0..c.len() {
                if c[j - offset][i] == '1' && c.len() > 1 {
                    c.remove(j - offset);
                    offset += 1;
                }
            }
        }

        v = (0,0);
    }

    co2 = c[0].clone();

    println!("Solution part two: {}", convert_char(&ogx) * convert_char(&co2));
}

fn convert(b: &Vec<&str>) -> u32 {
    let mut n: u32 = 0;

    for i in 0..b.len() {
        if b[b.len() - i - 1] == "1" {
            n += 2_i32.pow(i as u32) as u32;
        }
    }

    n
}

fn convert_char(b: &Vec<char>) -> u32 {
    let mut n: u32 = 0;

    for i in 0..b.len() {
        if b[b.len() - i - 1] == '1' {
            n += 2_i32.pow(i as u32) as u32;
        }
    }

    n
}