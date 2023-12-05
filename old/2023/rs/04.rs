fn part_one(inp: &Vec<Vec<Vec<u32>>>) -> u32 {
    inp.iter()
        .map(|v| v[1].iter().filter(|e| v[0].contains(e)).count())
        .filter(|&n| n != 0)
        .map(|n| 2u32.pow((n - 1) as u32))
        .sum::<u32>()
}

fn part_two(inp: &Vec<Vec<Vec<u32>>>) -> u32 {
    let mut ans = vec![1u32; inp.len()];

    for i in 0..inp.len() {
        for j in 1..=inp[i][1].iter().filter(|e| inp[i][0].contains(e)).count() {
            ans[i + j] += ans[i];
        }
    }

    ans.iter().sum::<u32>()
}

fn parse() -> Vec<Vec<Vec<u32>>> {
    std::fs::read_to_string("../input/04").unwrap().trim().to_string()
        .lines()
        .map(|l| 
            l.split_once(": ").unwrap().1.split(" | ")
                .map(|cur| cur.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let inp = parse();

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
