// Advent of Code 2016, day 19
// (c) aichingert

fn part1(inp: u32) -> u32 {
    ((inp - (inp.next_power_of_two() >> 1)) * 2) % inp + 1
}

fn part2(inp: u32) -> u32 {
    let mut survivor: u32 = 1;
    while survivor * 3 < inp { survivor *= 3; }

    match inp < 2 * survivor {
        true => inp - survivor,
        false => 2*inp - 3*survivor
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/19").unwrap().trim().parse::<u32>().unwrap();

    println!("Part 1: {}", part1(inp));
    println!("Part 2: {}", part2(inp));
}
