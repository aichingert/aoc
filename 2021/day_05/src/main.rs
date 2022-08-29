fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
}

fn solve_part_one(inp: &String) {
    let l: Vec<Vec<&str>> = inp.lines().map( | l | l.split(" -> ").collect()).collect();
    let mut m: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    

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