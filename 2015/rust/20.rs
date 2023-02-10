// Advent of Code 2015, day 20
// (c) aichingert

fn fact(n: u32) -> u32 {
    let mut fac = n+1;
 
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            fac += i;
            if i*i != n { fac += n / i; }
        }
    }

    return fac
}

fn part1(inp: u32) -> u32 {
    let mut ans = 2;

    while fact(ans) <= inp { ans += 1; }

    ans
}

fn part2(inp: u32) ->  u32 {
    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/20").unwrap().trim().parse::<u32>().unwrap();

    println!("Part 1: {}", part1(inp));
    //println!("Part 2: {}", part2());
}
