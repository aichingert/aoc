fn main() {
    let inp = std::fs::read_to_string("../../../input/2015/01").unwrap();

    let p1 = inp
        .chars()
        .filter(|&c| c == '(' || c == ')')
        .fold(0, |acc, cur| acc + if cur == '(' { 1} else { -1 });
    let mut p2 = 0;
    for (i, c) in inp.chars().enumerate() {
        p2 += if c == '(' { 1 } else { -1 };
        if p2 < 0 {
            p2 = i as i32 + 1;
            break;
        }
    }

    println!("p1: {p1}");
    println!("p2: {p2}");
}
