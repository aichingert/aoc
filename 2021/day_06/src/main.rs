fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
}

fn solve_part_one(inp: &String) {
    let mut a: [usize; 9] = p(inp);
}

fn p(inp: &String) -> [usize; 9] {
    let p: Vec<usize> = inp.split(',').map( | v | v.parse::<usize>().unwrap()).collect();
    let mut r: [usize; 9] = [0; 9];

    for i in 0..p.len() {
        r[p[i]] += 1;
    }

    r
}