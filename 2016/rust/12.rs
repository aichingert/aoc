// Advent of Code 2016, day 12
// (c) aichingert

#[path="asm.rs"] mod asm;
use asm::Runner;

fn part1(inp: &String) -> i128 {
    let mut runner = Runner::new(inp);
    runner.exec(&String::from("a"))
}

fn part2(inp: &String) -> i128 {
    let mut runner = Runner::new(inp);
    *runner.reg.get_mut(&String::from("c")).unwrap() = 1;
    runner.exec(&String::from("a"))
}

fn main() {
    let inp = std::fs::read_to_string("../input/12").unwrap();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
