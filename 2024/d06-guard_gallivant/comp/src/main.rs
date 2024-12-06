use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/06").unwrap();

    let mut m = inp.lines().filter(|s| !s.is_empty()).map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut p2 = 0;
    let mut sp = (0, 0);
    let mut sd = ' ';

    'out: for i in 0..m.len() {
        for j in 0..m[i].len() {
            match m[i][j] {
                '^' => sd = '^',
                'v' => sd = 'v',
                '>' => sd = '>',
                '<' => sd = '<',
                _ => {},
            }

            if sd != ' ' {
                sp = (i, j);
                break 'out;
            }
        }
    }

    let mut v = HashSet::from([sp]);
    let mut p = sp;
    let mut d = sd;

    loop {
        match d {
            '^' => {
                if p.0 == 0 {
                    break;
                }
                if m[p.0 - 1][p.1] == '#' {
                    d = '>';
                } else {
                    p.0 -= 1;
                    v.insert(p);
                }
            }
            'v' => {
                if p.0 == m.len() - 1 {
                    break;
                }
                if m[p.0 + 1][p.1] == '#' {
                    d = '<';
                } else {
                    p.0 += 1;
                    v.insert(p);
                }
            }
            '>' => {
                if p.1 == m[p.0].len() - 1 {
                    break;
                }
                if m[p.0][p.1 + 1] == '#' {
                    d = 'v';
                } else {
                    p.1 += 1;
                    v.insert(p);
                }
            }
            '<' => {
                if p.1 == 0 {
                    break;
                }
                if m[p.0][p.1 - 1] == '#' {
                    d = '^';
                } else {
                    p.1 -= 1;
                    v.insert(p);
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{p:?}");

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if (sp.0 == i && sp.1 == j) || m[i][j] == '#' {
                continue;
            }

            m[i][j] = '#';

            let mut s = 0;
            let mut p = sp;
            let mut d = sd;

            loop {
                s += 1;
                match d {
                    '^' => {
                        if p.0 == 0 {
                            break;
                        }
                        if m[p.0 - 1][p.1] == '#' {
                            d = '>';
                        } else {
                            p.0 -= 1;
                        }
                    }
                    'v' => {
                        if p.0 == m.len() - 1 {
                            break;
                        }
                        if m[p.0 + 1][p.1] == '#' {
                            d = '<';
                        } else {
                            p.0 += 1;
                        }
                    }
                    '>' => {
                        if p.1 == m[p.0].len() - 1 {
                            break;
                        }
                        if m[p.0][p.1 + 1] == '#' {
                            d = 'v';
                        } else {
                            p.1 += 1;
                        }
                    }
                    '<' => {
                        if p.1 == 0 {
                            break;
                        }
                        if m[p.0][p.1 - 1] == '#' {
                            d = '^';
                        } else {
                            p.1 -= 1;
                        }
                    }
                    _ => unreachable!(),
                }

                if s == m.len() * m[0].len() {
                    break;
                }
            }

            if s == m.len() * m[0].len() {
                p2 += 1;
            }

            m[i][j] = ' ';
        }
    }

    println!("{}", v.len());
    println!("{p2}");

}
