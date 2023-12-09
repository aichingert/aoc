fn calc(t: f64, d: f64) -> u32 {
    // p/2 +|- sqrt((p/2)^2 - q)
    // (p_h + sqrt) - (p_h - sqrt) ...p_h=p half, ...sqrt=(p/2)^2 - q
    // p_h - p_h + sqrt + sqrt

    let p_h = t / 2.;
    let sqrt = (p_h * p_h - d).sqrt();

    (2. * sqrt) as u32
}

fn part_one(inp: &String) -> u32 {
    let inp = inp.lines()
        .map(|l| l.split_whitespace().skip(1).map(|n| n.parse::<f64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{:?}", inp);
    for i in 0..inp[0].len() {
        if i == inp[0].len() - 1 {
            println!("{:?}", (inp[0][i], inp[1][i]));
        }
        println!("{}", calc(inp[0][i], inp[1][i]));
    }
    (0..inp[0].len()).map(|i| calc(inp[0][i], inp[1][i])).fold(1, |acc, nxt| acc * nxt)
}

fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap().trim().to_string();

    let num = inp.lines().map(|l| l.split_whitespace().skip(1).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut t = String::new();
    let mut d = String::new();

    for i in 0..num[0].len() {
        t.push_str(num[0][i]);
        d.push_str(num[1][i]);
    }

    let (t, d) = (t.parse::<f64>().unwrap(), d.parse::<f64>().unwrap());

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", calc(t, d));
}
