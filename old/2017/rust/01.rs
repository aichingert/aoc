// Advent of Code 2017, day 1
// (c) aichingert

fn part1(inp: &[char]) -> u32 {
    let ans = if inp[0] == inp[inp.len()-1] { ((inp[0] as u8) - ('0' as u8)) as u32 } else { 0 };
    (0..inp.len()-1).map(|i| if inp[i] == inp[i+1] { ((inp[i] as u8) - ('0' as u8)) as u32 } else { 0 } ).sum::<u32>() + ans
}

fn part2(inp: &[char]) -> u32 {
    let half: usize = inp.len()/2;
    (0..inp.len()).map(|i| if inp[i] == inp[(half+i)%inp.len()] { ((inp[i] as u8) - ('0' as u8)) as u32 } else { 0 } ).sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap();
    let inp = inp.trim().chars().collect::<Vec<char>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
