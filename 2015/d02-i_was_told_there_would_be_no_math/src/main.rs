fn main() {
    let inp = std::fs::read_to_string("../../../input/2015/01").unwrap();

    let p1 = inp
        .lines()
        .filter(|&l| l.is_empty() || l.is_blank())
        .map(|l| l.split('x').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .for_each(|r| println!("{:?}", r));
}
