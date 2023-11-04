fn solve(inp: &Vec<&str>) -> (i32, i32) {
    let (mut h, mut d) = (0, 0);
    let (mut ha, mut da, mut a) = (0, 0, 0);

    for i in 0..inp.len() {
        let values = inp[i].split(' ').collect::<Vec<_>>();

        match (values[0], values[1].parse::<i32>().unwrap()) {
            ("forward", n) => { h += n; ha += n; da += a * n; },
            ("down",    n) => { d += n; a += n; },
            ("up",      n) => { d -= n; a -= n; },
            _ => panic!(),
        }
    }


    (h * d, ha * da)
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap();
    let inp = inp.trim().lines().collect::<Vec<_>>();

    let (part_one, part_two) = solve(&inp);

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
