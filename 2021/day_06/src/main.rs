fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
    solve_part_two(&inp);
}

fn solve_part_one(inp: &String) {
    let mut a: [usize; 9] = p(inp);

    for _ in 0..80 {
        let z: usize = a[0];
        for i in 1..a.len() {
            a[i - 1] = a[i];
        }
        a[6] += z;
        a[8] = z;
    }

    println!("Solution part one: {}", s(&a));
}

fn solve_part_two(inp: &String) {
    let mut a: [usize; 9] = p(inp);

    for _ in 0..256 {
        let z: usize = a[0];
        for i in 1..a.len() {
            a[i - 1] = a[i];
        }
        a[6] += z;
        a[8] = z;
    }

    println!("Solution part two: {}", s(&a));
}

fn p(inp: &String) -> [usize; 9] {
    let p: Vec<usize> = inp.split(',').map( | v | v.parse::<usize>().unwrap()).collect();
    let mut r: [usize; 9] = [0; 9];

    for i in 0..p.len() {
        r[p[i]] += 1;
    }

    r
}

fn s(a: &[usize; 9]) -> usize {
    let mut s: usize = 0;

    for i in 0..a.len() {
        s += a[i];
    }

    s
}