// Advent of Code 2017, day 2
// (c) aichingert

fn part1(inp: &Vec<Vec<u32>>) -> u32 {
    inp.iter().map(|l| l.iter().max().unwrap() - l.iter().min().unwrap()).sum::<u32>()
}

fn part2(inp: &Vec<Vec<u32>>) -> u32 {
    inp.iter()
        .map(|l| {
            for i in 0..l.len() {
                for j in 0..l.len() {
                    if i == j { continue; }
                    if l[i] % l[j] == 0 {
                        return l[i] / l[j];
                    } else if l[j] % l[i] == 0 {
                        return l[j] / l[i];
                    }
                }
            }
            0u32
        }).sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap();
    let inp = inp.lines().map(|l| l.split('\t').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
