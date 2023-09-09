// Advent of Code 2018, day 8
// (c) aichingert

fn part1(tree: &[usize], cur: &mut usize) -> usize {
    let mut metadata = 0;
    let amount_of_metadata = tree[*cur + 1];

    for _ in 0..tree[*cur] {
        *cur += 2;
        metadata += part1(tree, cur);
    }

    for _ in 0..amount_of_metadata {
        metadata += tree[*cur + 2];
        *cur += 1;
    } 

    metadata
}

fn part2(tree: &[usize], cur: &mut usize) -> usize {
    let mut metadatas = Vec::<usize>::new();
    let mut metadata = 0usize;
    let amount_of_metadata = tree[*cur + 1];

    for _ in 0..tree[*cur] {
        *cur += 2;
        metadatas.push(part2(tree, cur));
    }

    for _ in 0..amount_of_metadata {
        metadata += if metadatas.len() > 0 {
            if tree[*cur + 2] - 1 < metadatas.len() {
                metadatas[tree[*cur + 2] - 1]
            } else { 0 }
        } else {
            tree[*cur + 2]
        };
        *cur += 1;
    } 

    metadata
}

fn main() {
    let inp = std::fs::read_to_string("../input/08").unwrap()
        .trim()
        .split(' ')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("Part 1: {}", part1(&inp, &mut 0));
    println!("Part 2: {}", part2(&inp, &mut 0));
}
