// Advent of Code 2016, day 23
// (c) aichingert

#[path="asm.rs"] mod asm;
use asm::Runner;

fn part1(inp: &String) -> i128 {
    let mut runner = Runner::new(inp);
    *runner.reg.get_mut(&String::from("a")).unwrap() = 7;
    runner.exec(&String::from("a"))
}

fn part2() {}

fn main() {
    let inp = std::fs::read_to_string("../input/23").unwrap();

    println!("Part 1: {}", part1(&inp));
    //println!("Part 2: {}", part2());
}
