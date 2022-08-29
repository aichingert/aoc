const S: usize = 1000;

fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
    solve_part_two(&inp);
}

fn solve_part_one(inp: &String) {
    let l: Vec<Vec<&str>> = inp.lines().map( | l | l.split(" -> ").collect()).collect();
    let mut m: Vec<Vec<i32>> = vec![vec![0; S]; S];
    
    for i in 0..l.len() {
        let mut v: Vec<&str> = l[i][0].split(',').collect();

        let f: (usize, usize) = (v[0].parse::<usize>().unwrap(), v[1].parse::<usize>().unwrap());

        v.clear();
        v = l[i][1].split(',').collect();

        let t: (usize, usize) = (v[0].parse::<usize>().unwrap(), v[1].parse::<usize>().unwrap());

        let ma: usize;
        let mi: usize;

        // 0,9 -> 5,9
        if f.0 == t.0 {
            ma = std::cmp::max(f.1, t.1);
            mi = std::cmp::min(f.1, t.1);

            for i in mi..=ma {
                m[i][f.0] += 1;
            }
        } else if f.1 == t.1 {
            ma = std::cmp::max(f.0, t.0);
            mi = std::cmp::min(f.0, t.0);

            for i in mi..=ma {
                m[f.1][i] += 1;
            }
        }
    }

    println!("Solution part one: {}", c(&m));
}

fn solve_part_two(inp: &String) {
    let l: Vec<Vec<&str>> = inp.lines().map( | l | l.split(" -> ").collect()).collect();
    let mut m: Vec<Vec<i32>> = vec![vec![0; S]; S];
    

    for i in 0..l.len() {
        let mut v: Vec<&str> = l[i][0].split(',').collect();

        let mut f: (usize, usize) = (v[0].parse::<usize>().unwrap(), v[1].parse::<usize>().unwrap());

        v.clear();
        v = l[i][1].split(',').collect();

        let mut t: (usize, usize) = (v[0].parse::<usize>().unwrap(), v[1].parse::<usize>().unwrap());

        let ma: usize;
        let mi: usize;

        // 1,1 -> 3,3
        // 9,7 -> 7,9
        if f.0 == t.0 {
            ma = std::cmp::max(f.1, t.1);
            mi = std::cmp::min(f.1, t.1);

            for i in mi..=ma {
                m[i][f.0] += 1;
            }
        } else if f.1 == t.1 {
            ma = std::cmp::max(f.0, t.0);
            mi = std::cmp::min(f.0, t.0);

            for i in mi..=ma {
                m[f.1][i] += 1;
            }
        } else {
            let x: i32 = f.0 as i32 - t.0 as i32;
            let y: i32 = f.1 as i32 - t.1 as i32;

            // -2 ; 2
            // -2 ; -2

            if (x > -1 && y > -1) || (x < 0 && y < 0) {
                if x < 0 && y < 0 {
                    let s = f;
                    f = t;
                    t = s;
                }

                for i in 0..=f.0-t.0 {
                    m[t.1 + i][t.0 + i] += 1;
                }
            } else {
                // 4,0 -> 0,4
                // 4 ; -4
                // -4 ; 4
                if x < 0 && y > -1 {
                    let s = f;
                    f = t;
                    t = s;
                }

                for i in 0..=f.0-t.0 {
                    m[t.1 - i][t.0 + i] += 1;
                }
            }
        }
    }

    println!("Solution part two: {}", c(&m));
}

fn c(m: &Vec<Vec<i32>>) -> i32 {
    let mut c: i32 = 0;

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if m[i][j] > 1 {
                c += 1;
            }
        }
    }
    
    c
}