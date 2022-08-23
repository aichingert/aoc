fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("error");

    solve_part_one(&inp);
    solve_part_two(&inp);
}

fn solve_part_one(inp: &String) {
    let b: Vec<usize> = inp.split('-').map( | i | i.parse::<usize>().unwrap()).collect();
    let mut n: i32 = 0;

    for i in b[0]..b[1] {
        let c: Vec<char> = i.to_string().chars().collect();
        let mut d: bool = false;

        for j in 0..c.len() -1 {
            if c[j] <= c[j + 1] {
                if c[j] == c[j + 1] {
                    d = true;
                }
            } else {
                d = false;
                break;
            }
        }

        if d {
            n += 1;
        }
    }

    println!("Solution part one: {}", n);
}

fn solve_part_two(inp: &String) {
    let b: Vec<usize> = inp.split('-').map( | i | i.parse::<usize>().unwrap()).collect();
    let mut n: i32 = 0;

    for i in b[0]..b[1] {
        let c: Vec<char> = i.to_string().chars().collect();
        let mut d: bool = false;

        for j in 0..c.len() -1 {
            if c[j] <= c[j + 1] {
                if c[j] == c[j + 1] {
                    d = true;
                }
            } else {
                d = false;
                break;
            }
        }

        let mut dd: bool = false;

        if d {
            let mut j: usize = 0;
            while j < c.len() {
                let mut l: usize = 1;

                for k in j + 1..c.len() {
                    if c[j] == c[k] {
                        l += 1;
                    } else {
                        break;
                    }
                }

                j += l;

                if l == 2 {
                    dd = true;
                    break;
                }
            }
    
            if dd {
                n += 1;
            }
        }
    }

    println!("Solution part one: {}", n);
}