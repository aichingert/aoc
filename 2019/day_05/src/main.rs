fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
}

fn solve_part_one(inp: &String) {
    let mut i: Vec<i32> = inp
        .split(',')
        .map( | c | c.parse::<i32>().unwrap())
        .collect();

    let mut idx: usize = 0;
    let mut s: i32 = 0;

    loop {
        let inst: usize = i[idx] as usize;
        let mut j: usize = 0;

        if inst == 99 {
            break;
        }

        match inst {
            1 => {
                let f: usize = i[idx + 1] as usize;
                let s: usize = i[idx + 2] as usize;
                let t: usize = i[idx + 3] as usize;

                i[t] = i[f] + i[s];
                j = 4;
            },
            2 => {
                let f: usize = i[idx + 1] as usize;
                let s: usize = i[idx + 2] as usize;
                let t: usize = i[idx + 3] as usize;

                i[t] = i[f] * i[s];
                j = 4;
            },
            3 => {
                let f: usize = i[idx + 1] as usize;
                i[f] = 1;
                j = 2;
            },
            4 => {
                let f: usize = i[idx + 1] as usize;
                s = i[f];
                j = 2;
            }
            _ => {
                let v: String = inst.to_string();
                let c: usize = v[v.len()-2..v.len()].parse::<usize>().unwrap();
                let mut h: usize = 0;
                let mut t: usize = 0;

                if v.len() > 2 {
                    h = v[v.len()-3..=v.len() - 3].parse::<usize>().unwrap();
                }

                if v.len() > 3 {
                    t = v[v.len()-4..=v.len() - 4].parse::<usize>().unwrap();
                }


                let mut d: bool = false;

                match c {
                    3 => {
                        i[idx + 1] = 1;
                        j = 2;
                        d = true;
                    },
                    4 => {
                        s = i[idx + 1];
                        j = 2;
                        d = true;
                    },
                    _ => {}
                }

                if !d {

                    let f: i32;
                    let s: i32;
                    let th: i32;
    
                    if h == 1 {
                        f = i[idx + 1];
                    } else {
                        f = i[i[idx + 1] as usize];
                    }
    
                    if t == 1 {
                        s = i[idx + 2];
                    } else {
                        s = i[i[idx + 2] as usize];
                    }
    
                    th = i[idx + 3];
    
                    match c {
                        99 => {
                            break;
                        },
                        1 => {
                            i[th as usize] = f + s;
                        },
                        2 => {
                            i[th as usize] = f * s;
                        }
                        _ => {
                            println!("oh no, {}", c)
                        }
                    }
    
                    j = 4;
                }
            }
        }

        idx += j;
    }

    println!("Solution part one: {}", s);
}
