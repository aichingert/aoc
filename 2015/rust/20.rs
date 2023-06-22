// Advent of Code 2015, day 20
// (c) aichingert

fn fact(n: u32) -> Vec<u32> {
    let mut fac = vec![1, n];

    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            fac.push(i);
            if i * i != n {
                fac.push(n / i);
            }
        }
    }

    return fac;
}

fn part1(inp: u32) -> u32 {
    let mut ans = 2;

    while 10 * fact(ans).iter().sum::<u32>() <= inp {
        ans += 1;
    }

    ans
}

fn part2(inp: u32) -> u32 {
    let mut ans = 2;

    while 11
        * fact(ans).iter().fold(0, |sum, cur| {
            sum + match ans / cur {
                0..=50 => *cur,
                _ => 0,
            }
        })
        <= inp
    {
        ans += 1;
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/20")
        .unwrap()
        .trim()
        .parse::<u32>()
        .unwrap();
    println!("{:?}", fact(20));

    println!("Part 1: {}", part1(inp));
    println!("Part 2: {}", part2(inp));
}
