use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let ins: Vec<&str> = input
        .lines()
        .collect();
    let mut m: Vec<(i32, i32)> = vec![];
    
    f(&mut ins[0].split(',').map( | s | s.to_string()).collect(), &mut m);

    println!("Solution part one: {}", s(&mut ins[1].split(',').map( | s | s.to_string()).collect(), &m));
}

fn solve_part_two(input: &String) {
    let ins: Vec<&str> = input
        .lines()
        .collect();
    let mut m: HashMap<(i32, i32), i32> = HashMap::new();

    fs(&mut ins[0].split(',').map( | s | s.to_string()).collect(), &mut m);

    println!("Solution part two: {:?}", ss(&mut ins[1].split(',').map( | s | s.to_string()).collect(), &m));
}

fn fs(ins: &mut Vec<String>, m: &mut HashMap<(i32, i32), i32>) {
    let mut p: (i32, i32) = (0, 0);
    let mut s: i32 = 0;

    for i in 0..ins.len() {
        let d: char = ins[i].remove(0);
        let v: i32 = ins[i].parse().unwrap();

        match d {
            'R' => {
                for _ in 0..v {
                    p.0 += 1;
                    s+=1;
                    if !m.contains_key(&(p.0, p.1)) {
                        m.insert((p.0, p.1), s);
                    }
                }
            },
            'L' => {
                for _ in 0..v {
                    p.0 -= 1;
                    s+=1;
                    if !m.contains_key(&(p.0, p.1)) {
                        m.insert((p.0, p.1), s);
                    }
                }
            },
            'U' => {
                for _ in 0..v {
                    p.1 += 1;
                    s+=1;
                    if !m.contains_key(&(p.0, p.1)) {
                        m.insert((p.0, p.1), s);
                    }
                }
            },
            'D' => {
                for _ in 0..v {
                    p.1 -= 1;
                    s+=1;
                    if !m.contains_key(&(p.0, p.1)) {
                        m.insert((p.0, p.1), s);
                    }
                }
            },
            _ => {
                println!("{}", d);
            }
        }
    }
}

fn ss(ins: &mut Vec<String>, m: &HashMap<(i32, i32), i32>) -> i32 {
    let mut p: (i32, i32) = (0, 0);
    let mut s: i32 = 0;
    let mut ms: i32 = 100000000;

    for i in 0..ins.len() {
        let d: char = ins[i].remove(0);
        let v: i32 = ins[i].parse().unwrap();

        match d {
            'R' => {
                for _ in 0..v {
                    p.0 += 1;
                    s+=1;
                    if m.contains_key(&(p.0, p.1)) {
                        let r: i32 = m[&(p.0, p.1)] + s;
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            'L' => {
                for _ in 0..v {
                    p.0 -= 1;
                    s+=1;
                    if m.contains_key(&(p.0, p.1)) {
                        let r: i32 = m[&(p.0, p.1)] + s;
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            'U' => {
                for _ in 0..v {
                    p.1 += 1;
                    s+=1;
                    if m.contains_key(&(p.0, p.1)) {
                        let r: i32 = m[&(p.0, p.1)] + s;
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            'D' => {
                for _ in 0..v {
                    p.1 -= 1;
                    s+=1;
                    if m.contains_key(&(p.0, p.1)) {
                        let r: i32 = m[&(p.0, p.1)] + s;
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            _ => {
                println!("{}", d);
            }
        }
    }

    ms
}

fn f(ins: &mut Vec<String>, m: &mut Vec<(i32, i32)>) {
    let mut p: (i32, i32) = (0, 0);

    for i in 0..ins.len() {
        let d: char = ins[i].remove(0);
        let v: i32 = ins[i].parse().unwrap();

        match d {
            'R' => {
                for _ in 0..v {
                    p.0 += 1;
                    if !m.contains(&(p.0, p.1)) {
                        m.push((p.0, p.1))
                    }
                }
            },
            'L' => {
                for _ in 0..v {
                    p.0 -= 1;
                    if !m.contains(&(p.0, p.1)) {
                        m.push((p.0, p.1))
                    }
                }
            },
            'U' => {
                for _ in 0..v {
                    p.1 += 1;
                    if !m.contains(&(p.0, p.1)) {
                        m.push((p.0, p.1))
                    }
                }
            },
            'D' => {
                for _ in 0..v {
                    p.1 -= 1;
                    if !m.contains(&(p.0, p.1)) {
                        m.push((p.0, p.1))
                    }
                }
            },
            _ => {
                println!("{}", d);
            }
        }
    }
}

fn s(ins: &mut Vec<String>, m: &Vec<(i32, i32)>) -> i32 {
    let mut p: (i32, i32) = (0, 0);
    let mut ms: i32 = 100000000;

    for i in 0..ins.len() {
        let d: char = ins[i].remove(0);
        let v: i32 = ins[i].parse().unwrap();

        match d {
            'R' => {
                for _ in 0..v {
                    p.0 += 1;
                    if m.contains(&(p.0, p.1)) {
                        let r: i32 = p.0.abs() + p.1.abs();
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            'L' => {
                for _ in 0..v {
                    p.0 -= 1;
                    if m.contains(&(p.0, p.1)) {
                        let r: i32 = p.0.abs() + p.1.abs();
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            'U' => {
                for _ in 0..v {
                    p.1 += 1;
                    if m.contains(&(p.0, p.1)) {
                        let r: i32 = p.0.abs() + p.1.abs();
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            'D' => {
                for _ in 0..v {
                    p.1 -= 1;
                    if m.contains(&(p.0, p.1)) {
                        let r: i32 = p.0.abs() + p.1.abs();
                        if ms > r {
                            ms = r;
                        }
                    }
                }
            },
            _ => {
                println!("{}", d);
            }
        }
    }

    ms
}