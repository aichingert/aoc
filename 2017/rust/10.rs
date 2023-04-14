// Advent of Code 2017, day 10
// (c) aichingert

#[path="./knot_hash.rs"] mod knot_hash;
use knot_hash::SIZE;

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap();
    let lengths: Vec<usize> = inp
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    println!("Part 1: {}", knot_hash::shuffling(&mut (0..SIZE as u32).collect::<Vec<u32>>(), &lengths, &mut 0, &mut 0));
    println!("Part 2: {}", knot_hash::knot_hash(&inp.trim()));
}
