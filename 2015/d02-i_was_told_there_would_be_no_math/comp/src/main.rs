fn main() {
    let inp = std::fs::read_to_string("../../../input/2015/02").unwrap();

    let p1 = inp
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| l.split('x').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .fold(0, |acc, mut cur| {
            cur.sort(); 
            acc + 2 * cur[0] * cur[1] + 2 * cur[1] * cur[2] + 2 * cur[0] * cur[2] + cur[0] * cur[1]
        });

    let p2 = inp
        .lines()
        .filter(|&l| !l.is_empty())
        .map(|l| l.split('x').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>())
        .fold(0, |acc, mut cur| {
            cur.sort(); 
            acc + cur[0] * 2 + cur[1] * 2 + cur[0] * cur[1] * cur[2]
        });

    println!("p1: {p1}");
    println!("p2: {p2}");
}
