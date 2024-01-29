pub fn solve() {
    let inp = std::fs::read_to_string("../input/2015/01").unwrap().trim().to_string();
    let (mut part_one, mut part_two) = (0, 0);

    for (i, ch) in inp.chars().enumerate() {
        if ch == '(' { part_one += 1; }
        else         { part_one -= 1; }

        if part_one < 0 && part_two == 0 {
            part_two = i + 1;
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
