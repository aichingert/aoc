fn solve(inp: Vec<i32>) -> (u32, u32) {
    let mut part_one: u32 = 0;
    let mut part_two: u32 = 0;

    for i in 0..inp.len() - 3 {
        if inp[i+1] > inp[i] {
            part_one += 1;
        }

        let partial = inp[i+1] + inp[i+2];
        if inp[i] + partial < inp[i+3] + partial {
            part_two += 1;
        }
    }

    if inp[inp.len() - 3] > inp[inp.len() - 2] {
        part_one += 1;
    }
    if inp[inp.len() - 2] > inp[inp.len() - 1] {
        part_one += 1;
    }

    (part_one, part_two)
}

fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap().trim().to_string();
    let inp = inp.lines().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let (part_one, part_two) = solve(inp);
   
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
