fn part_one(c: &Vec<i32>) -> i32 {
    let min = *c.iter().min().unwrap();
    let max = *c.iter().max().unwrap();

    let mut ans = i32::MAX;

    for i in min..=max {
        ans = ans.min(c.iter().map(|d| (d - i).abs()).sum::<i32>());
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/07").unwrap().trim().to_string();
    let inp = inp.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
}
