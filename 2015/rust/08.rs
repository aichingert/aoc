// Advent of Code 2015, day 8
// (c) aichingert

fn part1(inp: &str) -> u32 {
    inp.lines()
        .map(|l| {
            let ch = l.chars().collect::<Vec<char>>();
            let mut i: usize = 0;
            let mut c: u32 = 0;

            while i < ch.len()-1 {
                match ch[i] {
                    '\\' => match ch[i+1] {
                        '\\' | '"' => { i+=1; c+=1; },
                        'x' => { i+=3; c+=1 },
                        _ => panic!("invalid input"),
                    },
                    '"' => (),
                    _ => c+=1,
                };
                i += 1;
            }

            ch.len() as u32 - c
        }).sum::<u32>() 
}

fn part2(inp: &str) -> u32 {
    inp.lines()
        .map(|l| {
            let ch = l.chars().collect::<Vec<char>>();
            ch.iter()
                .map(|e| match e {
                    '"' | '\\' => 2,
                    _ => 1,
                }).sum::<u32>() + 2 - ch.len() as u32
        }).sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/08").unwrap();
    println!("Part 1: {:?}", part1(&inp));
    println!("Part 2: {:?}", part2(&inp));
}
