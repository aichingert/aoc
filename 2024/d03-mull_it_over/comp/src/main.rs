fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/03").unwrap();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut d = true;
    for line in inp.lines().filter(|s| !s.is_empty()) {
        let chs = line.chars().collect::<Vec<_>>();
        let mut i = 0;

        while i < chs.len() {
            match chs[i] {
                'd' => {
                    if i + 4 < chs.len() && &chs[i..i+4] == &['d', 'o', '(', ')'] {
                        d = true;
                        i += 4;
                        continue;
                    }

                    if i + 7 < chs.len() && &chs[i..i+7] == &['d','o','n','\'','t','(',')'] {
                        d = false;
                        i += 6;
                    }

                    i += 1;
                }
                'm' => {
                    if !d {
                        i += 1;
                        continue;
                    }

                    if i + 4 < chs.len() && &chs[i..i+4] != &['m','u','l', '('] {
                        i += 1;
                        continue;
                    }

                    i += 4;
                    let cp = i;

                    let mut j = i;
                    while j < chs.len() && chs[j] != ',' {
                        j += 1;
                    }

                    let l = chs[i..j].iter().copied().collect::<String>().parse::<i32>();

                    if j - i > 3 || l.is_err() {
                        continue;
                    }

                    i = j + 1;
                    j = i;

                    while j < chs.len() && chs[j] != ')' {
                        j += 1;
                    }

                    let r = chs[i..j].iter().copied().collect::<String>().parse::<i32>();

                    if j - i > 3 || r.is_err() {
                        i = cp;
                        continue;
                    }

                    p2 += l.unwrap() * r.unwrap();
                    i = j;
                }
                _ => i += 1,
            }
        }

        let l = line.split("mul").collect::<Vec<_>>();

        for m in &l {
            let n = m.split(',').collect::<Vec<_>>();

            println!("{n:?}");
            if n.len() < 2 {
                continue;
            }

            if n[0].len() <= 1 || &n[0][..1] != "(" {
                continue;
            }

            let mut idx = 0;
            while idx < n[1].len() && &n[1][idx..=idx] != ")" {
                idx += 1;
            }

            if n[0].len() > 4 || idx == 0 || idx > 4 {
                continue;
            }

            if idx >= n[1].len() || &n[1][idx..=idx] != ")" {
                continue;
            }
            println!("val: {:?}", &n);

            let l = n[0][1..].parse::<i128>();
            let r = n[1][..idx].parse::<i128>();

            if l.is_ok() && r.is_ok() {
                p1 += l.unwrap() * r.unwrap();
            }

        }
    }

    println!("{p1}");
    println!("{p2}");
}
