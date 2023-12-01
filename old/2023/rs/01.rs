fn part_one(inp: &Vec<Vec<char>>) -> u32 {
    inp.iter()
        .map(|chs| {
            (*chs.iter().find(|c| c.is_digit(10)).unwrap() as u8 - b'0') as u32 * 10
                + (*chs.iter().rev().find(|c| c.is_digit(10)).unwrap() as u8 - b'0') as u32
        })
    .sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap().trim().to_string();
    let inp = inp.lines().map(|c| c.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
    //println!("Part two: {}", part_two(&inp));
}
