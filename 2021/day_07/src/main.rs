fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
    solve_part_two(&inp);
}

fn solve_part_one(inp: &String) {
    let mut m: i32 = 0;
    let v: Vec<i32> = inp.split(',').map( | v |  { m = std::cmp::max(m, v.parse::<i32>().unwrap()); v.parse::<i32>().unwrap() }).collect();
    let mut f: i32 = 1000000000;

    for i in 0..m {
        let mut c: i32 = 0;

        for j in 0..v.len() {
            c += (i - v[j]).abs();
        }

        f = std::cmp::min(f, c);
    }

    println!("Solution part one: {}", f);
}

fn solve_part_two(inp: &String) {
    let mut m: i32 = 0;
    let v: Vec<i32> = inp.split(',').map( | v |  { m = std::cmp::max(m, v.parse::<i32>().unwrap()); v.parse::<i32>().unwrap() }).collect();
    let mut f: i32 = 1000000000;

    for i in 0..m {
        let mut c: i32 = 0;

        for j in 0..v.len() {
            let n = (i - v[j]).abs();
            c += (n * (n + 1)) / 2;
        }

        f = std::cmp::min(f, c);
    }

    println!("Solution part two: {}", f);
}