// Advent of Code 2015, day 20
// (c) aichingert

fn part1(inp: u32) -> u32 {
    let mut ans = 1u32;

    loop {
        let mut cur = ans;

        for i in 1..=ans / 2 {
            if ans % i == 0 { cur += i; }
        }

        if cur * 10 >= inp {
            return ans;
        }

        ans += 1;
    }
}

fn part2() {}

fn main() {
    let inp = std::fs::read_to_string("../input/20").unwrap().trim().parse::<u32>().unwrap();

    println!("Part 1: {}", part1(inp));
    //println!("Part 2: {}", part2());
}
