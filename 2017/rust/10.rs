// Advent of Code 2017, day 10
// (c) aichingert

const SIZE: usize = 256;

fn part1(lengths: &Vec<usize>) -> u32 {
    let mut list = (0..SIZE as u32).collect::<Vec<u32>>();
    let (mut skip_size, mut loc) = (0usize,0usize);

    for i in 0..lengths.len() {
        let mut col = (0..lengths[i]).map(|idx| list[(loc + idx) % SIZE]).collect::<Vec<u32>>();
        col = col.iter().rev().map(|n| *n).collect::<Vec<u32>>();

        for j in 0..lengths[i] {
            list[(loc + j) % SIZE] = col[j];
        }

        loc = (loc + lengths[i] + skip_size) % SIZE;
        skip_size += 1;
    }

    list[0] * list[1]
}
fn main() {
    let lengths: Vec<usize> = std::fs::read_to_string("../input/10").unwrap()
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    println!("Part 1: {}", part1(&lengths));
}
