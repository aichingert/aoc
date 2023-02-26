// Advent of Code 2022, day 3
// (c) aichingert

fn get_value(fst: &str, scn: &str, thr: &str) -> u32 {
    let c = fst.chars().nth(fst.find(|c| scn.contains(c) && thr.contains(c)).unwrap()).unwrap();

    if c.is_uppercase() { (c as u8 - ('A' as u8 - 27u8)) as u32 }
    else { (c as u8 - ('a' as u8 - 1u8)) as u32 }
}

fn part1(inp: &Vec<&str>) -> u32 {
    inp.iter().map(|s| {
        let half = s.len()/2;
        get_value(s, &s[..half], &s[half..])
    }).sum::<u32>()
}

fn part2(inp: &Vec<&str>) -> u32 {
    (0..inp.len()).step_by(3).map(|i| {
        get_value(&inp[i], &inp[i+1], &inp[i+2])
    }).sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/03").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
