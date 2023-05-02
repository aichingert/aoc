// Advent of Code 2018, day 8
// (c) aichingert

fn part1(tree: &[usize], cur: &mut usize) -> usize {
    let mut metadata = 0;
    let mut am = tree[*cur + 1];

    for _ in 0..tree[*cur] {
        *cur += 2;
        metadata += part1(tree, cur);
    }

    for j in 0..am {
        metadata += tree[*cur + 2];
        *cur += 1;
    } 

    metadata
}

fn part2(tree: &[usize]) -> i32 {
    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/08").unwrap()
        .trim()
        .split(' ')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1: {}", part1(&inp, &mut 0));
    println!("Part 2: {}", part2(&inp));
}
