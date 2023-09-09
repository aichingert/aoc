// Advent of Code 2017, day 10
// (c) aichingert

#[path="lib.rs"] mod lib;
use lib::SIZE;

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap();
    let lengths: Vec<usize> = inp
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    println!("Part 1: {}", lib::shuffling(&mut (0..SIZE as u32).collect::<Vec<u32>>(), &lengths, &mut 0, &mut 0));
    println!("Part 2: {}", lib::knot_hash(&inp.trim()));
}
